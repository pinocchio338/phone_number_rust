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

use std::ops::Deref;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Extension(pub(crate) String);

impl Deref for Extension {
	type Target = str;

	fn deref(&self) -> &str {
		&self.0
	}
}

impl AsRef<str> for Extension {
	fn as_ref(&self) -> &str {
		&self.0
	}
}
