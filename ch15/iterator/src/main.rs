use std::collections::HashMap;

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

        // inspect: RxJSのtapみたいなの
        // chain
        let v: Vec<i32> = (1..4).chain([10, 20, 30]).collect();

        // enumerate: zipWithIndex的な
        // zip
        // by_ref: イテレータに対する可変参照を借用する
        let message = "To: jimb\n\
                            From: id\n\
                            \n\
                            Hello, world!";
        let mut lines = message.lines();
        println!("Headers:");
        for header in lines.by_ref().take_while(|l| !l.is_empty()) {
            println!("{}", header);
        }

        println!("\nBody:");
        for body in lines {
            println!("{body}")
        }

        // cloned, copied
        // cycle
        use std::iter::{once, repeat};
        let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
        let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
        let fizzes_buzzes = fizzes.zip(buzzes);

        let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
            (i, ("", "")) => i.to_string(),
            (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
        });

        for line in fizz_buzz {
            println!("{line}");
        }
    }

    // イテレータの消費
    {
        // count, sum, product
        // max, min, max_by, min_by
        // max_by_key, min_by_key
        // any, all
        // position, rposition, ExactSizeIterator
        // fold, rfold(DoubleEndedIterator)
        // try_fold, try_rfold
        use std::error::Error;
        use std::io::prelude::*;
        use std::str::FromStr;

        fn m() -> Result<(), Box<dyn Error>> {
            let stdin = std::io::stdin();
            let sum =
                stdin
                    .lock()
                    .lines()
                    .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
                        Ok(sum + u64::from_str(line?.trim())?)
                    })?;
            println!("{}", sum);
            Ok(())
        }

        // nth, nth_back
        // last: イテレータを戦闘から全て消費する必要がないならiter.next_back()
        // find, rfind, find_map
        // collect, FromIterator
        // collectメソッドはコレクション型が実装しているFromIteratorのメソッドを実行しているだけ
        // Extendトレイト
        // partition
        // for_each, try_for_each
        ["doves", "hens", "birds"]
            .iter()
            .zip(["turtle", "french", "calling"])
            .zip(2..5)
            .rev()
            .map(|((item, kind), quantity)| format!("{} {} {}", quantity, kind, item))
            .for_each(|gift| {
                println!("You have received: {gift}");
            });
    }

    // ユーザ定義イテレータの実装
    {
        struct I32Range {
            start: i32,
            end: i32,
        }
        impl Iterator for I32Range {
            type Item = i32;
            fn next(&mut self) -> Option<i32> {
                if self.start >= self.end {
                    return None;
                }
                let result = Some(self.start);
                self.start += 1;
                result
            }
        }

        let mut pi = 0.0;
        let mut numerator = 1.0;

        for k in (I32Range { start: 0, end: 14 }) {
            pi += numerator / (2 * k + 1) as f64;
            numerator /= -3.0;
        }
        pi *= f64::sqrt(12.0);

        assert_eq!(pi as f32, std::f32::consts::PI);

        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>),
        }

        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>,
        }

        use BinaryTree::*;

        struct TreeIter<'a, T: 'a> {
            unvisited: Vec<&'a TreeNode<T>>,
        }

        impl<'a, T: 'a> TreeIter<'a, T> {
            fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
                while let NonEmpty(ref node) = *tree {
                    self.unvisited.push(node);
                    tree = &node.left;
                }
            }
        }

        impl<T> BinaryTree<T> {
            fn iter(&self) -> TreeIter<T> {
                let mut iter = TreeIter {
                    unvisited: Vec::new(),
                };
                iter.push_left_edge(self);
                iter
            }
        }

        impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
            type Item = &'a T;
            type IntoIter = TreeIter<'a, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a, T> Iterator for TreeIter<'a, T> {
            type Item = &'a T;
            fn next(&mut self) -> Option<&'a T> {
                let node = self.unvisited.pop()?;
                self.push_left_edge(&node.right);
                Some(&node.element)
            }
        }

        impl<T: Ord> BinaryTree<T> {
            fn add(&mut self, value: T) {
                match *self {
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
        tree.add("jaeger");
        tree.add("robot");
        tree.add("droid");
        tree.add("mecha");

        let mut v = Vec::new();
        for kind in &tree {
            v.push(*kind);
        }
        assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);
    }
}
