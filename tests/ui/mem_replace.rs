// Copyright 2014-2019 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-rustfix
#![allow(unused_imports, deprecated, invalid_value)]
#![warn(clippy::all, clippy::style, clippy::mem_replace_option_with_none)]

use std::mem;

fn might_panic(v: Vec<i32>) -> Vec<i32> { v }

fn main() {
    let mut an_option = Some(1);
    let _ = mem::replace(&mut an_option, None);
    let an_option = &mut Some(1);
    let _ = mem::replace(an_option, None);

    let mut v = vec![0i32; 4];
    // the following is UB if `might_panic` panics
    unsafe {
        let taken_v = mem::replace(&mut v, mem::uninitialized());
        let new_v = might_panic(taken_v);
        std::mem::forget(mem::replace(&mut v, new_v));
    }
    
    unsafe {
        let taken_v = mem::replace(&mut v, mem::zeroed());
        let new_v = might_panic(taken_v);
        std::mem::forget(mem::replace(&mut v, new_v));
    }
    
    // this is silly but OK, because usize is a primitive type
    let mut u: usize = 42;
    let uref = &mut u;    
    let taken_u = unsafe { mem::replace(uref, mem::zeroed()) };
    *uref = taken_u + 1
}
