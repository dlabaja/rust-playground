//input handling in crossplatform cli applications
//can be used on different threads, added ctrl+c handle for exitting

#[macro_use]
extern crate crossterm;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::stdout;

fn main() {
    let mut stdout = stdout();
    loop {
        enable_raw_mode().unwrap();
        let input = read().unwrap();
        match input {
            Event::Key(KeyEvent {
                           code: KeyCode::Char('p'),
                           modifiers: KeyModifiers::NONE,
                       }) => execute!(stdout, Print("pressed p")).unwrap(),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('c'),
                           modifiers: KeyModifiers::CONTROL,
                       }) => end_program(),
            _ => (),
        }
        disable_raw_mode().unwrap();
    }
}

fn end_program() {
    disable_raw_mode().unwrap();
    std::process::exit(0);
}