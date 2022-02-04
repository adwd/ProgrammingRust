fn main() {
    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s;
        // let u = s; // use of moved value: `s` value used here after move

        let u = t.clone();
        let v = t.clone();
    }

    {
        let mut s = "Govinda".to_string();
        s = "Siddhartha".to_string(); // value "Govinda" dropped here
    }
    {
        let mut s = "Govinda".to_string();
        let t = s;
        s = "Siddhartha".to_string(); // nothing is dropped here
    }

    {
        struct Person {
            name: String,
            birth: i32,
        }

        let mut composers = Vec::new();
        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });
    }
    {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }
        // let thrid = v[2]; // cannot move out of index of `std::vec::Vec<std::string::String>` move occurs because value has type `std::string::String`, which does not implement the `Copy` trait
        // let fifth = v[4]; // cannot move out of index of `std::vec::Vec<std::string::String>` move occurs because value has type `std::string::String`, which does not implement the `Copy` trait
        {
            let fifth = v.pop().expect("vector empty!");
            let second = v.swap_remove(1);
            let third = std::mem::replace(&mut v[2], "substitute".to_string());
        }

        {
            let v = vec!["oi".to_string(), "ei".to_string()];

            for mut s in v {
                s.push('!');
                println!("{}", s);
            }
        }
    }
    {
        struct Person {
            name: Option<String>,
            birth: i32,
        }
        let mut composers = Vec::new();
        composers.push(Person {
            name: Some("Palestrina".to_string()),
            birth: 1525,
        });
        // let first_name = composers[0].name; // cannot move out of index of `std::vec::Vec<main::Person>` move occurs because value has type `std::option::Option<std::string::String>`, which does not implement the `Copy` trait
        {
            let first_name = std::mem::replace(&mut composers[0].name, None);
        }
        {
            let first_name = composers[0].name.take();
        }
    }
    {
        // Copy型
        let num1 = 36_i32;
        let num2 = num1;
        let num3 = num1; // Copy型なのでnum1からnum2にmoveしていないのでnum1は未初期化状態にならない

        // 整数、浮動小数点数、char、bool
        // Copy型のタプル、固定長の配列もCopy型
        let t1 = (1, 2);
        let t2 = t1;
        let t3 = t1;

        let a1 = [1, 2, 3];
        let a2 = a1;
        let a3 = a1;

        // Stringはヒープ上にバッファを持つのでCopy型ではない
        // 値をドロップする際になにか特別なことをしなければならない型はCopy型ではない
        // Vecは要素を開放する必要がある、Fileはファイルのクローズ、MutexGuardは排他ロックを解放する

        // ユーザが定義したstruct, enum型はCopyではない
        // すべてのフィールドがCopy型であれば#[derive(Clone, Copy)]でCopy型になる
        #[derive(Clone, Copy)]
        struct Label {
            number: u32,
        }

        // #[derive(Clone, Copy)] // the trait `Copy` may not be implemented for this type
        // struct StringLabel {
        // name: String,
        // }
    }

    {
        // 参照カウントのポインタ型Rc, Arc
        // Arcは複数のスレッド間で直接共有しても安全なのが違い: Arc(Atomic reference count)
        // スレッド間でやり取りしないならより高速なRcを使う
        use std::rc::Rc;

        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t: Rc<String> = s.clone();
        let u: Rc<String> = s.clone();
        // Rc<T>はヒープ上に確保されたTとそれに付随する参照カウントを指すポインタとなる
        // Rc<T>をcloneするとTの値はコピーされず、同じものを指すポインタが作られ、参照カウントがインクリメントされる

        println!("{}", u.contains("shi"));

        // cannot borrow data in an `Rc` as mutable trait `DerefMut` is required to modify through a dereference, but it is not implemented for `std::rc::Rc<std::string::String>`
        // t.push_str(" noodles");
    }
}
