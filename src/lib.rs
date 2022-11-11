#![allow(dead_code, unused_variables)]      //TODO remove this for published versions

/*!
# dicediele

dicediele is a Rust program for rolling dice and calculating various
dice-related probabilities to assist tRPG players and DMs in making
appropriate decisions (currently WiP).

## Features

- Parses user input for dice notation
- Simulates rolling dice of different sizes
- Allows users to apply conditions such as explosions to dice rolls
- Calculates probability of success for various dice and success
conditions
- Provides the option to use either a CLI or GUI

## License

Copyright (C) 2022  Tom Su

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

Contact: <dieeisdiele.ts@gmail.com>

### Crates (MIT License)

- Clap [Copyright (c) 2015-2022 Kevin B. Knapp and Clap Contributors]

- egui/eframe [Copyright (c) 2018-2021 Emil Ernerfeldt
<emil.ernerfeldt@gmail.com>]

- lazy-static.rs [Copyright (c) 2010 The Rust Project Developers]

- Rand [Copyright (c) 2018 Developers of the Rand project;
Copyright (c) 2014 The Rust Project Developers]

- regex [Copyright (c) 2014 The Rust Project Developers]

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
*/

// use std::error::Error;

/// Core functionality of dicediele.
pub mod dice;

/// Graphical user interface (GUI) for dicediele.
pub mod gui;

/// Command line interface (CLI) for dicediele.
pub mod terminal;
