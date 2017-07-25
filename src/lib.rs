#![feature(const_fn)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!("bindings.rs");

#[macro_use]
extern crate lazy_static;

use std::ptr::{null, null_mut};
use std::os::raw::{c_int, c_char};
use std::ffi::CString;
use std::mem::transmute;

mod brackets;

// https://github.com/rust-lang/rfcs/issues/400#issuecomment-274140470
macro_rules! cstr {
  ($s:expr) => (
    concat!($s, "\0") as *const str as *const [c_char] as *const c_char as *mut c_char
  );
}

pub struct BinWrapper<'a>(&'a mut [builtin]);
unsafe impl<'a> Sync for BinWrapper<'a> { }

pub struct FeaturesWrapper(features);
unsafe impl Sync for FeaturesWrapper { }


pub static mut bintab: BinWrapper<'static> = BinWrapper(&mut [builtin {
    node: hashnode {
        next: null_mut(),
        nam: cstr!("fastbrackets"),
        flags: 0
    },
    handlerfunc: Some(bin_fastbrackets),
    minargs: 0,
    maxargs: -1,
    funcid: 0,
    optstr: null_mut(),
    defopts: null_mut()
}]);


lazy_static! {
    pub static ref module_features: FeaturesWrapper = FeaturesWrapper(features {
        bn_list: unsafe { &bintab.0[0] as *const builtin as *mut builtin },
        bn_size: unsafe { bintab.0.len() as c_int },
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
pub extern fn setup_(m: Module) -> c_int {
    println!("Setup!");
    //unsafe { bintab.0[0].node.nam = CString::new("Hi").unwrap().into_raw() }
    0
}

#[no_mangle]
pub extern fn boot_(m: Module) -> c_int {
    println!("Boot!");
    0
}

#[no_mangle]
pub extern fn cleanup_(m: Module) -> c_int {
    println!("Cleanup!");
    0
}

#[no_mangle]
pub extern fn finish_(m: Module) -> c_int {
    println!("Finish!");
    0
}
#[no_mangle]
pub extern fn features_(m: Module, features: *mut *mut *mut c_char) -> c_int {
    println!("Features!");
    unsafe {
        *features = featuresarray(m, &module_features.0 as *const features as *mut features);
    }
    0
}



#[no_mangle]
pub extern fn enables_(m: Module, enables: *mut *mut c_int) -> c_int {
    println!("Enables!");
    unsafe {
        return handlefeatures(m, &module_features.0 as *const features as *mut features, enables);
    }
}

#[no_mangle]
pub extern fn bin_fastbrackets(name: *mut c_char, args: *mut *mut c_char, options: Options, func: c_int) -> c_int {
    println!("Called!");
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
