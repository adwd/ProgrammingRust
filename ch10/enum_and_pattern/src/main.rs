use std::collections::HashMap;

fn main() {
    // 列挙型は他の言語にもあるが、Rustは値を持てるので強力
    enum Color {
        Red,
        Green(i32),
        Blue(i32, u32),
    }

    // 通常0から順番に番号が振られるが自分で決めることもできる
    enum HttpStatus {
        Ok = 200,
        NotModified = 304,
        NotFound = 404,
    }

    // == などを使うには明示的に必要
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum TimeUnit {
        MilliSecond,
        Second,
        Minute,
        Hour,
        Day,
    }

    // 列挙型はメソッドも持てる
    impl TimeUnit {
        fn plural(self) -> &'static str {
            match self {
                TimeUnit::MilliSecond => "milliseconds",
                TimeUnit::Second => "seconds",
                TimeUnit::Minute => "minutes",
                TimeUnit::Hour => "hours",
                TimeUnit::Day => "days",
            }
        }
    }

    // データを保持する列挙型
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum RoughTime {
        InThePast(TimeUnit, u32), // タプル型ヴァリアント
        JustNow,
        InTheFuture(TimeUnit, u32),
    }

    struct Point3d;
    enum Shape {
        Sphere { center: Point3d, radius: f32 }, // 構造体ヴァリアント
        Cuboid { center1: Point3d, center2: Point3d },
        None,
        Point(f32), // ヴァリアントが混ざってもいい
    }

    // 列挙型を用いた立地なデータ構造
    {
        // 列挙型はツリー状のデータ構造に向いている
        use std::collections::HashMap;

        enum Json {
            Null,
            Boolean(bool),
            Number(f64),
            String(String),
            Array(Vec<Json>),
            Object(Box<HashMap<String, Json>>), // メモリを節約するためにBoxを使う
        }
    }

    // ジェネリック列挙型
    enum Option<T> {
        None,
        Some(T),
    }

    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>),
    }

    struct TreeNode<T> {
        value: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }
    // メモリ配置から考えてコードに落とし込むといいらしい

    // パターン
    {
        fn rough_time_to_english(rt: RoughTime) -> String {
            match rt {
                RoughTime::InThePast(unit, n) => format!("{} {} ago", n, unit.plural()),
                RoughTime::JustNow => "just now".to_string(),
                // パターンはすごいたくさんある
                RoughTime::InTheFuture(_, 1) => "リテラル".to_string(),
                RoughTime::InTheFuture(_, 2..=10) => "範囲".to_string(),
                RoughTime::InTheFuture(_, count @ 11..=20) => {
                    "サブパターンを用いた束縛".to_string()
                }
                RoughTime::InTheFuture(TimeUnit::Hour, _) => "列挙型".to_string(),
                RoughTime::InTheFuture(TimeUnit::Day, ref count) => "参照、mutも".to_string(),
                // (tuple, t2), [a, b, c], [first, _, third], [first, .., last], []
                // Color(r, g, b), Point {x, y}, Card { suit: Clubs, rank: n }
                // Account { name, id, .. }
                // &v, &(k, v) 参照型にのみマッチ
                // 'a' | 'A', Some("right" | "left") Orパターン
                // _ ワイルドカードパターン
                // ガード式
                RoughTime::InTheFuture(_, count) if count > 100 => "参照、mutも".to_string(),
                RoughTime::InTheFuture(unit, n) => format!("{} {} from now", n, unit.plural()),
            }
        }

        #[derive(Debug)]
        struct Account {
            name: String,
            age: u8,
        }
        fn print(str: &str) {
            println!("{}", str);
        }
        let mut a = Account {
            name: "Alice".to_string(),
            age: 20,
        };
        match &mut a {
            // Account { name, age } => {
            //     print(&name);
            //     // borrow of partially moved value: `a` partial move occurs because `a.name`
            //     // has type `std::string::String`, which does not implement the `Copy` trait
            //     println!("{:?}", a);
            // }
            Account { ref name, age: 10 } => {
                print(name);
                println!("{:?}", a);
            }
            Account { ref mut name, .. } => {
                name.push_str(" pushed");
                print(name);
                println!("{:?}", a);
            }
        }

        let chars = vec!['a', 'b', 'c'];
        match &chars.into_iter().peekable().peek() {
            Some(&c) => println!("{}", c),
            _ => println!("not b"),
        }

        let c = 'x';
        match c {
            '0'..='9' => println!("0-9"),
            'a'..='z' | 'A'..='F' => println!("a-f"),
            char @ 'G'..='Z' => println!("{}", char),
            _ => println!("other"),
        }
        let Account { name, age } = a;
        fn _d((x, y): (i32, i32)) -> i32 {
            x + y
        }
        for (k, v) in HashMap::<String, i32>::new() {
            println!("{}:{}", k, v);
        }
        // An ordered collection of `T`s.
        #[derive(Debug)]
        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>),
        }

        // A part of a BinaryTree.
        #[derive(Debug)]
        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>,
        }

        impl<T: Ord> BinaryTree<T> {
            fn add(&mut self, value: T) {
                match self {
                    BinaryTree::Empty => {
                        *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                            element: value,
                            left: BinaryTree::Empty,
                            right: BinaryTree::Empty,
                        }))
                    }
                    BinaryTree::NonEmpty(ref mut node) => {
                        if value <= node.element {
                            node.left.add(value);
                        } else {
                            node.right.add(value);
                        }
                    }
                }
            }
        }

        let mut tree = BinaryTree::Empty;
        tree.add("test");
        tree.add("b");
        print!("{:?}", tree);
    }
}
