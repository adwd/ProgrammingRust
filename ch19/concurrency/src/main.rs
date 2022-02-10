use std::collections::HashMap;

fn main() {
    // 並列性

    // システムプログラマが使うイディオム
    // バックグラウンドスレッド: 1つだけ仕事があり、定期的に起きてその仕事を行う
    // 汎用ワーカプール: クライアントとタスクキューで通信する
    // パイプライン: データはこのパイプラインを通じて、あるスレッドから次のスレッドへ流れていく。個々のスレッドは仕事の一部を行う。
    // データ並列: コンピュータ全体が一つの大きな計算だけを行っている。その計算を分割してスレッドで実行する
    // 同期オブジェクトの海:
    // アトミックな整数処理

    // Rustで利用できる安全な並列スタイル3つを示す
    // フォークジョイン並列
    // チャネル
    // 可変状態の共有

    // フォークジョイン並列
    {
        // 複数の完全に独立したタスクを一度に実行する
        // フォークジョインの長所
        // 非常に単純
        // ボトルネックがない。共有資源のロックがないので別のスレッドを待つことがない
        // 性能を計算するのが簡単。理想的には4つのスレッドで4倍の速さになる
        // プログラムの正しさを推測するのが容易
        // 欠点は独立して分解できる仕事でなければ使えないこと

        use std::thread;

        // FnOnceか関数を引数として取る
        let handle = thread::spawn(|| {
            println!("Hello from a child thread");
        });
        handle.join().unwrap();

        use std::io;
        fn process_files(v: Vec<String>) -> io::Result<()> {
            println!("{:?}", v);
            Ok(())
        }
        fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
            const NTHREADS: usize = 8;
            let worklists: Vec<Vec<String>> = Vec::new(); // split_vec_int_chunks(filenames, NTHREADS);

            let mut thread_handles = vec![];
            for worklist in worklists {
                thread_handles.push(thread::spawn(move || process_files(worklist)));
            }

            for handle in thread_handles {
                handle.join().unwrap()?;
            }
            Ok(())
        }
        // handle.join()は子スレッドがパニックを起こした場合にエラーになる

        // 不変データのスレッド共有
        // Arcでアトミックな参照カウントを取る
        use std::sync::Arc;
        type GigabyteMap = HashMap<String, String>;

        fn process_files2(v: Vec<String>, glossary: &GigabyteMap) -> io::Result<()> {
            println!("{:?}", v);
            Ok(())
        }

        fn process_files_in_parallel2(
            filenames: Vec<String>,
            glossary: Arc<GigabyteMap>,
        ) -> io::Result<()> {
            const NTHREADS: usize = 8;
            let worklists: Vec<Vec<String>> = Vec::new(); // split_vec_int_chunks(filenames, NTHREADS);

            let mut thread_handles = vec![];
            for worklist in worklists {
                let glossary_for_child = glossary.clone();
                thread_handles.push(thread::spawn(move || {
                    process_files2(worklist, &glossary_for_child)
                }));
            }

            for handle in thread_handles {
                handle.join().unwrap()?;
            }
            Ok(())
        }

        // Rayon
        // 標準ライブラリのspawn関数は便利だがフォークジョイン向けに設計されているわけではない
        // 2章でつかったCrossbeamはスコープ付きスレッドでフォークジョイン並列を自然な形でサポートする
        // Rayonもフォークジョイン向けのライブラリ
        use rayon::prelude::*;

        // let (v1, v2) = rayon::join(fn1, fn2);

        // giant_vector.par_iter().for_each(|value| {
        // do_thins_with_value(value);
        // });

        fn process_files3(v: &str, glossary: &GigabyteMap) -> io::Result<()> {
            println!("{:?}", v);
            Ok(())
        }

        fn process_files_in_parallel3(
            filenames: Vec<String>,
            glossary: &GigabyteMap,
        ) -> io::Result<()> {
            filenames
                .par_iter()
                .map(|filename| process_files3(filename, glossary))
                .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
                .unwrap_or(Ok(()))
        }
        // rayonはスレッド間の共有参照をサポートしているのでArcを使わなくてもglossaryを渡せる
    }

    // チャネル
    {
        //
    }
}
