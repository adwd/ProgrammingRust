use std::num::ParseIntError;

use fern_sim::*;

pub mod plant_structures;

fn main() {
    // エディション
    // async, awaitがキーワードになるなど破壊的変更が入る場合はエディションが上がる
    // エディションは依存するクレートと合わせる必要はない

    // ビルドプロファイル
    // cargo build [profile.dev]
    // cargo build --release [profile.release]
    // cargo test [profile.test]

    // プロファイラを使いたい場合は特殊で、releaseビルドでしか有効にならない最適化を行いつつ
    // デバッグビルドでしか有効にならないデバッグシンボルがつくようにしなければならない
    // [profile.release]
    // debug = true # enable debug symboles in release builds

    // モジュール

    // モジュールを構成するには3つの方法がある
    // 1. 1つのファイルに収める方法
    // 2. ディレクトリにしてmod.rsを使う方法
    // 3. ファイルに本体を書き、補足ディレクトリにサブもージュールを構成するファイルを置く方法

    // パスとインポート
    {
        // 絶対パス
        let _s = crate::plant_structures::spores::spores2::Spore {};
        let _max = std::f64::MAX;
    }
    {
        // import
        use std::f64;
        let _max = f64::MAX;
        // use std::f64::MAX;まで書かないのが良いスタイルとされている

        use std::collections::{HashMap, HashSet};
        use std::fs::{self, File}; // std::fs, std::fs::File
        use std::io::prelude::*; // import everything

        use std::io::Result as IOResult;

        // モジュールは親モジュールの名前空間を自動で引き継がない
        mod proteins {
            pub enum AminoAcid {
                A,
            }

            mod synthesis {
                use super::*;
                pub fn synthesize_amid_acid() -> AminoAcid {
                    AminoAcid::A
                }
            }
        }
        // super: 親モジュール
        // crate: 現在のモジュールを含むクレート

        // :: 常に外部クレートを指す絶対パス
        let _m = ::std::f64::MAX;

        // Unixのファイルシステムとちょっと似ている
        // use -> ln, self -> ., super -> ..

        // 標準のプレリュード
        // stdは自動的にリンクされる
        // VecやResultなどが使えるように、 use std::preluce::v1::*; みたいになってる状態
        use std::prelude::rust_2021::*;

        let _v = Vec::<i32>::new();

        // use宣言をパブリックにする
        pub use crate::plant_structures::spores;

        // staticと定数
        // モジュールには関数、型、ネストしたモジュールに加えて定数とstaticを定義できる
        // 定数にはconstを使う。pubをつけられることと型指定が必須な点がletと異なる
        // 定数は慣習としてUPPER_CASEで書く
        pub mod nums {
            pub const ROOM_TEMPERATURE: f64 = 20.0;
            pub static ROOM_TEMPARETURE: f64 = 20.0;
        }
        // 定数はコンパイル時に値が使われるところに埋め込まれる
        // staticは実行から終了まで生き残る変数
        // マジックナンバーなどには定数を使い、データが大きいなどでその定数値への参照を借用する必要があるならstaticを使う
        // staticはmutがつけられるが、本質的に安全ではないので普通には使えない(19章)

        // 属性
        // アイテムに属性をつけることができる
        #[allow(non_camel_case_types)]
        pub struct git_revspec {}

        #[cfg(test)]
        #[cfg(debug_assertions)]
        #[cfg(unix)]
        #[cfg(windows)]
        #[cfg(target_pointer_width = "64")]
        #[cfg(target_arch = "x86_64")]
        #[cfg(target_os = "macos")]
        #[cfg(feature = "robots")]
        #[cfg(not(feature = "machines"))]
        #[cfg(all(test, unix))]
        #[cfg(any(test, unix))]
        struct _T {}

        #[inline]
        fn _f() {}
        // #[inline]をつけないとインライン展開が行われない場合がある
        #[inline(always)]
        fn _f2() {}
        #[inline(never)]
        fn _f3() {}

        // 属性をクレート是t内に付与するにはmain.rs, lib.rsの戦闘に書く。その場合#ではなく#!を用いる
        // feature属性は#![feature(trace_macros)]のように不安定な機能を使用するのに使う

        // /// はドキュメントコメント, #[doc = ""]と同じ
        /// doc comment
        fn _foo() {}

        #[doc = "doc comment"]
        fn _foo2() {}

        // //! は #doc![] 属性として扱われ、一般にそのモジュールやクレートに対して付与される

        // ドキュメントコメントの内容はMarkdown形式として扱われる
        // リンクにRustのアイテムパスを使える

        /// link to [`spores`](plant_structures::spores)
        fn _foo3() {}

        #[doc = include_str!("../Cargo.toml")]
        fn _foo4() {}

        // ドキュメントコメント内に書いたコードは実行され動作チェックされる
        /// foo5
        ///
        /// ```
        /// if foo {
        ///     _foo5();
        /// }
        /// ```
        fn _foo5() {}

        fern_sub::fern_sub();
    }
}

// #[test]属性
// cargo test
// cargo test math は名前にmathを含むテストが実行される
// assert! や assert_eq!マクロでテストを書く
// これらはテストコード以外でも使うことができるが、プロダクションビルドにも含まれる
// debug_assert!, debug_assert_eq!はデバッグビルドのみ
// パニックすることをテストするときは#[should_panic]属性をつける
#[test]
fn math_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
#[allow(unconditional_panic)]
fn devide_by_zero() {
    1 / 0;
}

// テストはResult<(), E>を返しても良い
#[test]
fn explicit_radix() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}

// テストのときだけ使うコードを#[cfg(test)]属性を設定したモジュールに入れるのが慣例になっている
#[cfg(test)]
mod tests {
    fn helper() {}

    #[test]
    fn test_helper() {
        helper();
        assert_eq!(2 + 2, 4);
    }
}
