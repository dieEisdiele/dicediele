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



/// Copyright notice to print on startup.
const NOTICE: &'static str = r#"dicediele  Copyright (C) 2022  Tom Su
This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it under certain
conditions.
See LICENSE.txt for details.

"#;

/// Logo to print on startup.
const LOGO: &'static str = r#"
          _    _                                    ,.::""""::.,
         | |  (_)                              ,.::"''  ***   ''"::.,
     ____| |   _    _____    ______         ;::"''      ***        ''":;
    /  _   |  | |  /  ___\  /  __  \       ::       ***      ***       ::
   |  (_)  |  | | |  (____ |  (_/ /__      ::':.,   ***      ***   ,.:'::
    \____/\_\ |_|  \_____/  \_______/      ::  ''"::.,        ,.::"''  ::
                           _               ::       ''"::;;::"''       ::
       _    _             | |              ::            ::  ***       ::
      | |  (_)            | |              ::    ***     ::  ***       ::
  ____| |   _    ______   | |   ______     ::     ***    ::        *** ::
 /  _   |  | |  /  __  \  | |  /  __  \    ::            ::        *** ::
|  (_)  |  | | |  (_/ /__ | | |  (_/ /__    "::.,        ::        ,.::"
 \____/\_\ |_|  \_______/ |_|  \_______/       ''"::.,   ::   ,.::"''
                                                    ''"::;;::"''

"#;

/// Lists user options and how to call them.
const MENU: &'static str = r#"What would you like to do?

1. x            Do x.
2. y            Do y.
3. z            Do z.

To learn more about a tool, enter its name followed by `--help` (e.g. enter
`x --help` to learn more about `x`).

"#;
