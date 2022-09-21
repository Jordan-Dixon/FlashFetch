/* FlashFetch is a program written in rust to fetch system information in the terminal.
    Copyright (C) 2022  Jordan Dixon

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/agpl-3.0.html/>.
*/


fn main() {
    let shell = "SHELL";
    match std::env::var(shell) {
        Ok(shell) => println!("Shell => {}", shell),
        Err(e) => panic!("Shell somehow doesn't exist, how'd you fuck up this badly?! ({})", e),
    }
    let user = "USER";
    match std::env::var(user) {
        Ok(user) => println!("User => {}", user),
        Err(e) => panic!("Somehow you don't seem to have a name, you anonymous hacker you ({})", e),
    }
}
