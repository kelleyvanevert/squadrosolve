#![feature(int_roundings)]

mod game;
mod play_against_human;
mod play_against_itself;
mod util;

// use play_against_human::play_against_human;
use play_against_itself::play_against_itself;

fn main() {
    play_against_itself();
}
