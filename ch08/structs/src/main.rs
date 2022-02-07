use std::{
    cell::{Cell, RefCell},
    fs::File,
    rc::Rc,
};

fn main() {
    // 3種類の構造体がある
    // 名前フィールド型、タプル型、ユニット型
    struct _T1 {
        name: String,
        age: u32,
    }
    struct _T2(String, u32);
    struct _T3 {}

    // 構造体の名前はCamelCase
    // フィールド名はsnake_case
    // 構造体はデフォルトでプライベートなのでpubをつけないとそのモジュールとサブモジュールからしか見えない
    // VecやStringを構造体式で作れないのはプライベートになっているフィールドがあるから
    pub struct GrayscaleMap {
        pub pixels: Vec<u8>,
        size: (usize, usize),
    }

    let _g1 = GrayscaleMap {
        pixels: vec![0; 100],
        size: (10, 10),
    };
    let pixels = vec![0; 100];
    let size = (10, 10);
    let g2 = GrayscaleMap { pixels, size }; // 同じ名前の変数で省略できる

    let g3 = GrayscaleMap {
        pixels: vec![1; 100],
        // 指定しなかったフィールドを_g2から取得する
        ..g2
    };
    assert_eq!(g3.pixels[0], 1);
    assert_eq!(g3.size.0, 10);

    struct Broom {
        name: String,
        height: u32,
        width: u32,
        position: (f32, f32, f32),
        intent: BroomIntent,
    }
    #[derive(Clone, Copy)]
    enum BroomIntent {
        FetchWater,
        DumpWater,
    }
    fn chop(b: Broom) -> (Broom, Broom) {
        // StringはCopyではないので、broom1はbのnameの所有権を得る
        let mut broom1 = Broom {
            height: b.height / 2,
            ..b
        };

        // StringはCopyではないので、nameを明示的にCloneする
        let mut broom2 = Broom {
            name: broom1.name.clone(),
            ..broom1
        };

        broom1.name.push_str(" I");
        broom2.name.push_str(" II");

        (broom1, broom2)
    }

    // タプル構造体
    pub struct Bounds(usize, pub usize);
    let b = Bounds(10, 20);
    assert_eq!(b.0 * b.1, 200);

    // タプル構造体はニュータイプ(Newtype)を作るのに便利
    struct Ascii(Vec<u8>);

    // ユニット型構造体
    struct Onesuch;

    // implによるメソッド定義
    pub struct Queue {
        older: Vec<char>,
        younger: Vec<char>,
    }

    impl Queue {
        // selfを引数に取らない型関連関数
        // コンストラクタをnewで書くのは慣習
        pub fn new() -> Queue {
            Queue {
                older: Vec::new(),
                younger: Vec::new(),
            }
        }

        pub fn push(&mut self, c: char) {
            self.younger.push(c);
        }

        pub fn pop(&mut self) -> Option<char> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }
                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }

            self.older.pop()
        }
    }

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('0');
    (&mut q).push('1'); // こう書かなくても暗黙的に解決される

    impl Queue {
        fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }
    }

    assert!(!q.is_empty());

    impl Queue {
        // self なので所有権の移動が起こる
        pub fn split(self) -> (Vec<char>, Vec<char>) {
            (self.older, self.younger)
        }
    }

    // qは未定義状態になった
    let (older, younger) = q.split();
    assert!(older.is_empty());
    assert_eq!(younger[0], '0');

    // selfをBox、Rc、Arcで渡す
    // メソッドのself引数をBox<Self>, Rc<Self>, Arc<Self>とすることもでき、そのポインタ型のときのみ呼び出すことができる。
    // これらのメソッドを呼び出すとポインタの所有権が移動する
    // Boxに対しても普通に書いたメソッドを呼び出せるので通常は必要ない
    let mut bq = Box::new(Queue::new());
    bq.push('b');

    struct Node {
        children: Vec<Node>,
    }

    impl Node {
        fn append_to(self: Rc<Self>, parent: &mut Node) {
            // parent.children.push(self);
        }
    }

    // 型関連定数
    pub struct Vector2 {
        x: f32,
        y: f32,
    }

    impl Vector2 {
        const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
        const UNIT: Vector2 = Vector2 { x: 1.0, y: 1.0 };

        // 関連付けられた型と同じ方でなくても良い
        const NAME: &'static str = "Vector2";
        const ID: u32 = 18;
    }

    // ジェネリック構造体
    {
        pub struct Queue<T> {
            older: Vec<T>,
            younger: Vec<T>,
        }

        impl<T> Queue<T> {
            // 特別な型パラメータSelfで対象の型を指定できる
            pub fn new() -> Self {
                Queue {
                    older: Vec::new(),
                    younger: Vec::new(),
                }
            }
        }

        // impl<T>が冗長ぽいけど特定の方に対してimplブロックが書けるので
        impl Queue<f64> {}
    }

    // 生存期間パラメータを持つジェネリック構造体
    {
        struct Extrema<'el> {
            greatest: &'el i32,
            least: &'el i32,
        }

        fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
            let mut greatest = &slice[0];
            let mut least = &slice[0];

            for i in 1..slice.len() {
                if slice[i] < *least {
                    least = &slice[i]
                }
                if slice[i] > *greatest {
                    greatest = &slice[i];
                }
            }

            Extrema { greatest, least }
        }

        // Rustコンパイラが推論するので関数呼び出しの際に生存期間パラメータは書かなくて良い
        let a = [0, -3, 0, 15, 48];
        let e = find_extrema(&a);
        assert_eq!(*e.least, -3);
        assert_eq!(*e.greatest, 48);
    }

    {
        // 定数パラメータを持つジェネリック構造体
        struct Polynominal<const N: usize> {
            coefficients: [f64; N],
        }

        impl<const N: usize> Polynominal<N> {
            fn new(coefficients: [f64; N]) -> Self {
                Polynominal { coefficients }
            }

            fn eval(&self, x: f64) -> f64 {
                let mut sum = 0.0;
                for i in (0..N).rev() {
                    sum = self.coefficients[i] + x * sum;
                }
                sum
            }
        }

        use std::f64::consts::FRAC_PI_2;
        let sine_poly = Polynominal::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
        assert_eq!(sine_poly.eval(0.0), 0.0);
        assert!((sine_poly.eval(FRAC_PI_2) - 1.).abs() < 0.005);

        // 定数ジェネリックパラメータにできるのはすべての整数型とchar, boolのみ
        // 浮動小数点型、列挙型などは使えない

        // 複数のジェネリックパラメータを取る場合はライフタイムパラメータ、型パラメータ、定数パラメータの順に書く
        struct LumpOfReferences<'a, T, const N: usize> {
            the_lump: [&'a T; N],
        }

        // 定数ジェネリックパラメータが単純なリテラルや識別子でないときは波括弧でかこう
        type _P = Polynominal<{ 3 + 4 }>;
    }

    // 一般的なトレイとの自動実装
    {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Point {
            x: f64,
            y: f64,
        }
    }

    // 内部可変性
    {
        pub struct SpiderRobot {
            species: String,
            web_enabled: bool,
            // leg_devices: [fd::FileDesc; 8],
            log_file: RefCell<File>,
            hardware_error_count: Cell<u32>,
        }

        pub struct SpiderSenses {
            robots: Rc<SpiderRobot>,
            //
        }
        // 殆どの部分が不変だけど一部が可変データの場合、Cell<T>, RefCell<T>が使える

        // Cell<T>: Cellそのものに対するmutな参照を持っていなくてもそのフィールドを見たりセットできる
        let value = 10;
        let c = Cell::new(value);
        assert_eq!(c.get(), 10);
        c.set(20); // 以前の値をドロップし、値をcellの中にセットする
        assert_eq!(c.get(), 20);

        // fn set(&self, value: T)
        // これはいままでの所有権のモデルと異なる

        impl SpiderRobot {
            pub fn add_hardware_error(&self) {
                let n = self.hardware_error_count.get();
                self.hardware_error_count.set(n + 1);
            }
        }

        // RefCell<T>はCell<T>と同様に型Tの値を一つだけ持つ
        // Cellと違ってRefCellはT値への参照の借用をサポートしている
        let value = 19;
        let r = RefCell::new(value);
        r.borrow(); // 値への共有参照をRef<T>として返す。値がすでに可変参照で借用されていた場合はパニックを起こす
        r.borrow_mut(); // 値への可変参照をRefMut<T>として返す。値がすでに借用されていた場合はパニックを起こす
                        // パニックではなくResultを返すメソッド
        r.try_borrow();
        r.try_borrow_mut();
        {
            let rc = RefCell::new("hello".to_string());

            let r = rc.borrow();
            let count = r.len();
            assert_eq!(count, 5);

            // let mut w = rc.borrow_mut();
            // w.push_str(" world");
        }

        impl SpiderRobot {
            fn log(&self, message: &str) {
                let mut file = self.log_file.borrow_mut();
                // writeln!(file, "{}", message);
            }
        }

        // Cellはスレッドセーフではないので、スレッドからCellにアクセスできない。
    }
}
