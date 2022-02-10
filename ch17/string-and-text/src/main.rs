fn main() {
    // Unicodeについて
    // char: Unicodeのコードポイント1つ
    // String: 所有されたUnicode文字列
    // str: 借用されたUnicode文字列
    {
        // ASCII, Latin-1, Unicode
        // Unicodeは0から0x7fまではASCIIと一致し、0xffまではLatin-1と一致する
        fn latin1_to_char(latin1: u8) -> char {
            latin1 as char
        }

        fn char_to_latin1(c: char) -> Option<u8> {
            if c as u32 <= 0xff {
                Some(c as u8)
            } else {
                None
            }
        }

        // UTF-8
        // RustのString型やstr型はUTF-8で表現される
        // UTF-8は1文字を1バイトから4バイトの列にエンコードする

        assert_eq!(
            "うどん: udon".as_bytes(),
            &[
                0xe3, 0x81, 0x86, // う
                0xe3, 0x81, 0xa9, // ど
                0xe3, 0x82, 0x93, // ん
                0x3a, 0x20, 0x75, 0x64, 0x6f, 0x6e, // : udon
            ]
        );
        // 0x7fまでのコードポイントはそのままASCII文字になる
        // とかなんとかで扱いやすいらしい
    }

    // char
    {
        // charはUnicodeのコードポイントを保持する32ビットの値

        // charのメソッドはasciiと付くのとそうじゃないのがある
        // asciiがつかない方はUnicodeの特殊な文字も考慮していて思った動作と違うかもなので注意
        [
            '4'.is_numeric(),
            '⑧'.is_numeric(),
            'q'.is_alphabetic(),
            '七'.is_alphabetic(),
            '9'.is_alphanumeric(),
            '藏'.is_alphanumeric(),
            ' '.is_whitespace(),
            '\n'.is_whitespace(),
            '\n'.is_control(),
            'h'.is_ascii(),
            '\n'.is_ascii(),
            'a'.is_ascii_alphabetic(),
            'Z'.is_ascii_alphabetic(),
            '9'.is_ascii_digit(),
            !'⑧'.is_ascii_digit(),
            '0'.is_ascii_hexdigit(),
            'f'.is_ascii_hexdigit(),
            !'g'.is_ascii_hexdigit(),
            '0'.is_ascii_alphanumeric(),
            '\n'.is_ascii_control(),
            '~'.is_ascii_graphic(), // 空白や改行など以外の表示できる文字
            'a'.is_ascii_lowercase(),
            'A'.is_ascii_uppercase(),
            '!'.is_ascii_punctuation(),
            ' '.is_ascii_whitespace(),
            // is_ascii_ メソッドはu8バイトにも使用できる
            32_u8.is_ascii_whitespace(),
        ]
        .iter()
        .for_each(|x| assert!(x));

        assert_eq!('F'.to_digit(16), Some(15));
        assert_eq!('F'.to_digit(10), None);
        assert_eq!('9'.to_digit(10), Some(9));

        // to_lowercase, to_uppercase
        // これらはイテレータを返す。言語によっては大文字にすると2文字になるとかあるので
    }

    // String, str
    {
        // Stringとstrは整形式なUTF-8を保持することが保証され、作られたあとの操作されたあとも同じ

        let str = "Hello, world!";
        let str2 = str;
        let string = str2.to_string();
        let slice = &str[1..];
        let own = slice.to_owned();

        // Stringはstd::fmt::Writeを実装しているのでwrite!,writeln!マクロでStringにテキストを追加できる

        // Patternトレイト

        // contains, starts_with, ends_with, find, rfind

        // FromStr

        // Displayトレイト
        // Debugトレイト
        // from_utf8

        // ヒープの確保をCowで遅延する
        // あるときはString,あるときは&'static strな場合、Cow(clone-on-write)が使える
        use std::borrow::Cow;
        fn get_name() -> Cow<'static, str> {
            std::env::var("User")
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed("whoever you are"))
        }
        println!("Greetings, {}!", get_name());

        fn get_name2() -> Cow<'static, str> {
            std::env::var("User")
                .map(|v| v.into())
                .unwrap_or_else(|_| "whoever you are".into())
        }
    }

    // 値のフォーマット出力
    {
        println!(
            "{:.3}us: relocated object at {:#x} to {:#x}, {} bytes",
            0.84391, 140737488346304_usize, 6299664_usize, 64
        );

        // format!, println!, print!, writeln!, write!, panic!
        // std::fmt, format_args!, std::fmt::Argumentsなどで拡張できる

        // {...} をフォーマットパラメータと呼ぶ。 {which:how} の形式を取る
        // 両者とも省略可能でその場合 {} となる
        // which: 引数の名前、インデックスで指定する。省略すると左から順に適用する
        // how: 最長テキスト長、最短フィールド幅、アラインメント、パディング
        let str = "bookends";
        println!("{}.", str); // デフォルト
        println!("{:4}.", str); // 最短フィールド幅
        println!("{:12}.", str); //
        println!("{:.4}.", str); // 最長テキスト長
        println!("{:.12}.", str); //
        println!("{:12.20}.", str); // フィールド幅、テキスト長
        println!("{:4.20}.", str); // 左寄せ、フィールド幅
        println!("{:4.6}.", str); //
        println!("{:6.4}.", str); //
        println!("{:<12}.", str); //
        println!("{:^12}.", str); //　中央、フィールド幅
        println!("{:>12}.", str); // 右寄せ、フィールド幅
        println!("{:=^12}.", str); // '='でパディング、中央、フィールド幅
        println!("{:*>12.4}.", str); // '*'でパディング、右寄せ、フィールド幅、テキスト長
        println!();

        // Unicodeのややこしい問題は無視している。ちゃんとしたいならUIツールキットやHTML/CSSとか、unicode-widthというクレートもある

        // 数値のフォーマット
        // パディングとアラインメント
        // 文字 '+': 符号を常に表示する
        // 文字 '#': 基数を明示する
        // 文字 '0': 最短フィールド幅を満たすのに通常のパディングでなく0を用いる
        // 最短フィールド幅
        // 精度: 浮動小数点数の小数点以下何桁出力するか
        // 記法: b, o, x, X で2進数、8進数、16進数、16進数大文字で出力する。浮動小数点数に対してはe, Eで科学技術記法になる

        // 整数のフォーマット
        println!("{}.", 1234); //
        println!("{:+}.", 1234); //
        println!("{:12}.", 1234); //
        println!("{:2}.", 1234); //
        println!("{:+12}.", 1234); //
        println!("{:012}.", 1234); //
        println!("{:+012}.", 1234); //
        println!("{:<12}.", 1234); //
        println!("{:^12}.", 1234); //
        println!("{:>12}.", 1234); //
        println!("{:<+12}.", 1234); //
        println!("{:^+12}.", 1234); //
        println!("{:>+12}.", 1234); //
        println!("{:=^12}.", 1234); //
        println!("{:b}.", 1234); //
        println!("{:12o}.", 1234); //
        println!("{:+12x}.", 1234); //
        println!("{:+12X}.", 1234); //
        println!("{:+#12x}.", 1234); //
        println!("{:+#012x}.", 1234); //
        println!("{:+#06x}.", 1234); //
        println!();

        // 浮動小数点数のフォーマット
        println!("{}.", 1234.5678); //
        println!("{:.2}.", 1234.5678); //
        println!("{:.6}.", 1234.5678); //
        println!("{:12}.", 1234.5678); //
        println!("{:12.2}.", 1234.5678); //
        println!("{:12.6}.", 1234.5678); //
        println!("{:012.6}.", 1234.5678); //
        println!("{:e}.", 1234.5678); //
        println!("{:.3e}.", 1234.5678); //
        println!("{:12.3e}.", 1234.5678); //
        println!("{:12.3E}.", 1234.5678); //

        // デバッグフォーマット
        // {:?} {:#?}
        // {:02?} {:02x?}
        // {:p} ポインタのアドレス

        assert_eq!(
            format!("{1},{0},{2}", "zeroth", "first", "second"),
            "first,zeroth,second"
        );

        assert_eq!(
            format!(
                "{description:.<25}{quantity:2} @ {price:5.2}",
                price = 3.25,
                quantity = 3,
                description = "Maple Turmeric Latte"
            ),
            "Maple Turmeric Latte..... 3 @  3.25"
        );
        // この名前付き引数はformatマクロだけのもの

        // 動的なフィールド幅
        println!("{:>1$}.", "hello", 10);
        println!("{:>width$}.", "hello", width = 10);

        // ユーザ定義型のフォーマット出力
        // std::fmt::Display, Binary, Octalなどを実装する
        // とかいろいろ
    }

    // 正規表現
    {
        // regexクレート
        use regex::Regex;

        let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();
        let haystack = r#"regex = "0.2.5""#;
        assert!(semver.is_match(haystack));

        let captures = semver
            .captures(haystack)
            .ok_or("semver regex should have matched")
            .unwrap();
        assert_eq!(&captures[0], "0.2.5");
        assert_eq!(&captures[1], "0");
        assert_eq!(&captures[2], "2");
        assert_eq!(&captures[3], "5");
    }

    // 正規化
    {
        // Unicodeは合成形、分解形といったややこしい問題がある
        // そういったものに対して正規型が定義されている
        // unicode-normalizationクレートがあり、正規化できる
    }
}
