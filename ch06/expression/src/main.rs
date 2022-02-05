fn main() {
    // Rustでは式(expression)と文(statement)が厳密に区別されている
    // 式には値があり文にはない
    let v = if true { 10 } else { 20 };
    println!(
        "{} {}",
        v,
        match Some(10) {
            Some(num) => num * 10,
            None => 0,
        }
    );
    fn f() -> Option<i32> {
        Some(20)
    }
    fn g() -> Option<i32> {
        None
    }
    let ok = true;
    let arr = vec![1, 2, 3];
    struct Point {
        x: i32,
        y: i32,
    }

    let _expressions = (
        [1, 2, 3],
        [0; 50],
        (6, "hello"),
        (2 + 2),
        {
            f();
            g()
        },
        if ok {
            f();
        },
        if ok { f() } else { g() },
        if let Some(x) = f() { x } else { 0 },
        match ok {
            true => 1,
            false => 0,
        },
        for _el in arr {
            f();
        },
        println!("ok"),
        std::f64::consts::PI,
        Point { x: 1, y: 2 },
        f(),
        1..10,
        1..=10,
    );

    // 演算子はすべて左結合
    // a - b - c は (a - b) - c と同じ

    // ブロックは式
    // セミコロンがない行は式となり値を生む

    // 宣言
    // let name: type = expr;

    // このような初期化の場合はmutとして宣言する必要がない
    let v;
    if true {
        v = Some(10);
    } else {
        v = None;
    }
    println!("{:?}", v);

    // Rustはシャドーイングができる
    fn _f() -> Result<i32, ()> {
        for line in [Ok(10), Err(())] {
            let line = line?;
            println!("{}", line);
        }
        Ok(10)
    }

    // ブロック内でアイテムの宣言ができる
    // アイテムの宣言: fn, struct, use

    // fnで外の値を取る事はできず、明確にクロージャとして宣言する
    fn _ff() {
        // println!("{:?}", v);// can't capture dynamic environment in a fn item use the `|| { ... }` closure form instead
    }

    // if
    if true {
        println!("true");
    } else if false {
        println!("else if");
    } else {
        println!("else");
    }

    // match
    match 4 {
        0 => println!("zero"),
        1 => println!("one"),
        _ => println!("other"),
    }
    // matchをジャンプテーブルにコンパイルして最適化する
    // match value {
    //     pattern => expr,
    //     ...
    // }
    // この類のmatchは上から順番にパターンを適用する

    // if let式
    // if let pattern = expr {
    //   block1
    // } else {
    //   block2
    // }

    // ループ
    // 4つのループ式, while, while let, loop, for

    let _range1 = 1..10;
    let _range2 = std::ops::Range { start: 1, end: 10 };

    for el in [1, 2, 3] {
        println!("{}", el);
    }
    let mut arr = [1, 2, 3];
    for el in &arr {
        println!("{}", el);
    }
    for el in &mut arr {
        *el += 1;
    }

    // loopでbreakで値を返すことができる
    // 'seach: for みたいにラベルを付けて break 'search; で抜けられる

    let _v = Vec::<i32>::with_capacity(100);

    let _ranges = ([..], [1..], [..10], [1..10], [..=10], [1..=10]);

    // Rustに++, --はない

    // 自動で行われるキャストがあり、&String -> &str, &Vec<i32> -> &[i32], &Box<Chessboard> -> &Chessboard
    // これは参照解決型変換(Deref型変換: deref coercion)と呼ばれる。組み込みトレイトのDerefを実装している。

    // クロージャ
    let _is_even = |x: i32| x % 2 == 0;
}
