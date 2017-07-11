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

use std::fmt;

use metadata::{DATABASE, Database, Metadata, Format};
use phone_number::PhoneNumber;
use consts;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
	E164,
	International,
	National,
	Rfc3966,
}

#[derive(Copy, Clone, Debug)]
pub struct Formatter<'n, 'd, 'f> {
	number:   &'n PhoneNumber,
	database: Option<&'d Database>,
	mode:     Mode,
	format:   Option<&'f Format>,
}

impl<'n, 'd, 'f> Formatter<'n, 'd, 'f> {
	pub fn database<'a>(self, database: &'a Database) -> Formatter<'n, 'a, 'f> {
		Formatter {
			number:   self.number,
			database: Some(database),
			mode:     self.mode,
			format:   self.format,
		}
	}

	pub fn mode(mut self, mode: Mode) -> Formatter<'n, 'd, 'f> {
		self.mode = mode;
		self
	}

	pub fn with<'a>(self, format: &'a Format) -> Formatter<'n, 'd, 'a> {
		Formatter {
			number:   self.number,
			database: self.database,
			mode:     self.mode,
			format:   Some(format),
		}
	}
}

pub fn format<'n>(number: &'n PhoneNumber) -> Formatter<'n, 'static, 'static> {
	Formatter {
		number:   number,
		database: None,
		mode:     Mode::E164,
		format:   None,
	}
}

pub fn format_with<'d, 'n>(database: &'d Database, number: &'n PhoneNumber) -> Formatter<'n, 'd, 'static> {
	Formatter {
		number:   number,
		database: Some(database),
		mode:     Mode::E164,
		format:   None,
	}
}

impl<'n, 'd, 'f> fmt::Display for Formatter<'n, 'd, 'f> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let db = self.database.unwrap_or(&*DATABASE);

		// If the country code is invalid, return an error.
		let meta = try_opt!(Err(fmt::Error);
			db.by_code(&self.number.country().code()).map(|m|
				m.into_iter().next().unwrap()));

		let national  = self.number.national().to_string();
		let formatter = self.format.or_else(|| formatter(&national,
			if meta.international_formats().is_empty() || self.mode == Mode::National {
				meta.formats()
			}
			else {
				meta.international_formats()
			}));

		match self.mode {
			// Requires no formatting at all, easy life.
			Mode::E164 => {
				write!(f, "+{}{}", self.number.country().code(), national)?;
			}

			// Space separated formatting with national specific rules.
			Mode::International => {
				write!(f, "+{} ", self.number.country().code())?;

				if let Some(formatter) = formatter {
					write!(f, "{}", replace(&national, meta, formatter, None, None))?;
				}
				else {
					write!(f, "{}", national)?;
				}

				if let Some(ext) = self.number.extension() {
					write!(f, "{}{}", meta.preferred_extension_prefix()
						.unwrap_or(" ext. "), ext)?;
				}
			}

			Mode::National => {
				if let Some(formatter) = formatter {
					let carrier = self.number.carrier().and_then(|c|
						formatter.domestic_carrier().map(|f| (c, f)));

					if let Some((carrier, format)) = carrier {
						write!(f, "{}", replace(&national, meta, formatter, Some(format), Some(carrier)))?;
					}
					else if let Some(prefix) = formatter.national_prefix() {
						write!(f, "{}", replace(&national, meta, formatter, Some(prefix), None))?;
					}
					else {
						write!(f, "{}", replace(&national, meta, formatter, None, None))?;
					}
				}
				else {
					write!(f, "{}", national)?;
				}

				if let Some(ext) = self.number.extension() {
					write!(f, "{}{}", meta.preferred_extension_prefix().unwrap_or(" ext. "), ext)?;
				}
			}

			Mode::Rfc3966 => {
				write!(f, "tel:+{}-", self.number.country().code())?;

				if let Some(formatter) = formatter {
					write!(f, "{}", consts::SEPARATOR_PATTERN.replace_all(
						&replace(&national, meta, formatter, None, None), "-"))?;
				}
				else {
					write!(f, "{}", national)?;
				}

				if let Some(ext) = self.number.extension() {
					write!(f, ";ext={}", ext)?;
				}
			}
		}

		Ok(())
	}
}

