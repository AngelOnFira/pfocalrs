// Generated by the cargo::prebuild() function. Do not edit by hand.

// If usage of .Call() and .Kall() functions in the package's R code changes,
// update this file by rerunning "cargo::prebuild(DIR)", where DIR is the root
// directory of this package.

/*
// Below is skeleton code that you can copy to your "src/rust/src/lib.rs" file
// and then uncomment. You can freely change the body and arguments names, but
// changing the name of a function or the number of arguments necessitates:
// 1. a corresponding change to invocations of .Call() and .Kall() in the R code
// and 2. rerunning cargo::prebuild().

mod registration;
use roxido::*;

#[roxido]
fn convolve2(a: Rval, b: Rval) -> Rval {
    Rval::nil()
}

#[roxido]
fn myrnorm(n: Rval, mean: Rval, sd: Rval) -> Rval {
    Rval::nil()
}

#[roxido]
fn zero(unnamed1: Rval, unnamed2: Rval, unnamed3: Rval, unnamed4: Rval) -> Rval {
    Rval::nil()
}
*/

use roxido::*;

#[no_mangle]
extern "C" fn R_init_pfocalrs_rust(info: *mut rbindings::DllInfo) {
    let mut call_routines = Vec::with_capacity(3);
    let mut _names: Vec<std::ffi::CString> = Vec::with_capacity(3);
    _names.push(std::ffi::CString::new(".convolve2").unwrap());
    call_routines.push(rbindings::R_CallMethodDef {
        name: _names.last().unwrap().as_ptr(),
        fun: unsafe { std::mem::transmute(crate::convolve2 as *const u8) },
        numArgs: 2,
    });
    _names.push(std::ffi::CString::new(".myrnorm").unwrap());
    call_routines.push(rbindings::R_CallMethodDef {
        name: _names.last().unwrap().as_ptr(),
        fun: unsafe { std::mem::transmute(crate::myrnorm as *const u8) },
        numArgs: 3,
    });
    _names.push(std::ffi::CString::new(".zero").unwrap());
    call_routines.push(rbindings::R_CallMethodDef {
        name: _names.last().unwrap().as_ptr(),
        fun: unsafe { std::mem::transmute(crate::zero as *const u8) },
        numArgs: 4,
    });
    call_routines.push(rbindings::R_CallMethodDef {
        name: std::ptr::null(),
        fun: None,
        numArgs: 0,
    });
    unsafe {
        rbindings::R_registerRoutines(
            info,
            std::ptr::null(),
            call_routines.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
        );
        rbindings::R_useDynamicSymbols(info, 0);
        rbindings::R_forceSymbols(info, 1);
    }
    roxido::r::set_custom_panic_hook();
}
