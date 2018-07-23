// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "foo"]

pub mod a {
    pub struct Foo;
    pub enum Bar {
        Baz,
    }
}

// @count 'foo/index.html' '//*[code="pub use a::Foo;"]' 1
#[doc(no_inline)]
pub use a::Foo;
// @count 'foo/index.html' '//*[code="pub use a::Bar::Baz;"]' 1
#[doc(no_inline)]
pub use a::Bar::Baz;