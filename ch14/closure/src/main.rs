fn main() {
    {
        let mut v = vec![1, 2, 3];
        v.sort_by_key(|i| -i);
        // | arg | { ... } がクロージャ
    }
}
