use std::{collections::HashMap, path::PathBuf, thread::JoinHandle};

mod index;

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
        // あるスレッドから別のスレッドに値を送信する一方通行のパイプ、スレッド安全なキューと考えても良い
        // 値をコピーではなく所有権の移動で行うのでサイズが大きくても高速
        // 転置インデックスを並列に作る
        // https://github.com/ProgrammingRust/fingertips

        use std::sync::mpsc;
        use std::{fs, thread};

        fn start_file_reader_thread(
            documents: Vec<PathBuf>,
        ) -> (
            mpsc::Receiver<String>,
            thread::JoinHandle<std::io::Result<()>>,
        ) {
            let (sender, receiver) = mpsc::channel();

            let handle: JoinHandle<Result<(), std::io::Error>> = thread::spawn(move || {
                for filename in documents {
                    let text = fs::read_to_string(filename)?;

                    if sender.send(text).is_err() {
                        break;
                    }
                }
                Ok(())
            });

            (receiver, handle)
        }

        fn start_file_indexing_thread(
            texts: mpsc::Receiver<String>,
        ) -> (mpsc::Receiver<index::InMemoryIndex>, thread::JoinHandle<()>) {
            let (sender, receiver) = mpsc::channel();

            let handle = thread::spawn(move || {
                for (doc_id, text) in texts.into_iter().enumerate() {
                    let index = index::InMemoryIndex::from_single_document(doc_id, text);
                    if sender.send(index).is_err() {
                        break;
                    }
                }
            });

            (receiver, handle)
        }

        // mpsc: multiple producer, single consumer

        // バックプレッシャーの仕組みを持つ同期チャネルがある
        let (sender, receiver) = std::sync::mpsc::sync_channel::<String>(1000);

        // スレッド安全性: Send, Sync
        // Sendを実装する型は他のスレッドに値で渡しても安全で、スレッド間で移動もできる
        // Syncを実装する型は他のスレッドに非mut参照で渡しても安全で、スレッド間で共有もできる
        // ほとんどの型はSendかつSync
        // Cell, ReceiverはSendではない
        // Rc, *mutはどちらでもない
        // thread::spawnで渡すクロージャはSendでなければならない（クロージャに含まれるすべての値がSendでなければならない）

        pub trait OffThreadExt: Iterator {
            fn off_thread(self) -> std::sync::mpsc::IntoIter<Self::Item>;
        }

        impl<T> OffThreadExt for T
        where
            T: Iterator + Send + 'static,
            T::Item: Send + 'static,
        {
            fn off_thread(self) -> std::sync::mpsc::IntoIter<Self::Item> {
                let (sender, receiver) = std::sync::mpsc::sync_channel(1024);
                std::thread::spawn(move || {
                    for item in self {
                        if sender.send(item).is_err() {
                            break;
                        }
                    }
                });
                receiver.into_iter()
            }
        }
    }

    // 可変状態の共有
    {
        // 排他ロック、リード・ライトロック、条件変数、アトミック整数について説明する
        // 排他ロック
        // データ競合を防ぐ
        // 不変条件(invariant)を用いたプログラミングをサポートする
        // Mutex<T>
        type PlayerId = u32;
        const GAME_SIZE: usize = 8;
        type WaitingList = Vec<PlayerId>;

        use std::sync::Mutex;

        struct FernEmpireApp {
            waiting_list: Mutex<WaitingList>,
        }

        use std::sync::Arc;

        let app = Arc::new(FernEmpireApp {
            waiting_list: Mutex::new(Vec::new()),
        });

        impl FernEmpireApp {
            fn join_waiting_list(&self, player: PlayerId) {
                let mut guard = self.waiting_list.lock().unwrap();

                guard.push(player);
                if guard.len() == GAME_SIZE {
                    let players = guard.split_off(0);
                    drop(guard);
                    self.start_game(players);
                }
            }

            fn start_game(&self, players: Vec<PlayerId>) {
                // ...
            }
        }

        // join_waiting_listはselfはmut参照ではないのにwaiting_listをいじれている
        // Rustでは&mutは排他アクセスを意味する。mutでない&による参照は共有アクセスを意味する
        // 親から子へと&mutアクセスを渡していくのが通常のやり方
        // Mutexは静的ではなく動的に排他アクセスを提供している
        // RefCellとMutexは内部可変性を実装するもの
        // Mutexは原因の特定が難しいバグを生みやすいのでより構造化された手段(チャネルなど)を使えない場合だけにする
        // Rustの借用システムではデッドロックを防ぐことはできない
        // Mutexを保持したスレッドがパニックになるとRustはそのMutextを毒されたものとしてマークする

        // 複数の消費者を持つチャネル
        pub mod shared_channel {
            use std::sync::mpsc::{channel, Receiver, Sender};
            use std::sync::{Arc, Mutex};

            #[derive(Clone)]
            pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

            impl<T> Iterator for SharedReceiver<T> {
                type Item = T;

                fn next(&mut self) -> Option<T> {
                    let guard = self.0.lock().unwrap();
                    guard.recv().ok()
                }
            }

            pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
                let (sender, receiver) = channel();
                (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
            }
        }

        // リードライトロック RwLock<T>
        // 読み込みは複数のスレッドで同時に行っても安全なのでリードのロックが取れるようになっているもの
        {
            use std::sync::RwLock;
            struct AppConfig {
                mushrooms_enabled: bool,
            }
            impl AppConfig {
                fn load() -> std::io::Result<Self> {
                    Ok(Self {
                        mushrooms_enabled: true,
                    })
                }
            }
            struct FernEmpireApp {
                config: RwLock<AppConfig>,
            }

            impl FernEmpireApp {
                fn mushrooms_enabled(&self) -> bool {
                    let config_guard = self.config.read().unwrap();
                    config_guard.mushrooms_enabled
                }

                fn reload_config(&self) -> std::io::Result<()> {
                    let new_config = AppConfig::load()?;
                    let mut config_guard = self.config.write().unwrap();
                    *config_guard = new_config;
                    Ok(())
                }
            }
        }

        // 条件変数 Condvar
        // ある条件をスレッドが待つ
        use std::sync::Condvar;
        // .wait(), .notify_all()

        // アトミック整数
        // アトミックな値に対しては複数のスレッドが同時に読み書きしてもデータ競合が発生しない
        use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

        let atom = AtomicIsize::new(0);
        atom.fetch_add(1, Ordering::SeqCst);

        // グローバル変数
        // アトミック変数にするとスレッド安全にグローバル変数を使える
        // Mutexx, RwLock, アトミック変数は非mutで宣言されていても変更できる
        static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);

        // 関数シグネチャの冒頭にconstをつければconst関数を定義できる
        // const関数でないものでもlazy_static!を使うと定義できる

        use lazy_static::lazy_static;
        lazy_static! {
            static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
        }
    }
}
