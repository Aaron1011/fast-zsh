#![feature(const_fn)]
#![feature(const_ptr_null_mut)]
#![feature(test)]
#![feature(conservative_impl_trait)]


#[macro_use]
extern crate lazy_static;
extern crate test;

#[allow(warnings)]
mod ffi;
pub mod brackets;

use std::ptr::null_mut;
use std::os::raw::{c_int, c_char};
use std::ffi::CStr;
use std::error::Error;
use ffi::*;

use brackets::brackets_paint;

// https://github.com/rust-lang/rfcs/issues/400#issuecomment-274140470
macro_rules! cstr {
  ($s:expr) => (
    concat!($s, "\0") as *const str as *const [c_char] as *const c_char as *mut c_char
  );
}

pub struct BinWrapper(Box<[builtin]>);
unsafe impl Sync for BinWrapper { }

pub struct FeaturesWrapper(features);
unsafe impl Sync for FeaturesWrapper { }

pub struct Args {
    buf: String,
    bracket_color_size: usize,
    cursor: usize,
    widget: String
}

pub type ParseArgsError = Box<Error>;

impl Args {
    pub unsafe fn from_raw(raw_args: *mut *mut c_char) -> Result<Args, ParseArgsError> {
        let args_str;
        args_str = CStr::from_ptr(*raw_args as *const c_char).to_str().unwrap().to_owned();

        let args = args_str.splitn(4, ',').collect::<Vec<_>>();

        let bracket_color_size = match args[0].parse::<usize>() {
            Ok(s) => s,
            Err(e) => {
                return Err(Box::new(e))

            }
        };

        let cursor = match args[1].parse::<usize>() {
            Ok(s) => s,
            Err(e) => {
                return Err(Box::new(e))
            }
        };

        let widget = args[2].to_string();
        let buffer = args[3].to_string();

        Ok(Args {
            buf: buffer.to_owned(),
            bracket_color_size: bracket_color_size,
            cursor: cursor,
            widget: widget
        })
    }
}


lazy_static! {
    static ref BINTAB: BinWrapper = BinWrapper(Box::new([builtin {
        node: hashnode {
            next: null_mut(),
            nam: cstr!("fastbrackets"),
            flags: 0
        },
        handlerfunc: Some(bin_fastbrackets),
        minargs: 1,
        maxargs: 1,
        funcid: 0,
        optstr: null_mut(),
        defopts: null_mut()
    }]));
}


lazy_static! {
    pub static ref MODULE_FEATURES: FeaturesWrapper = FeaturesWrapper(features {
        bn_list: &BINTAB.0[0] as *const builtin as *mut builtin,
        bn_size: BINTAB.0.len() as c_int,
        cd_list: null_mut(),
        cd_size: 0,
        mf_list: null_mut(),
        mf_size: 0,
        pd_list: null_mut(),
        pd_size: 0,
        n_abstract: 0
    });
}


#[no_mangle]
#[allow(unused_variables)]
pub extern fn setup_(m: Module) -> c_int {
    println!("Setup!");
    0
}

#[no_mangle]
#[allow(unused_variables)]
pub extern fn boot_(m: Module) -> c_int {
    println!("Boot!");
    0
}

#[no_mangle]
#[allow(unused_variables)]
pub extern fn cleanup_(m: Module) -> c_int {
    println!("Cleanup!");
    0
}

#[no_mangle]
#[allow(unused_variables)]
pub extern fn finish_(m: Module) -> c_int {
    println!("Finish!");
    0
}
#[no_mangle]
pub unsafe extern fn features_(m: Module, features: *mut *mut *mut c_char) -> c_int {
    println!("Features!");
    *features = featuresarray(m, &MODULE_FEATURES.0 as *const features as *mut features);
    0
}



#[no_mangle]
pub unsafe extern fn enables_(m: Module, enables: *mut *mut c_int) -> c_int {
    println!("Enables!");
    handlefeatures(m, &MODULE_FEATURES.0 as *const features as *mut features, enables)
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern fn bin_fastbrackets(name: *mut c_char, raw_args: *mut *mut c_char, options: Options, func: c_int) -> c_int {
    let args = Args::from_raw(raw_args).unwrap();
//unsafe { zwarnnam(name, CString::new(format!("Bad bracket color size (should be impossible): {:?} {:?}", args[0], e)).unwrap().into_raw()) } ;
//unsafe { zwarnnam(name, CString::new(format!("Invalid cursor argument: {:?} {:?}", args[2], e)).unwrap().into_raw()) } ;
    brackets_paint(args.bracket_color_size, &args.buf, args.cursor, &args.widget);

    0
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::ffi::CString;
    use test::Bencher;

    #[test]
    fn simple_bracket() {
        brackets_paint(8, "[]", 0, "lol");
    }

    #[test]
    fn cursor_bracket() {
        brackets_paint(3, ": ((( )))", 2, "no");
    }

    #[test]
    fn unmatched_bracket() {
        brackets_paint(3, "[]]", 0, "");
    }

    #[test]
    fn crazy_buffer() {
        brackets_paint(3, ",  []())sdfs$ ,,sdfsdf -.", 0, "");
    }

    #[test]
    fn large_buffer() {
        let data = &[b'a'; 30_000];
        let buf = unsafe { str::from_utf8_unchecked(data) };
        brackets_paint(3, &buf, 0, "");
    }

    #[test]
    #[should_panic]
    fn parse_invalid_args() {
        let args = "a,b";
        let c_args = CString::new(args).unwrap().into_raw() as *mut c_char;
        let parsed_args = unsafe { Args::from_raw((&c_args) as *const *mut c_char as *mut *mut c_char).unwrap() };
    }

    #[bench]
    fn bench_many_brackets(b: &mut Bencher) {
        use brackets::brackets_paint;
        use test::Bencher;

        let brackets_str = (0..5000).map(|_| "[]").collect::<String>();
        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from disregarding
            // unused values
            test::black_box(brackets_paint(0, &brackets_str, 0, ""));
        });
    }

    #[bench]
    fn even_more(b: &mut Bencher) {
        use brackets::brackets_paint;
        use test::Bencher;

        let brackets_str = (0..500000).map(|_| "[[[]]]").collect::<String>();
        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from disregarding
            // unused values
            test::black_box(brackets_paint(0, &brackets_str, 0, ""));
        });
    }

}
