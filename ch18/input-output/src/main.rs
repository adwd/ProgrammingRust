fn main() {
    // Rustの入出力はRead, BufRead, Writeを中心に構築されている
    // Read: バイト単位の入力メソッドを持ち、readerと呼ばれる
    // BufRead: バッファ付きreaderと呼ぶ
    // Write: バイト単位の出力とUTF-8テキストの出力をサポートし、writerと呼ばれる

    // reader, writer
    {
        // reader
        // std::fs::File::open(filename)
        // std::net::TcpStream
        // std::io::stdin()
        // std::io::Cursor<&[u8]>, std::io::Cursor<Vec<u8>>

        // writer
        // std::fs::File::create(filename)
        // std::net::TcpStream
        // std::io::stdout(), std::io::stderr()
        // Vec<u8>
        // std::io::Cursor<Vec<u8>>
        // std::io::Cursor<&mut [u8]>

        // reader: std::io::Read
        // writer: std::io::Write

        use std::io::{self, ErrorKind, Read, Write};
        const DEFAULT_BUF_SIZE: usize = 8 * 1024;

        fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
        where
            R: Read,
            W: Write,
        {
            let mut buf = [0; DEFAULT_BUF_SIZE];
            let mut written = 0;
            loop {
                let len = match reader.read(&mut buf) {
                    Ok(0) => return Ok(written),
                    Ok(len) => len,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };

                writer.write_all(&buf[..len])?;
                written += len as u64;
            }
        }

        // reader
        // reader.read(&mut bufrer)
        // データ源からバイト列を読み出し、引数bufferに格納する
        // データ源にもっとデータが有ったとしてもbuffer.len()以下のバイトだけ読み出す
        // 低レベルなところを見てるのでデータ読み出しにはもっと高水準なメソッドを使うのが良い
        // reader.read_to_end(&mut byte_vec)
        // read_to_string, read_exact
        // bytes, chain, take
        // readerをクローズする必要はない。たいていreader, writerはDropを実装していて自動的にクローズする

        // バッファ付きreader
        // システムコールが遅いのでreadの度にコールするよりもバッファを持つほうが早い

        // 行の読み出し
        use std::io::prelude::*;
        fn grep(target: &str) -> io::Result<()> {
            let stdin = io::stdin();
            for line_result in stdin.lock().lines() {
                let line = line_result?;
                if line.contains(target) {
                    println!("{line}");
                }
            }
            Ok(())
        }

        fn grep2<R>(target: &str, reader: R) -> io::Result<()>
        where
            R: io::BufRead,
        {
            for line_result in reader.lines() {
                let line = line_result?;
                if line.contains(target) {
                    println!("{line}");
                }
            }
            Ok(())
        }

        use std::error::Error;
        use std::fs::File;
        use std::io::{BufReader, BufWriter};
        use std::path::PathBuf;

        fn grep3<R>(target: &str, reader: R) -> io::Result<()>
        where
            R: io::BufRead,
        {
            for line_result in reader.lines() {
                let line = line_result?;
                if line.contains(target) {
                    println!("{}", line);
                }
            }
            Ok(())
        }

        fn grep_main() -> Result<(), Box<dyn Error>> {
            let mut args = std::env::args().skip(1);
            let target = match args.next() {
                Some(s) => s,
                None => return Err("usage: grep PATTERN FILE...".into()),
            };
            let files = args.map(PathBuf::from).collect::<Vec<PathBuf>>();

            if files.is_empty() {
                let stdin = io::stdin();
                grep3(&target, stdin.lock())?;
            } else {
                for file in files {
                    let f = File::open(file)?;
                    grep3(&target, BufReader::new(f))?;
                }
            }

            Ok(())
        }

        let f = File::open("test.txt").unwrap();
        let reader = BufReader::new(f);
        let lines = reader.lines();

        // Vec<io::Result<String>>じゃなくResult<Vec<String>>をcollectできる
        // let z = lines.collect::<Vec<io::Result<String>>>();
        let x = lines.collect::<io::Result<Vec<String>>>();

        // writer
        // writer.write(&buf), write_all(&buf), flush(): バッファされたデータを書き出す

        // file
        // File::open(filename), File::create(filename)
        // OpenOptions

        // Seek

        // 他のreader, writer
        // io::stdin(), io::stdout(), io::stderr()
        // Vec<u8>
        // Cursor::new(buf) bufから読み出すバッファ付きreaderであるCursorを生成すsる
        // std::net::TcpStream
        // std::process::Command
        // io::sink(), io::empty(), io::repeat(byte)
    }

    // ファイルとディレクトリ
    {
        // OsStr, Path
        // ファイル名に有効ではないUnicodeがありうるのでそれに対処するのがstd::ffi::OsStr, OsString
        // Stringはヒープ上に確保されたstrを所有する
        // std::ffi::OsStringはヒープ上に確保されたOsStrを所有する
        // std::path::PathBufはヒープ上に確保されたPathを所有する

        // Path::new(str)
        // .parent(), file_name(), is_absolute(), is_relative(), join(path2), components(), ancestors(), to_str(), to_string_lossy(), display()
        // create_dir, remove_dir, remove_file, copy, rename, hard_link, ...

        // プラットフォーム固有
    }

    // ネットワークプログラム
    {
        // std::netで低レベルなネットワークをかける

        use std::error::Error;
        use std::io;

        fn http_get_main(url: &str) -> Result<(), Box<dyn Error>> {
            let mut response = reqwest::blocking::get(url)?;
            if !response.status().is_success() {
                return Err(format!("{}", response.status()).into());
            }

            let stdout = io::stdout();
            io::copy(&mut response, &mut stdout.lock())?;
            Ok(())
        }
    }
}
