// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
  A reduced test case for Issue #506, provided by Rob Arnold.

  Testing spawning foreign functions
*/

extern mod std;

#[abi = "cdecl"]
extern mod rustrt {
    #[legacy_exports];
    fn rust_dbg_do_nothing();
}

fn main() {
    task::spawn(rustrt::rust_dbg_do_nothing);
}
