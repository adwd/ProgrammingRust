use fern_sim::Fern;

#[test]
fn test_fiddlehead_unfurling() {
    let _f = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };
}

// 結合テストはクレート外部のユーザの世界から見える公開APIをテストする
