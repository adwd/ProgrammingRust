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
    }
}
