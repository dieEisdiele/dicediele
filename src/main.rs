//     dicediele is a Rust program for rolling dice and calculating various dice-related probabilities to assist tRPG players and DMs in making appropriate decisions.
//     Copyright (C) 2022  Tom Su

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.

//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

//     E: dieeisdiele.ts@gmail.com



use terminal_io::t_roll_dice_loop;

mod dice;
mod terminal_io;
fn main() {
    t_roll_dice_loop();
}
