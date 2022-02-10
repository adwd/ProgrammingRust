use std::collections::*;

fn main() {
    // Rustと他の言語のコレクションの違い
    // 1. 移動と借用があらゆるところで起こる
    // 2. 無効化によるエラーが生じない
    // 3. nullがない
    let _collections = (
        Vec::<i32>::new(),
        VecDeque::<i32>::new(),      // 両端から出し入れ可能なキュー
        LinkedList::<i32>::new(),    // 二重リンクリスト
        BinaryHeap::<i32>::new(),    // マックスヒープ
        HashMap::<i32, i32>::new(),  // キーバリューハッシュテーブル
        BTreeMap::<i32, i32>::new(), // ソートされたキーバリューテーブル
        HashSet::<i32>::new(),       // ソートされないハッシュベースのセット
        BTreeSet::<i32>::new(),      // ソートされたセット
    );

    // Vec<T>
    {
        // 長さ、容量、要素の格納するヒープ上に確保したバッファへのポインタからなる
        // slice. first, last, get, first_mut, last_mut, get_mut
        // to_vec: スライス全体を複製し、新しいベクタを返す
        // slice. len, is_empty
        // capacity, reserve, reserve_exact, shrink_to_fit
        // push, pop, insert(index, value), remove(index)
        // resize(new_len, value), resize_with(new_len, closure), truncate(new_len), clear
        // extend(terable), split_off(index), append(&mut vec2), drain(range)
        // retain(test), dedup, dedup_by(same), dedup_by_key(key)
        // concat, join

        // スライスなどを分割して可変参照を得るメソッドがある
        // iter, iter_mut
        // split_at, split_at_mut
        // split_first, split_first_mut
        // split_last, split_last_mut
        // split(is_sep), split_mut(is_sep)
        // split_inclusive(is_sep), split_inclusive_mut(is_sep)
        // rsplit, splitn, rsplitn, chunks, rchunks, chunks_exact, windows

        // swap(i, j) swap(&mut slice_b), swap_remoe(i)
        // fill, fill_with
        // sort, sort_by, sort_by_key
        // reverse
        // binary_search, binary_search_by, binary_search_by_key
        // contains

        // starts_with, ends_with

        // rng.choose(slice), rng.shuffle(slice)

        // Rustでは無効化エラーは生じない
    }

    // VecDeque<T>
    {
        // 末尾だけでなく先頭を追加、削除するならVecよりもVecDequeが良い
        // push_front, push_back, pop_front, pop_back
        // front, back (mut)
        // make_contiguous, Vec::from(deque), VecDeque::from(vec)
    }

    // BinaryHeap<T>
    {
        // 最大値が常にキューの先頭に浮き上がってくるような緩やかに構造化されたコレクション
        // push, pop, peek, peek_mut
        // 注意: BinaryHeapはイテレート可能だが、大きい順ではない。
        // その場合はwhileループでpopしていく
    }

    // HashMap<K,V>, BTreeMap<K,V>
    {
        // contains_key
        // get, get_mut
        // insert, extend, append, remove, remove_entry
        // into_iter, into_keys, into_values
        // retain, clear
        // map[&key]
        // btree_map.split_at(&key)

        // エントリ
        let mut student_map = HashMap::<String, String>::new();
        if !student_map.contains_key("bob") {
            student_map.insert("bob".to_string(), "Bob".to_string());
        }
        let record = student_map.get_mut("bob").unwrap();
        // これだとマップを何度もアクセスして非効率
        // 検索をEntryの値として生成し、それに対して処理を行うことで検索の回数を減らす
        let record = student_map
            .entry("bob".to_string())
            .or_insert_with(|| "Bob".to_string());

        // map.entry
        // entry.or_insert(value), or_default(), or_insert_with(fn)
        // and_modify(closure)
        let text = "hello world";
        let mut word_frequency = HashMap::<String, u32>::new();
        for c in text.split_whitespace() {
            word_frequency
                .entry(c.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    // HashSet<T>, BTreeSet<T>
    {
        // セットは値のないマップのようなもの
        // set.intersection(&set2)
        // &set1 & &set2 と同じ
        // union, difference, symmetric_difference
        // disjoint, is_subset, is_superset
    }

    // ハッシュ
    {
        // ある値に対して参照は参照先と同じハッシュ値を返さなければならない

        #[derive(Clone, PartialEq, Eq, Hash)]
        enum MuseumNumber {}
        #[derive(Clone, PartialEq, Eq, Hash)]
        enum Curture {}
        #[derive(Clone, PartialEq, Eq, Hash)]
        enum RoughTime {}
        struct Artifact {
            id: MuseumNumber,
            name: String,
            cultures: Vec<Curture>,
            date: RoughTime,
        }

        impl PartialEq for Artifact {
            fn eq(&self, other: &Artifact) -> bool {
                self.id == other.id
            }
        }

        impl Eq for Artifact {}

        use std::hash::{Hash, Hasher};
        impl Hash for Artifact {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                // こうしないとHashSet<Artifact>が正しく動作しない。ハッシュテーブルでは常にa == bならばhash(a) == hash(b)でなければならない
                self.id.hash(hasher)
            }
        }
        let _collection = HashSet::<Artifact>::new();
    }

    // ハッシュアルゴリズムのカスタマイズ
    {
        // RustのデフォルトハッシュアルゴリズムはSipHash-1-3
        // これほど高度なアルゴリズムが必要じゃない場合は高速なものに差し替えられる
        // fnv = "1.0"
        // use fnv::{FnvHashMap, FnvHashSet};
    }

    // 標準コレクションを超えて
    {
        //
    }
}
