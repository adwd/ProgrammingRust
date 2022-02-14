fn main() {
    // 外部言語関数
    // Cの構造体と互換性を持つRustの構造体を定義するには #[repr(C)]属性を用いる

    ffi();
}

use std::os::raw::{c_char, c_int};

#[repr(C)]
struct git_error {
    pub message: *const c_char,
    pub klass: c_int,
}
// Cスタイルの列挙型の表現を制御するにも使える
#[repr(C)]
#[allow(non_camel_case_types)]
enum git_error_code {
    GIT_OK = 0,
    GIT_ERROR = -1,
    GIT_ENOTFOUND = -3,
    GIT_EEXISTS = -4,
    // ...
}
// 列挙型の表現を別の整数型にすることもできる
#[repr(i16)]
#[allow(non_camel_case_types)]
enum git_error_code2 {
    GIT_OK = 0,
    GIT_ERROR = -1,
    GIT_ENOTFOUND = -3,
    GIT_EEXISTS = -4,
    // ...
}

#[repr(C)]
enum Tag {
    Float = 0,
    Int = 1,
}

#[repr(C)]
union FloatOrInt {
    f: f32,
    i: i32,
}

#[repr(C)]
struct Value {
    tag: Tag,
    union: FloatOrInt,
}

fn is_zero(v: &Value) -> bool {
    use self::Tag::*;
    unsafe {
        match v {
            Value {
                tag: Int,
                union: FloatOrInt { i: 0 },
            } => true,
            Value {
                tag: Float,
                union: FloatOrInt { f: num },
            } => (*num == 0.0),
            _ => false,
        }
    }
}

// externブロックではRustの実行ファイルとリンクされるライブラリで定義されている関数や変数を宣言する
extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

use std::ffi::CString;

fn ffi() {
    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();

    unsafe {
        assert_eq!(strlen(null_terminated.as_ptr()), 12);
    }

    // externブロックではグローバル変数も宣言できる
    ffi2();
    ffi3();
}

use std::ffi::CStr;

extern "C" {
    static environ: *mut *mut c_char;
}

fn ffi2() {
    unsafe {
        if !environ.is_null() && (*environ).is_null() {
            let var = CStr::from_ptr(*environ);
            println!("first environment variable: {}", var.to_string_lossy());
        }
    }
}

// 特定のライブラリで提供される関数を用いるには#[link]属性をexternブロックの上につける
#[link(name = "git2")]
extern "C" {
    pub fn git_libgit2_init() -> c_int;
    pub fn git_libgit2_shutdown() -> c_int;
}

fn ffi3() {
    unsafe {
        git_libgit2_init();
        git_libgit2_shutdown();
    }
}

// bindgenクレートでCのヘッダファイルをパースしてRustの宣言を自動的に生成できる
