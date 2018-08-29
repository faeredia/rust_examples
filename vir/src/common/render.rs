use common::screen::Screen;
use std::io::{stdin, stdout};

use termion::{clear, cursor};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//TODO
//re-write so that we redraw the enter screen. makes it far simpler
pub fn render_screen(sc: &mut Screen, buf: &Vec<String>) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{gotostart}",
        gotostart = cursor::Goto(sc.minx, sc.miny)
    );
    //let mut ftext = String::new();
    for (i, line) in buf.iter().enumerate() {
        //match line {
            //Some(t) => {
                let mut ftext = String::new();
                for ch in line.chars() {
                    match ch {
                        '\t'  => {
                            for _ in 0..4 {
                                ftext.push(' ');
                            }
                        },
                        c     => {
                            ftext.push(c);
                        },
                        //None        => (),
                    }
                };
                print!("{goto}{clearline}{print}",
                    goto = cursor::Goto(sc.minx, i as u16 + 1),
                    clearline = clear::CurrentLine,
                    print = ftext
                );
            //},
            //None    => break,
        //}
    }
}


pub fn render_line(sc: &mut Screen, buf: &Vec<String>, line_num: usize) {
       
    let text = match buf.get(line_num as usize - 1) {
        Some(t) => t,
        None => return,
    };
    
    //new string for the formatted text
    let mut ftext = String::new();
    for (i, ch) in text.chars().enumerate() {
        match ch {
            '\t' => {
                //let tabstop = 4;
                let spaces = 4;//(i+1_ % tabstop;
                for _ in 0..spaces {
                    ftext.push(' ');
                }
            },
            c => ftext.push(c),
        }
    }
    
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    print!("{goto}{clearline}",
        goto = cursor::Goto(1, line_num as u16),
        clearline = clear::CurrentLine
    );
    
    print!("{text}{goback}",
        text = ftext,
        goback = cursor::Goto(sc.linex, sc.liney)
    );
    //we leave flipping the screen up to the caller.
}
