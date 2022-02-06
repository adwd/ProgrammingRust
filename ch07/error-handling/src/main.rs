use std::{error::Error, io::BufRead};

fn main() {
    // パニック
    // パニックが起きるのはプログラマの過ちがあった場合
    // Rustはパニックが起きるとスタックを巻き戻すか、プロセスをアボートする

    fn pirate_share(total: u64, crew_size: usize) -> u64 {
        let half = total / 2;
        half / crew_size as u64
    }

    pirate_share(100, 0);
    /*
        ❯ RUST_BACKTRACE=1 cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/error-handling`
    thread 'main' panicked at 'attempt to divide by zero', src/main.rs:8:9
    stack backtrace:
       0: rust_begin_unwind
                 at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:498:5
       1: core::panicking::panic_fmt
                 at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:107:14
       2: core::panicking::panic
                 at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:48:5
       3: error_handling::main::pirate_share
                 at ./src/main.rs:8:9
       4: error_handling::main
                 at ./src/main.rs:11:5
       5: core::ops::function::FnOnce::call_once
                 at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/ops/function.rs:227:5
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
     */

    // パニックはスレッド単位で発生する
    // スタックの巻き戻しをキャッチしてスレッドを殺さず実行を続けることもできる。
    // std::panic::catch_unwind()

    // Result
    let result: Result<i32, &str> = Ok(1);
    let _result_methods = (
        result.is_ok(),
        result.ok(),
        result.err(),
        result.unwrap_or(0),
        result.unwrap_or_else(|e| if e.contains("ok") { 1 } else { 0 }),
        result.unwrap(),
        result.expect("error"),
        // ここまでのメソッドはResultを消費してしまうのでそれで困る場合次の2つが使える
        result.as_ref(),
        // result.as_mut(),
        result.as_ref().is_ok(),
    );

    let err = std::io::Error::new(std::io::ErrorKind::Other, "error");
    let _error_methods = (err.to_string(), err.source());

    use std::error::Error;
    use std::io::{stderr, Write};
    fn print_error(mut err: &dyn Error) {
        let _ = writeln!(stderr(), "error: {}", err);
        while let Some(source) = err.source() {
            let _ = writeln!(stderr(), "caused by: {}", err);
            err = source;
        }
    }

    type GenericError = Box<dyn Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T, GenericError>;

    fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
        let mut numbers = Vec::new();
        for line in file.lines() {
            let line = line?;
            numbers.push(line.parse()?);
        }
        Ok(numbers)
    }

    // downcast_refメソッドで特定のエラー型に変換できる
    let err = GenericError::from("error");
    match err.downcast_ref::<std::num::ParseIntError>() {
        Some(err) => println!("{}", err),
        None => println!("not a ParseIntError"),
    }

    let res: Result<String, std::io::Error> = Ok("ok".to_string());
    let message = match res {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        }
    };
    println!("{}", message);

    #[derive(Debug, Clone)]
    struct JsonError {
        message: String,
        line: usize,
        column: usize,
    }
    impl std::fmt::Display for JsonError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{} ({}: {})", self.message, self.line, self.column)
        }
    }
    impl std::error::Error for JsonError {}

    use thiserror::Error;
    #[derive(Error, Debug)]
    #[error("{message:} ({line:}, {column:})")]
    struct JsonError2 {
        message: String,
        line: usize,
        column: usize,
    }
    let err = JsonError {
        message: "error".to_string(),
        line: 1,
        column: 2,
    };
}
