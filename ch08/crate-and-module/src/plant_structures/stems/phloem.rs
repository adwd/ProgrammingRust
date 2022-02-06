// ドキュメントコメント内にテストをかける
// #から始めるとドキュメントには含まれなくなる

/// fn
/// ```
/// # use fern_sim::plant_structures::stems::phloem;
/// assert_eq!(phloem::phloem(), 1);
/// ```
///
/// ```no_run
/// // no_runとつけるとテスト実行されない
/// loop {
///     println!("Hello, world");
/// }
/// ```
///
/// ```ignore
/// // ignoreとつけるとコンパイルもされない
/// fn m( {}'
/// ```
pub fn phloem() -> i32 {
    1
}
