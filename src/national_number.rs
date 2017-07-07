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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NationalNumber {
	pub(crate) value: u64,

	/// In some countries, the national (significant) number starts with one or
	/// more "0"s without this being a national prefix or trunk code of some kind.
	/// For example, the leading zero in the national (significant) number of an
	/// Italian phone number indicates the number is a fixed-line number.  There
	/// have been plans to migrate fixed-line numbers to start with the digit two
	/// since December 2000, but it has not happened yet. See
	/// http://en.wikipedia.org/wiki/%2B39 for more details.
	///
	/// These fields can be safely ignored (there is no need to set them) for most
	/// countries. Some limited number of countries behave like Italy - for these
	/// cases, if the leading zero(s) of a number would be retained even when
	/// dialling internationally, set this flag to true, and also set the number of
	/// leading zeros.
	///
	/// Clients who use the parsing or conversion functionality of the i18n phone
	/// number libraries will have these fields set if necessary automatically.
	pub(crate) zeroes: Option<u8>,
}

impl NationalNumber {
	pub fn value(&self) -> u64 {
		self.value
	}

	pub fn zeroes(&self) -> Option<u8> {
		self.zeroes
	}
}
