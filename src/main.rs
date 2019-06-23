extern crate termion;
extern crate rustty;

use rustty::{Terminal, Cell, Color};

use std::time;
use std::thread;
use std::convert::TryFrom;



fn run() {
    let mut term = Terminal::new().unwrap();

    term.set_cursor(1, 1).unwrap();

    term[(0, 1)].set_bg(Color::Red);
    term[(0, 2)].set_fg(Color::Blue);

    assert_eq!(term[(0, 2)].fg(), Color::Blue);
    assert_eq!(term[(0, 1)].bg(), Color::Red);
    
    let mut n: u32 = 100;

    loop {
        if n == 150 {
            break;
        }

        term[(0, 0)] = Cell::with_char(char::try_from(n).unwrap());

        term.swap_buffers().unwrap();
        thread::sleep(time::Duration::from_millis(500));
        n += 1;
    }

    println!("Writing to main screen.");
}

fn main() {
    run();
}

