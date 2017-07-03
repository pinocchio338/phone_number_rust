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

#[derive(Copy, Clone, Debug)]
pub struct CountryCode {
	/// The country code value.
	value: u16,

	/// The source from which the country_code is derived.
	source: Source,
}

/// The source from which the country_code is derived. This is not set in the
/// general parsing method, but in the method that parses and keeps raw_input.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Source {
	/// The country_code is derived based on a phone number with a leading "+",
	/// e.g. the French number "+33 1 42 68 53 00".
	Plus,

	/// The country_code is derived based on a phone number with a leading IDD,
	/// e.g. the French number "011 33 1 42 68 53 00", as it is dialled from US.
	Idd,

	/// The country_code is derived based on a phone number without a leading
	/// "+", e.g. the French number "33 1 42 68 53 00" when defaultCountry is
	/// supplied as France.
	Number,

	/// The country_code is derived NOT based on the phone number itself, but
	/// from the defaultCountry parameter provided in the parsing function by the
	/// clients. This happens mostly for numbers written in the national format
	/// (without country code). For example, this would be set when parsing the
	/// French number "01 42 68 53 00", when defaultCountry is supplied as
	/// France.
	Default,
}

impl Default for Source {
	fn default() -> Source {
		Source::Default
	}
}
