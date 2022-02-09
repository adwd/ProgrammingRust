use std::{collections::HashMap, io::BufRead, string};

fn main() {
    // イテレータは値の列を生成する値
    fn triangle(n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..=n {
            sum += i;
        }
        sum
    }
    // 式 1..=n はRangeInclusive<i32>の値でイテレータ

    // foldメソッド
    fn t2(n: i32) -> i32 {
        (1..=n).fold(0, |sum, item| sum + item)
    }
    fn t3(n: i32) -> i32 {
        (1..=n).sum()
    }

    // IteratorトレイトとIntoIteratorトレイト
    {
        trait Iterator2 {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
            // many default methods
        }
        // ある型に対して自然にスキャンする方法があるならIntoIteratorトレイトを実装できる
        trait IntoIterator2
        where
            Self::IntoIter: Iterator<Item = Self::Item>,
        {
            type Item;
            type IntoIter: Iterator;
            fn into_iter(self) -> Self::IntoIter;
        }
    }

    // イテレータの作成
    {
        // iter, iter_mutメソッド
        let v = vec![1, 2, 3, 4];
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));

        // IntoIter
        // ほとんどのコレクションは複数のIntoIteratorを実装していて、
        // 共有参照、可変参照を返すもの、そして値を返すものはそこでコレクション自体もドロップされる
        // for el in  &collection { .. }
        // for el in &mut collection { .. }
        // for el in collection { .. }

        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        // from_fnとsuccessors
        use rand::random;
        use std::iter::from_fn;

        let length = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
            .take(1000)
            .collect::<Vec<f64>>();

        use num::Complex;
        use std::iter::successors;

        fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
            let zero = Complex { re: 0.0, im: 0.0 };
            successors(Some(zero), |&z| Some(z * z * c))
                .take(limit)
                .enumerate()
                .find(|&(_i, z)| z.norm_sqr() > 4.0)
                .map(|(i, _z)| i)
        }

        // from_fnとsuccessorsはFnMutクロージャも受け付けるので周辺の変数をキャプチャして使える

        fn fibonacci() -> impl Iterator<Item = usize> {
            let mut state = (0, 1);
            std::iter::from_fn(move || {
                state = (state.1, state.0 + state.1);
                Some(state.0)
            })
        }
        // from_fnとsuccessorでいろんな事ができるが、後述のイテレータのメソッドを使って名前から読みやすくするほうがいい

        // drainメソッド
        // コレクションへの可変参照を引数として取り、値の所有者を消費者に引き渡すイテレータを返す
        let mut outer = "Earth".to_string();
        let inner = String::from_iter(outer.drain(1..4));

        assert_eq!(outer, "Eh");
        assert_eq!(inner, "art");

        fn vec() -> Vec<i32> {
            vec![1, 2, 3, 4, 5]
        }
        fn string() -> String {
            "hello".to_string()
        }
        fn map() -> HashMap<String, String> {
            HashMap::new()
        }

        // 他のイテレータの生成方法
        let _iterators = (
            1..10,
            (1..10).step_by(2),
            1..,
            1..=10,
            Some(10).iter(),
            Ok::<&str, ()>("ok").iter(),
            vec().windows(16),
            vec().chunks(16),
            vec().chunks_mut(10),
            vec().split(|i| i % 3 != 0),
            vec().split_mut(|i| i % 3 != 0),
            vec().rsplit(|i| i % 3 != 0),
            vec().splitn(3, |i| i % 3 != 0),
            string().bytes(),
            string().chars(),
            string().split_whitespace(),
            string().lines(),
            string().split('/'),
            string().matches(char::is_numeric),
            map().keys(),
            map().values(),
            map().values_mut(),
            // たくさん
        );
    }

    // イテレータアダプタ
    {
        // Iteratorトレイト外提供する様々なアダプタメソッドを利用できる
        // アダプタは1つのイテレータを消費し別のイテレータを作る
        // 代表的なのはmapとfilter
        // イテレータアダプタについて2つ重要な点がある
        // アダプタをあるイテレータに呼び出すだけではアイテムを全く消費しない
        // nextやcollectを呼び出すまで何かは起こらない

        // iterators are lazy and do nothing unless consumed
        ["hello", "world"].iter().map(|el| println!("{el}"));

        // もう一つの重要な点はイテレータアダプタはオーバーヘッドのない抽象化でfor文と書くのと同程度に効率的

        // filter_map, flat_map
        use std::str::FromStr;
        let text = "1\nfront .25  289\n3.1415 estuarty\n";
        for number in text
            .split_whitespace()
            .filter_map(|w| f64::from_str(w).ok())
        {
            println!("{:4.2}", number.sqrt());
        }

        // flatten
        let v = vec![None, Some("day"), None, Some("one")]
            .iter()
            .flatten()
            .collect::<Vec<_>>();

        let v2 = vec![None, Some("day"), None, Some("one")]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(v2, vec!["day", "one"]);

        fn to_uppercase(s: &str) -> String {
            s.chars().flat_map(char::to_uppercase).collect()
        }

        // take, take_while, skip, skip_while

        // peekable
        use std::iter::Peekable;

        fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
        where
            I: Iterator<Item = char>,
        {
            let mut n = 0;
            loop {
                match tokens.peek() {
                    Some(r) if r.is_digit(10) => {
                        n = n * 10 + r.to_digit(10).unwrap();
                    }
                    _ => return n,
                }
                tokens.next();
            }
        }

        // fuse
        // 一度Noneを返したIteratorで再度nextメソッドを呼び出した場合の動作を規定していない
        // ほとんどはNoneをかえすが、そうでないものもある。fuseアダプタは一度Noneを返したら常にNoneを返すようにする

        // 反転可能イテレータとrev
        // DoubleEndedIteratorトレイトを実装したイテレータを反転させる

        // inspect
    }
}
