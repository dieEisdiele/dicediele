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



use dicediele::*;



#[doc(hidden)]
fn main() {
    println!("dicediele is a work in progress. Please come back later.");

    let test_str: &str = "d612-!h2\r\n";
    let test_parse = dice::lexer_parser::parse(test_str);
    for caps in test_parse {
        println!("N = {}\ndX = {}\ncond = {}", &caps["N"], &caps["dX"], &caps["cond"]);
    }
}
