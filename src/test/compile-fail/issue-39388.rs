// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_macros)]

macro_rules! assign {
    (($($a:tt)*) = ($($b:tt))*) => { //~ ERROR 14:22: 14:29: expected one of: `*`, `+`, or `?`
        $($a)* = $($b)*
    }
}

fn main() {}
