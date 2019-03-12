extern crate game_of_life;
use game_of_life::Universe;

use std::{thread, time};

fn main() {
    let mut u = Universe::new();
    println!("{}", u.render());

    loop {
        thread::sleep(time::Duration::from_millis(500));
        u.tick();
        println!("{}", u.render());
    }
}
