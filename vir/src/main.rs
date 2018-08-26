
extern crate termion;

use termion::{clear, cursor, color};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdin, stdout};
use std::time::Duration;
use std::thread;
use std::process::exit;

mod modes;
use modes::sup::mode_super;

mod common;

fn main() { 
    //modes::sup::mode_super();
    let mut buf: Vec<String> = Vec::new();
    buf.push(String::new());
    mode_super(&mut buf);
    //let stdin = stdin();
    //let mut stdout = stdout().into_raw_mode().unwrap();
    //
    //let mut maxx: u16 = 0;
    //let mut maxy: u16 = 0;

    //match termion::terminal_size() {
    //    Ok((xx, yy))  => { maxx = xx; maxy = yy;},
    //    Err(e)      => println!("Yeah nah, don't have a clue what the term size is... {}", e),
    //}

    //println!("Max x: {}, Max y: {}", maxx, maxy);
    ////return;
    ////type ':q' to exit
    //write!(stdout, "{clear}{goto}Type ':q' to exit.\n",
    //    clear = clear::All,
    //    goto = cursor::Goto(1,1)
    //);
    //stdout.flush().unwrap();
    //write!(stdout, "{}", cursor::Goto(1,2));
    //stdout.flush().unwrap();

    //for c in stdin.keys() {
    //    write!(stdout, "{goto}{clear}", 
    //        goto = cursor::Goto(1,2), 
    //        clear = clear::CurrentLine
    //    ).unwrap();
    //    match c.unwrap() {
    //        Key::Char(':')  => { print!("{}:", cursor::Goto(1, maxy)); stdout.flush().unwrap(); parse_cmd() } ,
    //        Key::Char('\n') => print!("<enter>"),
    //        Key::Char('i')  => mode_insert(),
    //        Key::Char(c)    => print!("{}", c),
    //        Key::Left       => print!("<left>"),
    //        Key::Right      => print!("<right>"),
    //        _               => (),
    //    }
    //    
    //    stdout.flush().unwrap();
    //}
}

fn mode_insert() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut line = String::new();
    let mut x = 1;
    let mut y = 2;
    
    let mut maxx: u16;
    let mut maxy: u16;
    match termion::terminal_size() {
        Ok((xx, yy))    => { maxx = xx; maxy = yy },
        Err(_)          => { maxx = 48; maxy = 48 },
    }

    write!(stdout, "{}{}-- INSERT --", cursor::Goto(1, maxy), clear::CurrentLine); 
    write!(stdout, "{}{}", cursor::Goto(x,y), clear::CurrentLine);
    stdout.flush().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c)    => {
                if x as usize > line.len() {
                    line.push(c);
                    x = x + 1;
                } else {
                    line.insert(x as usize - 1, c);
                    x = x + 1;
                }
            },
            Key::Esc        => return,
            Key::Left       => {
                if x > 1 {
                    x = x - 1;
                }
            }
            Key::Right      => { 
                if x < (line.len() as u16) + 1 {
                    x = x + 1;
                }
            },
            Key::Backspace  => {
                if x - 1 > 0 {
                    line.remove(x as usize - 2);
                    x = x - 1;
                }
            },
            _               => (),
        }
        write!(stdout, "{}{}{}{}", cursor::Goto(1, y), clear::CurrentLine, line, cursor::Goto(x,y)).unwrap();
        stdout.flush().unwrap();

    }
}

fn parse_cmd() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut input = String::new();
    for c in stdin.keys() {
       // write!(stdout, "{goto}{clear}", 
       //     goto = cursor::Goto(1,2), 
       //     clear = clear::CurrentLine
       // ).unwrap();
        stdout.flush().unwrap(); 
        match c.unwrap() {
            //Key::Ret
            //Key::Char('\n') => { print!("Pressed enter!"); stdout.flush().unwrap(); break },
            Key::Char('\n') => { break },
            Key::Char(c)    => { print!("{}", c); stdout.flush().unwrap(); input.push(c); },
            Key::Esc        => return, //and unset clear what we wrote i guess...`
            _               => print!("Not a command"),
        }
    }
    //println!("{}", input);
    match input.as_ref() {
        "q" | "quit" => exit(0),
        _ => { println!("Not a command"); },
    }
}
