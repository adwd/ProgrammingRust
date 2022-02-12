fn main() {
    {
        use std::{net, thread};

        let address = "0.0.0.0";
        let listener = net::TcpListener::bind(address).unwrap();

        for socket_result in listener.incoming() {
            let socket = socket_result.unwrap();
            // let groups = chat_group_table.clone();
            // thread::spawn(|| {
            // log_error(serve(socket, groups));
            // });
        }
    }
    // こんな感じだと数万の接続でメモリ使用量がGBを超えてしまう
    // 非同期タスクはスレッドよりも遥かに少ないコストで実行できる
    {
        async fn chat() -> std::io::Result<()> {
            use async_std::prelude::*;
            use async_std::{net, task};

            let listener = net::TcpListener::bind("0.0.0.0").await?;

            let mut new_connections = listener.incoming();
            while let Some(socket_result) = new_connections.next().await {
                let socket = socket_result?;
                task::spawn(async {
                    // log_error(serve(socket, chat_group_table.clone()));
                });
            }

            Ok(())
        }
    }
    // 非同期プログラムの動作機構: フューチャー、非同期関数、await式、タスク、エグゼキュータ(block_on, spawn_local)
    // 非同期ブロック、spawnエグゼキュータ
    // Pin型

    {
        use std::io::prelude::*;
        use std::net;

        fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
            let mut socket = net::TcpStream::connect((host, port))?;

            let request = format!("GET {} HTTP/1.1\r\nHost: {} \r\n\r\n", path, host);
            socket.write_all(request.as_bytes())?;
            socket.shutdown(net::Shutdown::Write)?;

            let mut response = String::new();
            socket.read_to_string(&mut response);

            Ok(response)
        }
        // この関数はほとんどシステムコールを待つ時間になりその間スレッドをブロックしてしまう
        // 関数が同期型のため
        // 非同期型の関数を使えばシステムコールを待つ間スレッドは別のことをできる

        // Feature
        use std::pin::Pin;
        use std::task::Context;
        trait Future2 {
            type Output;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll2<Self::Output>;
        }
        enum Poll2<T> {
            Ready(T),
            Pending,
        }
        // Featureは終了を確認できる関数を返す
        // Pendingを返した場合、再度pollを呼び出すべき状況になったらContextの中のwakerコールバックを呼び出すことになっている
        // fn read_to_string(&mut self, buf: &mut String) -> Result<usize>
        // fn read_to_string(&mut self, buf: &mut String) -> impl Future<Output=Result<usize>>
        // Furureを返す関数を呼んだだけでは処理は行われず、実際の仕事はpollで行われる
        // Futureは呼び出された対象の入力ストリームと読みだしたデータを追記していくStringを覚えておくのでシグネチャはこうなる
        // fn read_to_string<'a>(&'a mut self, buf: &'a mut String) -> impl Future<Output=Result<usize>> + 'a
        // selfとbufが借用する生存期間の範囲内でしかFutureが生存できないことを示す

        // async関数とawait式
        {
            use async_std::net;
            use async_std::prelude::*;
            async fn cheapo_request2(host: &str, port: u16, path: &str) -> std::io::Result<String> {
                let mut socket = net::TcpStream::connect((host, port)).await?;

                let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
                socket.write_all(request.as_bytes()).await?;
                socket.shutdown(net::Shutdown::Write)?;

                let mut response = String::new();
                socket.read_to_string(&mut response).await?;

                Ok(response)
            }
            // cheapo_request2はpollされるたびにTcpStream::conect, socket.write_all, read_to_stringのawait式が返すポーリングを返す。
            // 再度ポーリングされた際はその途中から継続される

            // 非同期関数を同期コードから呼び出す: block_on
            fn fake_main() -> std::io::Result<()> {
                use async_std::task;

                let response = task::block_on(cheapo_request2("example.com", 80, "/"))?;
                println!("{response}");
                Ok(())
            }
            // block_onがスレッドをブロックしてしまうので同期呼び出しを変わらない
            // async_std::task::spawn_local
            // spawn_localにFutureをたくさん与え、block_onすると個々のFeatureが再起に進める状態になるたびにポーリングが行われる
            // まだunstable

            async fn many_requests(
                requests: Vec<(String, u16, String)>,
            ) -> Vec<std::io::Result<String>> {
                use async_std::task;

                let mut handles = vec![];
                for (host, port, path) in requests {
                    handles.push(task::spawn_local(cheapo_owning_request(host, port, path)));
                }

                let mut results = vec![];
                for handle in handles {
                    results.push(handle.await);
                }

                results
            }

            // 非同期関数に参照を渡すとライフタイムの問題があるので、所有権を移動させて'staticにする
            async fn cheapo_owning_request(
                host: String,
                port: u16,
                path: String,
            ) -> std::io::Result<String> {
                cheapo_request2(&host, port, &path).await
            }

            // 非同期タスクの切り替えはawait式を実行しPendingが帰ってきた場合だけなので、
            // 時間のかかる計算をすると他のタスクは実行する機会を得られない
            // スレッドの場合はOSが任意のスレッドをサスペンドして切り替えられる
            // 長時間実行と非同期コードを共存させる方法は後述する

            // 非同期ブロック
            let serve_one = async {
                // let listener = net::TcpListener::bind("localhost:8080").await?;
                // let (mut socket, _addr) = listener.accept().await?;
            };

            let serve_two = async move {};

            async fn many_requests2(
                requests: Vec<(String, u16, String)>,
            ) -> Vec<std::io::Result<String>> {
                use async_std::task;

                let mut handles = vec![];
                for (host, port, path) in requests {
                    handles.push(task::spawn_local(async move {
                        cheapo_request2(&host, port, &path).await
                    }));
                }

                let mut results = vec![];
                for handle in handles {
                    results.push(handle.await);
                }

                results
            }

            let input = async_std::io::stdin();
            let future = async {
                let mut line = String::new();

                input.read_line(&mut line).await?;

                print!("Read line: {line}");
                // Ok(()) // cannot infer type for type parameter `E` declared on the enum `Result`
                Ok::<(), std::io::Error>(())
            };

            // 非同期ブロックを用いた非同期関数の記述
            // async fnだと関数のボディが即座に実行されないのでそうした場合他の書き方にする
            fn cheapo_request3(
                host: &str,
                port: u16,
                path: &str,
            ) -> impl Future<Output = std::io::Result<String>> + 'static {
                let host = host.to_string();
                let path = path.to_string();
                async move {
                    // ...
                    Ok("ok".to_string())
                }
            }

            // 非同期タスクをスレッドプールで実行
            // プロセッサを使う処理とブロック街の処理が混ざるようなワークロード
            // async_std::task::spawnを用いてスレッドプールでFutureを実行できる
            {
                use async_std::task;
                let mut handles = vec![];
                let requests = vec![("example.com", 80, "/")];
                for (host, port, path) in requests {
                    handles.push(task::spawn(async move {
                        cheapo_request2(host, port, path).await
                    }));
                }
            }
            // この場合、スレッドローカルなストレージをつかってると問題になるのでタスクローカルストレージを使うのが良い

            // spawnはspawn_localと違って別スレッドで実行するのでFutureがSendトレイトを実装している必要がある
            // std::thread::spawnと同じだが、非同期タスクは実行するたびに別のスレッドに移動する可能性がある
            {
                use async_std::task;
                use std::rc::Rc;
                async fn some_async_thing() {
                    // ...
                }

                async fn reluctant() -> String {
                    let string = Rc::new("ref-counted string".to_string());

                    some_async_thing().await;
                    format!("Your splendid string; {string}")
                }

                task::spawn(reluctant());
            }
        }
    }
}
