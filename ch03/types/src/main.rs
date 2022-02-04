use regex::Regex;

fn main() {
    println!("Hello, world!");

    // 可変な参照
    {
        let mut i = 32;
        {
            let j: &mut i32 = &mut i;
            *j += 1;
        }

        let mut arr: [f64; 5] = [0.0; 5];
        {
            let arr2: &mut [f64; 5] = &mut arr;
            arr2[0] = 1.0;
        }
    }

    // 数値型
    {
        let _t1 = (
            1u8, 2u16, 3u32, 4u64, 5u128, -1i8, -2i16, -3i32, -4i64, -5i128,
        );
        let t2: (usize, isize) = (1, -1);

        // 配列のインデックスはusize
        let arr = [1, 2, 3, 4, 5];
        println!("{}", arr[t2.0]);

        let _t3 = (0x19, 0o10, 0b10, 100_000, 0xff_ffff, 127_u8);

        let _t4 = (b'X', b'\x1b');

        let _t5 = (1_u16.pow(4), (-4_i32).abs(), 0b101101_u8.count_ones());

        // すべての整数型にabsメソッドは定義されているが、どの整数型かを指定する必要がある
        // can't call method `abs` on ambiguous numeric type `{integer}`
        // println!("{}", (-4).abs());
        println!("{}", i32::abs(-4));
        // 単項前置演算子よりもメソッド呼び出しの優先順位が高いのでカッコで括る
        println!("{}", (-4_i32).abs());
    }

    {
        // デバッグビルドでは整数演算中のオーバーフローは検出され、パニックを発生させる
        // リリースビルドではラップする（値の範囲で除算した余りを返す）

        let _t1 = (
            10_i32.checked_mul(10),
            10_u8.checked_add(20),
            100_u16.wrapping_mul(100),
            10_i16.wrapping_shl(10),
            32760_i16.saturating_add(100),
            255_u8.overflowing_add(2),
        );
    }

    {
        // 浮動小数点
        let _t1 = (0.1_f32, 0.1_f64, 5., 123.456e-12_f64);

        let _t2 = (f32::INFINITY, f32::MAX);

        let _t3 = (5f32.sqrt(), (-22f64).floor());

        let _t4 = (std::f64::consts::PI, f32::EPSILON);
    }

    {
        // 文字
        let _t1 = ('x', '\'', '\\', '\n', '\r', '\t', '\u{1009ab}');
    }

    {
        // ポインタ型

        // 参照
        let str = "hello".to_string();
        let _p1: &String = &str; // strへの参照を借用する

        // &T
        // 変更不能な共有参照。複数の共有参照を持てるが、読み出すことしか出来ない

        // &mut T
        // 排他的な可変参照。この参照が存在する間は他の参照は不変であれ可変であれ存在できない

        let _b = Box::new(str);
        // Box
        // ヒープ上に値を確保する

        // rawポインタ
        // *mut T, *const T
        // Rsutが管理していないので安全ではなく、unsafeブロックの名kでしか出来ない

        // 配列、ベクタ、スライス
        let _arr: [i32; 2] = [1, 2];
        // [T; N] は固定長の配列でコンパイル時に定まる

        let _vec: Vec<i32> = vec![1, 2, 3];
        // Vec<T> は動的に確保される可変長の配列でヒープ上に確保される

        let _slice: &[i32] = &_vec[1..];
        // &[T], &mut [T] は共有スライスおよび可変スライスで配列やベクタへの連続した要素の参照

        // 配列
        let _arr2 = [1, 2, 3];
        let _arr3 = [0; 10];

        // ベクタ
        // 3つの値で構成される
        // 1. ヒープ上に確保されるバッファへのポインタ
        // 2. バッファに保持できる容量
        // 3. 現在保持している要素数
        let mut _v = vec![1, 2, 3];
        _v.push(4);
        let _v2 = Vec::<i32>::with_capacity(100);
        println!("{}, {}", _v.capacity(), _v2.capacity());

        // スライス
        // スライスの参照はファットポインタ、すなわちスライスの最初の要素を指すポインタと含まれる要素数
        // ベクタ、配列でも裏側ではスライスへの参照が使われている。スライスへの参照に関数を定義すればどちらでも使える
        {
            let v = vec![1, 2, 3, 4];
            let a = [1, 2, 3, 4];

            let sv = &v;
            let sa = &a;

            fn print(n: &[i32]) {
                for el in n {
                    println!("{}", el);
                }
            }
            print(&a);
            print(&v);

            print(&v[0..2]);
            print(&a[2..]);
            print(&sv[1..3]);
            print(&sa[..2]);
            print(&sa[..]);
        }
    }
    {
        // 文字列型
        let _s1 = "\"Ouch!\" said the well.\n";
        println!(
            "In the room the women come and go,
        Singing of Mount Abora"
        );
        println!(
            "It was a bright, cold day in April, and \
        there were four of us-\
        more or less."
        );

        let _s2 = r"C:\Users\user\Documents\file.txt";
        let _p = Regex::new(r"\d+");
        println!(
            r###"
        This is a raw string.###
        '"'"## '###'
        "###
        );

        // バイト文字列
        let _b = b"GET";

        // Rustの文字列はUnicode文字の列だが、charの配列としてメモリに格納されているわけではない。
        // 文字列は可変長のエンコーディングｄめおあるUTF-8で格納される。ASCII文字は1バイトで格納される。他の文字は複数バイトとなる。
        // &strはなにか別のものが所有している連続したUTF-8テキストへの参照でテキストを「借用」している
        // 他のスライス参照と同様にデータのアドレスと長さを含むファットポインタである
        // .len()はバイト数であって文字数ではない
        // 実行時に新しい文字列を作る際にはStringを用いる

        // &strは&[T]に似ている。StringはVec<T>に対応する。
        {
            let _s1 = "str".to_string();
            let _s2 = format!("{} {:02} {:02}", 24, 5, 23);
        }
        // 他の文字列に類する型
        // std::path::PathBuf &Path OsString &OsStr std::ffi::CString &CStr

        // 型エイリアス
        type _Bytes = Vec<u8>;
    }
}
