extern crate termion;

use termion::{clear, cursor, color};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdin, stdout};
use std::time::Duration;
use std::thread;
use std::process::exit;
use std::cmp::{max, min};

use common::screen::{Screen, Bar};
use common::render::{render_line};
use common::curs::{XDir, move_lx};

fn print_bar(sc: &Screen, buf: &Vec<String>, bar: &Bar) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{goto}{clearline}-- INSERT --        ",
        goto = cursor::Goto(bar.x, bar.y),
        clearline = clear::CurrentLine
    );
    print!("Cursor{{x: {cursx}, y: {cursy}}}, Line{{len: {linelen}}}, Doc{{lines: {doclines}}}{goback}",
        cursx = sc.linex,
        cursy = sc.liney,
        linelen = buf[sc.liney as usize - 1].len(),
        doclines = buf.len(),
        goback = cursor::Goto(sc.linex, sc.liney)
    );
    stdout.flush().unwrap();
}


pub fn mode_insert( sc: &mut Screen, buf: &mut Vec<String> ) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    //let mut line = buf.get(sc.liney);
    //let mut x = 1;
    //let mut y = 2;
    //
    //let mut maxx: u16;
    //let mut maxy: u16;
    //match termion::terminal_size() {
    //    Ok((xx, yy))    => { maxx = xx; maxy = yy },
    //    Err(_)          => { maxx = 48; maxy = 48 },
    //}
    
    let bar = Bar{ x: 1, y: sc.maxy, text: "--INSERT--".to_string() };
    
    //draw the bar
    //print!("{goto}{clearline}{text}",
    //    goto = cursor::Goto(bar.x, bar.y),
    //    clearline = clear::CurrentLine,
    //    text = bar.text
    //);
    //set the cursor
    //print!("{}", cursor::Goto(sc.cursx, sc.cursy));
    //stdout.flush().unwrap();

    //write!(stdout, "{}{}-- INSERT --", cursor::Goto(1, sc.maxy), clear::CurrentLine); 
    //write!(stdout, "{}{}", cursor::Goto(sc.linex, sc.liney), clear::CurrentLine);
    //stdout.flush().unwrap();

    print_bar(sc, buf, &bar);

    //we assume that the buffer has at least one valid line to work with.
    for c in stdin.keys() {
        let ly = sc.liney as usize;
        let lx = sc.linex as usize;
        let len: usize; 

        match(buf.get(ly - 1)) {
            Some(i) => len = i.len() as usize,
            None => { 
                //buf.insert(ly - 1, String::new());
                len = 0
            },
        }
        //if len == 0 {
        //    buf.insert(ly - 1, String::new());
        //}
        print!("{}lx: {}, ly: {}, len: {}{}", cursor::Goto(20, sc.maxy), lx, ly, len, cursor::Goto(sc.linex, sc.liney));
        //stdout.flush().unwrap();
        match c.unwrap() {
            Key::Char('\n') => {
                //buf.insert(sc.liney as usize, String::new());
                //buf[ly - 1].insert(lx - 1, '\n');
                let remainder = buf[sc.liney as usize - 1].split_off(lx - 1);
                if sc.liney == buf.len() as u16{
                    //last line, use push
                    buf.push(remainder);
                } else {
                    buf.insert(ly, remainder);
                }
                //buf.insert(ly, remainder);
                //buf[sc.liney as usize] = remainder;
                sc.liney = sc.liney + 1;
                sc.linex = 1;
                //we need to force print every line below the newline
                for i in 0..(buf.len() as u16) {
                    let te: String;
                    match buf.get(i as usize) {
                        Some(t) => te = t.to_string(),
                        None => te = "".to_string(),
                    }
                    print!("{goto}{clearline}{text}",
                        goto = cursor::Goto(1, i as u16 + 1),
                        clearline = clear::CurrentLine,
                        text = te 
                    );
                }
                print!("{}", cursor::Goto(sc.linex, sc.liney));
                stdout.flush().unwrap();
            
            }
            Key::Char(c)    => { 
                //if len == 0 {
                //    buf.insert(ly - 1, String::new());
                //}
                if lx > len {
                    buf[ly - 1].push(c);
                    //sc.linex = sc.linex + 1;
                } else {
                    buf[ly - 1].insert(lx - 1, c);
                    //sc.linex = sc.linex + 1;
                }
                sc.linex = sc.linex + 1;
                //sc.cursx = sc.cursx + 1;
            },
            Key::Esc        => break,
            Key::Left       => {
                //if lx > 1 {
                //    sc.linex = sc.linex - 1;
                //}
                move_lx(sc, buf, 1, XDir::Left);
            }
            Key::Right      => { 
                //if lx < len + 1 {
                //    sc.linex = sc.linex + 1;
                //}
                move_lx(sc, buf, 1, XDir::Right);
            },
            Key::Up         => {
                if sc.liney > 1 {
                    sc.liney = sc.liney - 1;
                    sc.linex = min(sc.linex, buf.get(sc.liney as usize - 1).expect("whinge").len() as u16 + 1);
                }
                            },
            Key::Down       => {
                if sc.liney < buf.len()  as u16{
                    sc.liney = sc.liney + 1;
                    sc.linex = min(sc.linex, buf.get(sc.liney as usize - 1).expect("whinge").len() as u16 + 1);
                }
                            },
            Key::Backspace  => {
                if lx - 1 > 0 {
                    buf[ly - 1].remove(lx - 2);
                    sc.linex = sc.linex - 1;
                }
            },
            Key::Delete     => {
                if (sc.linex as usize) <= buf[sc.liney as usize - 1].len() {
                    buf[sc.liney as usize - 1].remove(sc.linex as usize - 1);
                    //sc.linex = sc.linex - 1;
                } 
            },
            _               => (),
        }
        let line: String;
        match buf.get(ly - 1) {
            Some(t) => line = t.to_string(),
            None => line = "".to_string(),
        }
        //add context to the bar
        print!("{}lx: {}, ly: {}, len: {}, lines: {}{}", 
            cursor::Goto(20, sc.maxy), 
            sc.linex, sc.liney, 
            buf[sc.liney as usize - 1].len(), 
            buf.len(), 
            cursor::Goto(sc.linex, sc.liney)
        );
        //reprint the damaged line
        //print!("{}{}{}{}", cursor::Goto(1, ly as u16), clear::CurrentLine, line, cursor::Goto(sc.linex,sc.liney));
        //a little test, can we render all lines in one go without flicker?
        for (i, _) in buf.iter().enumerate(){
            //render_line(sc, buf, i);
        print!("{}{}{}{}", cursor::Goto(1, ly as u16), clear::CurrentLine, line, cursor::Goto(sc.linex,sc.liney));
        }
        //render_line(sc, buf, ly);
        stdout.flush().unwrap();
        print_bar(sc, buf, &bar);

    }
    //print!("{}The terminal info is {}", cursor::Goto(1, sc.maxy - 1), termion::get_terminal_attr());
    //stdout.flush().unwrap();
}

