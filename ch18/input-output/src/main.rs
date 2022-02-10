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
    }
}
