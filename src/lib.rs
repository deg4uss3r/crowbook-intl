// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate rustc_serialize;

mod macrogen;
mod lang;
mod error;
mod localizer;

pub use lang::Lang;
pub use macrogen::{generate_localize, generate_macro_file};
pub use error::{Error, Result};
pub use localizer::Localizer;