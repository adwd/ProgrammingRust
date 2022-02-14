#![feature(trace_macros)]

fn main() {
    // マクロ
    // マクロは関数にはできない機能がいくつかある
    // ファイル名と行番号を含んだエラーメッセージを出す
    // コンパイルの際に型チェックより先にマクロ呼び出しは展開される
    // より強力な手続きマクロというのもある
    {
        //
        macro_rules! assert_eq2 {
            // パターン
            {$left:expr, $right:expr} => {
                // テンプレート
                match (&$left, &$right) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            panic!(
                                "assertion failed: `(left == right)`\
                                    (left: `{:?}`, right `{:?}`)",
                                left_val, right_val
                            )
                        }
                    }
                }
            };
        }
        // macro_rules!で定義する
        // パターンマッチのみで動作し、いくつかのパターンとそのテンプレートを並べることもできる
        // パターンやテンプレートの周りの括弧、およびマクロを呼び出す際のカッコはどれでも良い
        assert_eq2!(4, 2 * 2);
        assert_eq2![4, 2 * 2];
        assert_eq2! {4, 2 * 2}

        // マクロ展開の基礎
        // $left:exprというフラグメント
        // exprは式が来ることを期待している
        // フラグメント型は出力テンプレートに書かない
        // Webプログラミングでのテンプレート言語とそれほど変わらないが違いは出力がRustのコードということ

        // フラグメントはコードに置き換えられるので何度も実行するのはまずい
        // 参照を借用するのも、マクロで値が消費されないようにするため

        // 繰り返し

        // 上のパターンから順にマッチを試みる
        macro_rules! vec2 {
            ($elem:expr ; $n:expr) => {
                ::std::vec::from_elem($elem, $n)
            };
            ( $( $x: expr ),* ) => {
                <[_]>::into_vec(Box::new([ $( $x ),* ]))
            };
            ( $( $x:expr ),+ ,) => {
                vec2![ $( $x ),* ]
            };
        }

        let buffer = vec2![0_u8; 10];
        let numbers = vec2![1, 2, 3, 4, 5];

        assert_eq2!(buffer, vec![0_u8; 10]);
        assert_eq2!(numbers, vec![1, 2, 3, 4, 5]);

        // $( ... )*  0個以上にマッチ、セパレータなし
        // $( ... ),* 0個以上にマッチ、セパレータはカンマ
        // $( ... );* 0個以上にマッチ、セパレータはセミコロン
        // $( ... )+  1個以上にマッチ、セパレータはなし
        // $( ... ),+ 1個以上にマッチ、セパレータはカンマ
        // $( ... );+ 1個以上にマッチ、セパレータはセミコロン
        // $( ... )?  0個または1個にマッチ、セパレータはなし
        // $( ... ),? 0個または1個にマッチ、セパレータはカンマ
        // $( ... );? 0個または1個にマッチ、セパレータはセミコロン
    }

    // 組み込みマクロ
    {
        // file!(), line!(), column!()
        // stringify!(...tokens...), concat!(str0, str1, ...), cfg!(...)
        // env!("VAR_NAME"), option_env!("VAR_NAME")
        // include!("file.rs"), include_str!("file.txt"), include_bytes!("file.dat")
        // todo!(), unimplemented!()
        // matches!(value, pattern)
    }

    // マクロのデバッグ
    {
        macro_rules! vec2 {
            ($elem:expr ; $n:expr) => {
                ::std::vec::from_elem($elem, $n)
            };
            ( $( $x: expr ),* ) => {
                <[_]>::into_vec(Box::new([ $( $x ),* ]))
            };
            ( $( $x:expr ),+ ,) => {
                vec2![ $( $x ),* ]
            };
        }

        // -Z unstable-options --pretty expanded
        // log_syntax!() #![feature(log_syntax)]
        // trace_macros!(true);

        trace_macros!(true);
        let numbers = vec2![1, 2, 3];
        trace_macros!(false);
        println!("total: {}", numbers.iter().sum::<i32>());
        /*
        ❯ rustc src/main.rs
        note: trace_macro
        --> src/main.rs:105:23
            |
        105 |         let numbers = vec2![1,2,3];
            |                       ^^^^^^^^^^^^
            |
            = note: expanding `vec2! { 1, 2, 3 }`
            = note: to `< [_] > :: into_vec(Box :: new([1, 2, 3]))` */
    }

    // json!マクロの構築
    {
        macro_rules! json1 {
            (null) => {
                Json::Null
            };
        }

        assert_eq!(json1!(null), Json::Null);
        // フラグメント型
        // expr, stmt, ty, path, pat, item, block, meta, ident, literal, lifetime, vis, tt
        // exprなどはRustコードとしてなので、JSONには使いにくい。
        // tt(トークンツリー)は (...), [...], {...}などのカッコで囲まれている部分すべてとカッコのない単一のトークンにマッチする
        // JSON値はすべて1つのトークンツリーとなる。null,文字列、数値などの単一トークンとオブジェクトと配列すべてがttとなる

        #[macro_export]
        macro_rules! json {
            (null) => {
                $crate::Json::Null
            };
            ([ $( $element:tt ),* ]) => {
                $crate::Json::Array(vec![ $( json!($element) ),* ])
            };
            ({ $( $key:tt : $value:tt ),* }) => {
                {
                    let mut fields = $crate::macros::Box::new(
                        $crate::macros::HashMap::new());
                    $(
                        fields.insert($crate::macros::ToString::to_string($key),
                                    json!($value));
                    )*
                    $crate::Json::Object(fields)
                }
            };
            ($other:tt) => {
                $crate::Json::from($other)
            };
        }

        impl From<bool> for Json {
            fn from(b: bool) -> Json {
                Json::Boolean(b)
            }
        }

        impl From<String> for Json {
            fn from(s: String) -> Json {
                Json::String(s)
            }
        }

        impl<'a> From<&'a str> for Json {
            fn from(s: &'a str) -> Json {
                Json::String(s.to_string())
            }
        }

        macro_rules! impl_from_num_for_json {
            ( $( $t:ident )* ) => {
                $(
                    impl From<$t> for Json {
                        fn from(n: $t) -> Json {
                            Json::Number(n as f64)
                        }
                    }
                )*
            }
        }
        impl_from_num_for_json!(u8 i8 u16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

        let width = 4.0;
        let desc = json!({
            "width": width,
            "height": (width * 9.0 / 4.0),
        });

        // proc macro
        // https://danielkeep.github.io/tlborm/book/index.html
    }
}

use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}
