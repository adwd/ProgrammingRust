// モジュールはアイテムの集合体
// アイテム: 構造体、関数
pub mod spores2 {

    pub struct Spore {}

    // pub
    // モジュールの外からアクセスできる
    pub fn produce_spore() -> Spore {
        Spore {}
    }

    // pub(crate)
    // crateの内部からはアクセスできるが外部には公開されず、クレートのドキュメントにも表れない
    pub(crate) fn genes() -> Vec<Spore> {
        vec![]
    }

    // pubがないのでこのモジュールと子モジュールからしかアクセスできない
    fn recombine() {}

    // ネストしたモジュール
    pub mod roots {

        // pub(super) 親モジュールからのみアクセス可能になる
        pub(super) fn roots() {}

        pub mod products {
            // in <path>で特定の親モジュールとその子孫からのみアクセスできるようにする
            pub(in crate::plant_structures::spores) struct Cytokinin {}
        }
    }
}
