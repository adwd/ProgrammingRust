fn main() {
    {
        let mut v = vec![1, 2, 3];
        v.sort_by_key(|i| -i);
        // | arg | { ... } がクロージャ

        // クロージャはキャプチャした外側の値の生存期間に縛られる
        struct V {
            value: i32,
        }
        use std::thread;
        fn start_sort(mut v_arr: Vec<V>, v: V) -> thread::JoinHandle<Vec<V>> {
            // moveでvの所有権をkey_fnに移動する
            let key_fn = move |vv: &V| -> i32 { -vv.value - v.value };

            // moveでv_arr, key_fnの所有権を移動する
            // この関数が所有する値が無く、全てスレッドに移動しているので安全に関数を終了できる
            thread::spawn(move || {
                v_arr.sort_by_key(key_fn);
                v_arr
            })
        }
        // スレッドが終了するまで生存してしまうので、v_arrとvの所有権を借用ではなく移動(move)するようにRustに指示する
    }

    // 関数型とクロージャ型
    {
        struct City {
            name: String,
        }

        // 関数とクロージャは同じ型ではない
        fn count_selected_cities<F>(cities: &[City], f: F) -> usize
        where
            F: Fn(&City) -> bool,
        {
            let mut count = 0;
            for city in cities {
                if f(city) {
                    count += 1;
                }
            }
            count
        }
        // Fnトレイト
        // fn(&City) -> bool; fn型(関数のみ)
        // Fn(&City) -> bool: Fnトレイト(関数とトレイト)
        // クロージャはキャプチャする外側のデータによってRustがアドホックに型を付けている
        // その型はシグネチャに合わせてFn(&City) -> boolトレイトを実装するようになっている
        // Rustのクロージャは性能上の欠点はない
        // 型が決まるのでヒープに確保されることはなく、インライン化することができる
    }

    // クロージャと安全性
    {
        // 殺すクロージャ
        let s = "hello".to_string();
        let f = || drop(s);
        f();
        // f(); // use of moved value: `f` value used here after move

        fn call_twice<F>(closure: F)
        where
            F: Fn(),
        {
            closure();
            closure();
        }
        let s = "hello".to_string();
        // expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
        // this closure implements `FnOnce`, not `Fn`
        let f = || drop(s);
        // call_twice(f);

        // 値をドロップするクロージャはFnではなくFnOnceを実装する。
        // このトレイトのクロージャは一度だけしか呼び出されることができない

        // FnMut
        // mut参照を持つクロージャ
        // Fn()はFnMut()fのサブトレイト、FnMut()はFnOnce()のサブトレイト
        fn call_twice2<F>(mut closure: F)
        where
            F: FnMut(),
        {
            closure();
            closure();
        }

        // Rustコンパイラはクロージャが一度しか呼び出せないか、Copy、Cloneかを自動的に判断できる
        // クロージャは構造体として表現され、それに対するCopy、Cloneは他の構造体とほとんど同じ
    }

    // コールバック
    {
        use std::collections::HashMap;

        // actix-webのルータを自前で書いてみる
        struct Request {
            method: String,
            url: String,
            headers: HashMap<String, String>,
            body: Vec<u8>,
        }

        struct Response {
            code: u32,
            headers: HashMap<String, String>,
            body: Vec<u8>,
        }
        // URLからコールバックへのマップを保持しておき、必要に応じてコールバックを呼ぶ
        type BoxedCallback = Box<dyn Fn(&Request) -> Response>;
        struct BasicRouter {
            routes: HashMap<String, BoxedCallback>,
        }

        impl BasicRouter {
            /// create an empty router.
            fn new() -> BasicRouter {
                BasicRouter {
                    routes: HashMap::new(),
                }
            }

            /// Add a route to the router.
            fn add_route<C>(&mut self, url: &str, callback: C)
            where
                C: Fn(&Request) -> Response + 'static,
            {
                self.routes.insert(url.to_string(), Box::new(callback));
            }

            fn handle_request(&self, request: &Request) -> Response {
                match self.routes.get(&request.url) {
                    None => Response {
                        code: 400,
                        headers: HashMap::new(),
                        body: Vec::new(),
                    },
                    Some(callback) => callback(request),
                }
            }
        }

        // 変数をキャプチャするクロージャを使えない代わりに性能を向上させる
        struct FnPointerROuter {
            routes: HashMap<String, fn(&Request) -> Response>,
        }
        let mut fnr = FnPointerROuter {
            routes: HashMap::new(),
        };
        fnr.routes.insert("/".to_string(), |_| Response {
            code: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        });
    }

    // クロージャの効率的な利用
    {
        // Rustで例えばMVCのような相互参照のあるデザインパターンで「オブジェクトの海」を作ろうとすると大変面倒
        // Rustが課す制約の中で他の方法を取るのが良い
    }
}
