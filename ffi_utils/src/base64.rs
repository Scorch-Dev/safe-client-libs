// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use rustc_serialize::base64::{CharacterSet, Config, FromBase64, FromBase64Error, Newline, ToBase64};

/// Encode the data using base64 encoding.
pub fn base64_encode(input: &[u8]) -> String {
    input.to_base64(config())
}

/// Decode base64 encoded data.
pub fn base64_decode(input: &str) -> Result<Vec<u8>, FromBase64Error> {
    input.from_base64()
}

#[inline]
fn config() -> Config {
    Config {
        char_set: CharacterSet::UrlSafe,
        newline: Newline::LF,
        pad: false,
        line_length: None,
    }
}