fn formatter<'a>(number: &str, formats: &'a [Format]) -> Option<&'a Format> {
	for format in formats {
		let leading = format.leading_digits();

		if leading.is_empty() || leading.last().unwrap().find(&number).map(|m| m.start() == 0).unwrap_or(false) {
			if format.pattern().find(&number).map(|m| m.start() == 0 && m.end() == number.len()).unwrap_or(false) {
				return Some(format);
			}
		}
	}

	None
}

fn replace(national: &str, meta: &Metadata, formatter: &Format, transform: Option<&str>, carrier: Option<&str>) -> String {
	use std::borrow::Cow;

	formatter.pattern().replace(national, &*if let Some(transform) = transform {
		let first  = consts::FIRST_GROUP.captures(&formatter.format()).unwrap().get(1).unwrap().as_str();
		let format = consts::NP.replace(transform, meta.national_prefix().unwrap_or(""));
		let format = consts::FG.replace(&format, &*format!("${}", first));
		let format = consts::CC.replace(&format, carrier.unwrap_or(""));

		consts::FIRST_GROUP.replace(formatter.format(), &*format)
	}
	else {
		Cow::Borrowed(formatter.format())
	}).into()
}

#[cfg(test)]
mod test {
	use parser;
	use formatter::Mode;
	use country::Country;

	#[test]
	fn us() {
//		assert_eq!("650 253 0000", formatter::format(Mode::National,
//			&parser::parse(Some(Country::US), "+1 6502530000").unwrap()).to_string());
//
//		assert_eq!("+1 650 253 0000", formatter::format(Mode::International,
//			&parser::parse(Some(Country::US), "+1 6502530000").unwrap()).to_string());
//
//		assert_eq!("800 253 0000", formatter::format(Mode::National,
//			&parser::parse(Some(Country::US), "+1 8002530000").unwrap()).to_string());
//
//		assert_eq!("+1 800 253 0000", formatter::format(Mode::International,
//			&parser::parse(Some(Country::US), "+1 8002530000").unwrap()).to_string());
//
//		assert_eq!("900 253 0000", formatter::format(Mode::National,
//			&parser::parse(Some(Country::US), "+1 9002530000").unwrap()).to_string());
//
//		assert_eq!("+1 900 253 0000", formatter::format(Mode::International,
//			&parser::parse(Some(Country::US), "+1 9002530000").unwrap()).to_string());

		assert_eq!("tel:+1-900-253-0000",
			parser::parse(Some(Country::US), "+1 9002530000").unwrap()
				.format().mode(Mode::Rfc3966).to_string());
  }

	#[test]
	fn gb() {
//		assert_eq!("(020) 7031 3000",
//			parser::parse(Some(Country::US), "+44 2070313000").unwrap()
//				.format().mode(Mode::National).to_string());

		assert_eq!("+44 20 7031 3000",
			parser::parse(Some(Country::US), "+44 2070313000").unwrap()
				.format().mode(Mode::International).to_string());

//    assertEquals("(020) 7031 3000", phoneUtil.format(GB_NUMBER, PhoneNumberFormat.NATIONAL));
//    assertEquals("+44 20 7031 3000", phoneUtil.format(GB_NUMBER, PhoneNumberFormat.INTERNATIONAL));
//
//    assertEquals("(07912) 345 678", phoneUtil.format(GB_MOBILE, PhoneNumberFormat.NATIONAL));
//    assertEquals("+44 7912 345 678", phoneUtil.format(GB_MOBILE, PhoneNumberFormat.INTERNATIONAL));
//
	}
}
