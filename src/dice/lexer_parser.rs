// dicediele is a Rust program for rolling dice and calculating various
// dice-related probabilities to assist tRPG players and DMs in making
// appropriate decisions.
// Copyright (C) 2022  Tom Su

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Please also see README.md for a list of crates used in this program and
// their respective copyright and licensing information. 

// Contact: dieeisdiele.ts@gmail.com

use lazy_static::lazy_static;
use regex::{Regex, CaptureMatches};



/// A regular expression for recognising dice notation.
pub fn parse(dice_string: &str) -> CaptureMatches {
    //TODO Handle errors properly
    //TODO Replace lazy_static with once_cell
    lazy_static! {
    pub static ref DICE_NOTATION_REGEX: Regex = Regex::new(r"\s(?P<N>\d*)d(?P<X>\d+)(?P<cond>[^\s]*)\s").unwrap();
    }

    let dice_tokens = DICE_NOTATION_REGEX.captures_iter(dice_string);
    dice_tokens
}
