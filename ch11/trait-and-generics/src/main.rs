use std::{collections::HashMap, fmt::Result, fs::File};

fn main() {
    {
        // RustのトレイトとジェネリクスはHaskellの型クラスにヒントを得ている
        // Haskellから拝借した3つの機能、Self型、関連関数、関連型
        // トレイトはRustにおけるインターフェースもしくは抽象基底クラス
        use std::io::Write;

        // &mut dyn Write: Writeトレイトを実装している任意の値への可変参照
        fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
            out.write_all(b"hello world\n")?;
            out.flush()
        }

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        say_hello(&mut handle).unwrap();

        let mut bytes = vec![];
        say_hello(&mut bytes).unwrap();
        assert_eq!(bytes, b"hello world\n");

        // <T: Ord>: Ordトレイトを実装する任意の型T（制約）
        fn min<T: Ord>(v1: T, v2: T) -> T {
            if v1 < v2 {
                v1
            } else {
                v2
            }
        }
    }

    // トレイとの使い方
    // 型がサポートする性質を意味する
    {
        let mut buf: Vec<u8> = vec![];
        // このトレイトがスコープに入らないとwrite_allは使えない
        use std::io::Write;
        buf.write_all(b"hello");

        // CloneやIteratorのメソッドが何もしなくても使えるのは、デフォルトでスコープに入っているから（プレリュード）
        // このようなメソッド呼び出しにオーバヘッドはないが、&mut dyn Writeのような場合は動的ディスパッチになる
        // dyn Writeはトレイトオブジェクトと呼ばれる
    }

    {
        // トレイトオブジェクト
        use std::io::Write;
        let mut buf: Vec<u8> = vec![];
        // dyn Write型の変数を持つことは出来ない。コンパイル時に変数のサイズが決まっていなければいけないため
        // let writer: dyn Write = buf;
        // 参照と明示すれば良い
        let writer: &mut dyn Write = &mut buf;
        // このようなトレイト型への参照をトレイトオブジェクトと呼ぶ
        // 他の参照と同様に何らかの値を指し、生存期間を持ち、可変か共有かのどちらか
        // 他の参照と異なるのはコンパイル時に参照先の実際の型がわからないこと
        // dyn WriteからVec<u8>の実際の型にダウンキャストすることもできない
        // トレイトオブジェクトはファットポインタで、値へのポインタと値の方を表すテーブルへのポインタで構成される
        // C++でもこのような実行時の方情報を保持していて仮想テーブル(virtual table, vtable)と呼ばれる
        // Rustは通常の参照を自動的にトレイとオブジェクトに変換するのでsay_helloに&mut Vec<u8>を渡せる
    }

    // ジェネリック関数と型パラメータ
    {
        use std::io::Write;
        fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
            out.write_all(b"hello world\n")?;
            out.flush()
        }
        // Wから実行時に実際の型のメソッドを呼び出すことを単相化(monomorphization)と呼ぶ
        let mut buf: Vec<u8> = vec![];
        // 型パラメータを明示的に書くこともできるがほとんどやらない
        say_hello::<Vec<u8>>(&mut buf).unwrap();
        // Rustが推論できない場合はあるのでその場合は各必要がある
        let v = (0..100).collect::<Vec<i32>>();

        trait Mapper {}
        trait Serialize {}
        trait Reducer {}
        type DataSet = HashMap<String, String>;
        // 複数の型パラメータを持てる
        fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
            data: &DataSet,
            map: M,
            reduce: R,
        ) -> Result {
            Ok(())
        }

        // whereを使える
        fn run_query2<M, R>(data: &DataSet, map: M, reduce: R) -> Result
        where
            M: Mapper + Serialize,
            R: Reducer + Serialize,
        {
            Ok(())
        }
        // where節はジェネリック関数だけでなく、構造体、列挙型、型エイリアス、メソッドなど型制約を書けるすべての構文で使用できる
        // ジェネリック関数は型パラメータに加えて生存期間パラメータも持てる
        // さらに定数パラメータも持てる
    }

    // トレイトオブジェクトとジェネリック関数どちらをつかうか
    {
        trait Vegetable {}
        // これは1種類のVegetableしか取れない
        struct Salad<V: Vegetable> {
            veggies: Vec<V>,
        }
        struct Salad2 {
            veggies: Vec<Box<dyn Vegetable>>,
        }
        // Rustはジェネリック関数を使用した型ごとにコンパイルするのでコードサイズの面でトレイトオブジェクトが優れているかもしれない
        // 2つの点でジェネリクスのほうがトレイトオブジェクトより優れている
        // 1. スピード、ジェネリクスでは動的ディスパッチは発生しないし、型を見て最適化を掛けられる
        // 2. トレイとの型関連関数などはトレイトオブジェクトでは使えない
        // 3. 複数のトレイトを用いた型制約を指定するのが容易 T: Debug + Hash + Eq, &mut (dyn Debyg + Hash + Eq)はできない
    }

    // トレイトの定義と実装
    {
        type Canvas = Vec<u8>;
        trait Visible {
            fn draw(&self, canvas: &mut Canvas);
            fn hit_test(&self, x: i32, y: i32) -> bool;
        }

        struct Broom {}
        impl Visible for Broom {
            fn draw(&self, canvas: &mut Canvas) {
                canvas.push(b'B');
            }

            fn hit_test(&self, x: i32, y: i32) -> bool {
                x == 0 && y == 0
            }
        }
    }

    // デフォルトメソッド
    {
        use std::io::{Result, Write};
        struct Sink;
        impl Write for Sink {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                Ok(buf.len())
            }

            fn flush(&mut self) -> Result<()> {
                Ok(())
            }

            // write_allを書かなくてもデフォルト実装があるのでそちらが使われる
        }
    }

    // 任意の型に任意のトレイトを実装でき、拡張トレイトと呼ぶ
    // Serdeは基本的な型にSerializeトレイトを定義している
    {
        use serde::Serialize;
        fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
            let writer = File::create("foo.txt")?;
            let mut serializer = serde_json::Serializer::new(writer);

            // HashMapにserializeが生えてる
            config.serialize(&mut serializer)?;
            Ok(())
        }
    }

    // サブトレイト
    {
        type Canvas = Vec<u8>;
        trait Visible {
            fn draw(&self, canvas: &mut Canvas);
            fn hit_test(&self, x: i32, y: i32) -> bool;
        }

        type Direction = (i32, i32, i32, i32);
        trait Creature: Visible {
            fn position(&self) -> (i32, i32);
            fn facing(&self) -> Direction;
        }
        // CreatureはVisibleのサブトレイトであり、VisibleはCreatureのスーパートレイトとなる
        // Selfへの制約の省略記法に過ぎない
        trait Creature2
        where
            Self: Visible,
        {
        }
    }

    // 型関連関数
    {
        trait StringSet {
            fn new() -> Self;
        }
        fn create_string_set<S: StringSet>() -> S {
            S::new()
        }

        trait SizedSet {
            fn new() -> Self
            where
                Self: Sized;
        }
        // 関連関数にSizedの制約を与えることでトレイトオブジェクトを作れるようになる
    }

    // 完全修飾メソッド呼び出し
    {
        let _ = "hello".to_string();
        let _ = str::to_string("hello");
        let _ = ToString::to_string("hello");
        let _ = <str as ToString>::to_string("hello");
        // 型が推論できない場合や古い実装と新しいのがある場合完全修飾メソッド呼び出しを使う
    }

    // 型と型の関係を定義するトレイト
    {
        trait Iterator2 {
            type Item; // 関連型(associated type)

            fn next(&mut self) -> Option<Self::Item>;
        }

        fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
            let mut results = Vec::new();
            for item in iter {
                results.push(item);
            }
            results
        }

        fn dump<I>(iter: I)
        where
            I: Iterator,
            I::Item: std::fmt::Debug,
        {
            for (index, value) in iter.enumerate() {
                println!("{}: {:?}", index, value);
            }
        }

        fn dump2(iter: &mut dyn Iterator<Item = String>) {
            for (index, value) in iter.enumerate() {
                println!("{}: {:?}", index, value);
            }
        }

        // イテレータ以外にも、Taskトレイトは関連するOutput型がある
    }
}
