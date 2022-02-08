use std::net::IpAddr;

fn main() {
    // From, Into
    {
        use std::net::Ipv4Addr;
        fn ping<A>(address: A) -> std::io::Result<bool>
        where
            A: Into<Ipv4Addr>,
        {
            let ipv4_address = address.into();
            Ok(true)
        }

        let addr = Ipv4Addr::new(127, 0, 0, 1);
        ping(addr).unwrap();
        ping([127, 0, 0, 1]).unwrap();
        IpAddr::from([127, 0, 0, 1]);

        // FromがあればIntoを自動で実装してくれる
        // From/Intoは変換が安価であることをAsRef, AsMutと違って保証しない
        // ?演算子はFron/IntoをつかってErrorを変換している

        // TryFrom/TryInfo
        let huge = 1000_i64;
        let _ = huge.try_into().unwrap_or(i32::MAX);
    }

    // ToOwned
    {
        // cloneしたいけど、&str, &[i32]のクローンがほしいときどうする？
        // to_ownedのほうがいいらしい

        let arr = [1, 2, 3];
        let a = &arr;
        let b = a.clone();
        let c = a.to_owned();

        let str = "hello";
        let c = str.clone();
    }

    // Cow
    {
        // 関数の引数を参照か値で受け取るべきかという所有権の問題をよく考える必要がある
        // 実行時まで決められないケースのためにCow(clone on write)型が用意されている

        fn msg<'a>(flag: bool) -> &'a str {
            if flag {
                "Hello"
            } else {
                &"World".to_string()
            }
        }
    }
}
