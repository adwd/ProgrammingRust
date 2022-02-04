use std::vec;

fn main() {
    // 参照は所有権を持たないポインタ型
    // 参照は参照先よりも長生きしてはいけない
    // ある値に対して参照を作ることを借用(borrowing)と呼ぶ
    {
        use std::collections::HashMap;

        type Table = HashMap<String, Vec<String>>;

        fn show(table: &Table) {
            for (artist, works) in table {
                println!("works by {}:", artist);
                for work in works {
                    println!("{}", work);
                }
            }
        }

        let mut table = Table::new();
        table.insert(
            "Gesualdo".to_string(),
            vec![
                "many madrigals".to_string(),
                "Tenebrae Responsoria".to_string(),
            ],
        );
        table.insert(
            "Caravaggio".to_string(),
            vec![
                "The Musicians".to_string(),
                "The Calling of St. Matthew".to_string(),
            ],
        );
        table.insert(
            "Callini".to_string(),
            vec![
                "Perseus with the head of Medusa".to_string(),
                "a salt celler".to_string(),
            ],
        );

        // show(table);
        // println!("{}", table["Gesualdo"][0]); // move occurs because `table` has type `HashMap<String, Vec<String>>`, which does not implement the `Copy` trait

        // &T 共有参照(shared reference)は同時にいくつでも持てる。共有参照はCopy型である。
        // &mut T 可変参照(mutable reference)は他の参照と同時に使用できない。可変参照はCopy型でない。
        // 複数読み出しか単一書き込みをコンパイル時に強制する
        // 共有参照がある間は借用する側だけでなく、所有者も値の変更を禁じられる。
        // 可変参照がある間は所有者はその値を使うことができない。
        show(&table);

        // HashMapの共有参照に対する繰り返し実行は個々のエントリのキーと値へも共有参照を作るようになっている。

        fn sort_works(table: &mut Table) {
            for (_artist, works) in table {
                works.sort();
            }
        }

        // 値の所有権を移動して関数へ値を渡すことを値渡し(pass by value)と呼び、
        // 関数に値の参照を渡すことを参照渡し(pass by reference)と呼ぶ。
        sort_works(&mut table);
        show(&table);

        {
            let x = 10;
            let r = &x;
            assert!(*r == 10);

            let mut y = 32;
            let m = &mut y;
            *m += 32;
            assert!(*m == 64);
        }
        {
            // Rustでは.演算子が必要に応じて暗黙に左のオペランドを参照解決するようになっている
            struct Anime {
                name: &'static str,
                bechdel_pass: bool,
            }
            let aria = Anime {
                name: "Aria",
                bechdel_pass: true,
            };
            let anime_ref = &aria;
            assert_eq!(anime_ref.name, "Aria");

            assert_eq!((*anime_ref).name, "Aria");
        }
        {
            // .演算子はメソッド呼び出しの際に必要であれば暗黙的に左オペランドへの参照を借用する
            let mut v = vec![123, 121];
            v.sort();
            (&mut v).sort();
        }
        {
            // 参照の代入
            let x = 10;
            let y = 20;
            let mut r = &x;

            if false {
                r = &y;
            }
            assert!(*r == 10 || *r == 20);
        }
        {
            // 参照の参照
            struct Point {
                x: i32,
                y: i32,
            }
            let point = Point { x: 1000, y: 729 };
            let r: &Point = &point;
            let rr: &&Point = &r;
            let rrr: &&&Point = &rr;
            assert_eq!(rrr.y, 729);
        }
        {
            let x = 10;
            let y = 20;
            let rx = &x;
            let ry = &y;
            let rrx = &rx;
            let rry = &ry;

            assert!(rrx < rry);
            assert!(rx < ry);
            // assert!(rrx < ry); // mismatched types expected reference `&&_` found reference `&{integer}`
        }

        // Rustの参照はnullにはならない
        // Option<&T>が使われる

        {
            // 任意の式への参照の借用
            fn factorial(n: usize) -> usize {
                (1..n + 1).product()
            }
            let r = &factorial(6);
            assert_eq!(r + &1009, 1729);
            // なんのための機能・・・？
        }

        // Rustには2種類のファットポインタがある
        // ファットポインタはなんらかの値へのアドレスと、その値を使うために必要な情報を持つワードの2ワードで構成される
        // 一つはスライスへの参照で、もう一つが特定のトレイトを実装した値への参照であるトレイトオブジェクト

        {
            // let r;
            {
                let x = 1;
                // r = &x; // `x` does not live long enough borrowed value does not live long enough
            }
            // assert_eq!(*r, 1);

            // Rustコンパイラはすべての参照型にたいして生存期間(lifetime)を割り当てる。
        }

        {
            static mut STASH: &i32 = &128;
            fn f(p: &'static i32) {
                unsafe {
                    STASH = p;
                }
            }
            static PP: i32 = 1000;
            f(&PP);
            // Rustでグローバル変数に該当するものはstaticと呼ばれる
            // この変数はプログラムが実行開始する際に作られ、終了するまで維持される
            // f<'a>(p: &'a i32) { ... }
            // 'a (tick A)はfの生存期間パラメータ
        }

        {
            fn g<'a>(p: &'a i32) {
                // ...
            }

            let x = 10;
            g(&x);
        }

        {
            // 返り値としての参照
            fn smallest(v: &[i32]) -> &i32 {
                let mut s = &v[0];
                for r in &v[1..] {
                    if *r < *s {
                        s = r;
                    }
                }
                s
            }

            let s;
            {
                let parabola = [9, 4, 1, 0, 1, 4, 9];
                s = smallest(&parabola);
                assert_eq!(*s, 0);
            }
            // assert_eq!(*s, 0);
        }

        {
            // 参照を含む構造体
            struct S {
                r: &'static i32,
            }

            struct S2<'a> {
                r: &'a i32,
            }

            struct D {
                s: S2<'static>,
            }
            struct D2<'a> {
                s: S2<'a>,
            }
        }

        {
            // 個別の生存期間パラメータ
            struct S<'a, 'b> {
                x: &'a i32,
                y: &'b i32,
            }

            let x = 10;
            let r;
            {
                let y = 20;
                {
                    let s = S { x: &x, y: &y };
                    r = s.x;
                }
            }
            println!("{}", r);
            // ライフタイムパラメータが多くなるとシグネチャが読みにくくなる
            // 単純な定義から初めてコンパイルできるまで制約を緩めていけばいい
        }

        {
            // 共有と変更
            let v = vec![1, 2, 3];
            let r = &v;
            // let aside = v; // cannot move out of `v` because it is borrowed move out of `v` occurs here
            r[0];

            fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
                for el in slice {
                    vec.push(*el);
                }
            }

            let mut wave = Vec::new();
            let head = vec![0.0, 1.0];
            let tail = [0.0, -1.0];
            extend(&mut wave, &head);
            extend(&mut wave, &tail);

            assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

            // extend(&mut wave, &wave); // cannot borrow `wave` as immutable because it is also borrowed as mutable immutable borrow occurs here
            // 共有アクセスは読み出しのみのアクセスになる
            // 可変アクセスは排他アクセスになる
            {
                let mut x = 10;
                let r1 = &x;
                let r2 = &x;
                x += 10;

                let m = &mut x;
                // println!("{}, {}, {}", r1, r2, m);

                let mut y = 20;
                let m1 = &mut y;
                let m2 = &mut y;
                let z = y;
                // println!("{} {} {}", m1, m2, z);

                let mut w = (107, 109);
                let r = &w;
                let r0 = &r.0;
                // let m1 = &mut r.1;
                println!("{}", r0);

                let mut v = (136, 139);
                let m = &mut v;
                let m0 = &mut m.0;
                *m0 = 137;

                let r1 = &m.1;

                // v.1;

                println!("{}", r1);
            }
        }
    }
}
