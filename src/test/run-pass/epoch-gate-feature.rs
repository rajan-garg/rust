// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Checks if the correct registers are being used to pass arguments
// when the sysv64 ABI is specified.

// compile-flags: -Zepoch=2018

pub trait Foo {}

// should compile without the dyn trait feature flag
fn foo(x: &dyn Foo) {}

pub fn main() {}
