// Copyright (C) 2017 1aim GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

extern crate regex;
extern crate regex_cache;
extern crate fnv;
extern crate quick_xml as xml;
extern crate itertools;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

pub mod error;
pub use error::{Error, ErrorKind, Result};

pub mod metadata;
pub use metadata::Metadata;

mod national_number;
pub use national_number::NationalNumber;

pub mod country_code;
pub use country_code::CountryCode;

mod extension;
pub use extension::Extension;

pub mod phone_number;
pub use phone_number::PhoneNumber;

pub mod parser;
pub use parser::parse;

pub mod formatter;
pub use formatter::format;

pub fn init() -> error::Result<()> {
	lazy_static::initialize(&metadata::DATABASE);

	Ok(())
}
