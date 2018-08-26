extern crate termion;

use termion::{clear, cursor, color};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdin, stdout};
use std::time::Duration;
use std::thread;
use std::process::exit;

//mod insert;
use modes::insert::mode_insert;
use common::screen::{Screen, Bar};

//mode_super() should accept a Screen and text buffer

pub fn mode_super(buf: &mut Vec<String>) {
    let maxx: u16;
    let maxy: u16;

    match termion::terminal_size() {
        Ok((x, y))  => { maxx = x; maxy = y; },
        Err(_)      => { maxx = 46; maxy = 46; },
    }
    let mut sc = Screen{maxx: maxx, maxy: maxy, cursx: 1, cursy: 1, linex: 1, liney: 1};

    let bar = Bar{x: 1 as u16, y: sc.maxy, text: "-- SUPER --".to_string()};

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    //clear the screen
    print!("{clear}{goto}",
        clear = clear::All,
        goto = cursor::Goto(1,1)
    );
    //draw the bar
    print!("{goto}{text}",
        goto = cursor::Goto(bar.x, bar.y),
        text = bar.text
    );
    //set the cursor position
    print!("{}", cursor::Goto(sc.cursx, sc.cursy));
    stdout.flush().unwrap();

    //do something
    for c in stdin.keys() {
       // write!(stdout, "{goto}{clear}", 
       //     goto = cursor::Goto(1,2), 
       //     clear = clear::CurrentLine
       // ).unwrap();
        
        match c.unwrap() {
            Key::Char(':')  => print!("mode_command"),
            Key::Char('\n') => print!("<enter>"),
            Key::Char('i')  => mode_insert(&mut sc, buf),//print!("mode_insert"), //mode_insert(),
            Key::Char(c)    => print!("{}", c),
            Key::Left       => print!("<left>"),
            Key::Right      => print!("<right>"),
            Key::Esc        => exit(0),
            _               => (),
        }
        redraw(&sc, &bar);
        stdout.flush().unwrap();
    }

}

fn redraw(sc: &Screen, bar: &Bar) {
    //let sc = ScreenCurs{maxx: maxx, maxy: maxy, cursx: 1, cursy: 1};

    //let bar = Bar{x: 1 as u16, y: sc.maxy, text: "-- SUPER --".to_string()};

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    //draw the bar
    print!("{goto}{clearline}{text}",
        goto = cursor::Goto(bar.x, bar.y),
        clearline = clear::CurrentLine,
        text = bar.text
    );
    //draw text
    //not yet implemented
    //set the cursor position
    print!("{}", cursor::Goto(sc.cursx, sc.cursy));
    stdout.flush().unwrap();
}
