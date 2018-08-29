extern crate num_traits;
use common::curs::num_traits::sign::signum;

use std::cmp::{min};

use common::screen::Screen;

pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}
//Cursor module. 
//use to control the cursor on the screen etc.
//we could use enums and stuff.... that may stop dumb things....
//this is easier though, business improvement for later. 

fn char_at_index(line: &str, index: usize) -> char {
    for (i, c) in line.chars().enumerate() {
        match i {
            index   => return c,
            _       => (),
        }
    }
    ' '
}

fn line_len_for_cursor(sc: &Screen, buf: &Vec<String>, index: u16) -> usize {
    let mut count: usize = 0;
    
    for ch in buf.get(sc.liney as usize - 1).unwrap().chars() {
        match ch {
            '\t'    => count = count + 4,
            _       => count = count + 1,
        };
    }
    count
}

pub fn move_cursor(sc: &mut Screen, buf: &Vec<String>, num: u16, dir: Dir) {
    match dir {
        Dir::Left   => {
            for _ in 0..num {
                if sc.cursx <= sc.minx {
                    break;
                }
                sc.cursx = sc.cursx - 1;
            }
        },

        Dir::Right  => {
            let len = match buf.get(sc.liney as usize - 1) {
                Some(t) => t.len(),
                None    => 0,
            };
            //let len = line_len_for_cursor(sc, buf, sc.liney - 1);
            for _ in 0..num {
                if sc.cursx >= sc.maxx || sc.cursx as usize > len {
                    break;
                }
                sc.cursx = sc.cursx + 1;
                //get the current char
                //let c = buf.get(sc.liney as usize -1).unwrap().get(sc.linex as usize).unwrap();
                //match buf.get(sc.liney as usize -1){
                //    Some(line)  => {
                //        match line.chars().nth(sc.linex as usize - 2) {
                //            //Some('\t')  => sc.cursx = sc.cursx + 4,
                //            _           => sc.cursx = sc.cursx + 1
                //        }
                //    },
                //    None        => (),
                //}
                //{
                //    '\t'    => sc.cursx = sc.cursx + 3,
                //    _       =>sc.cursx = sc.cursx + 1,
                //}
            }
        },

        Dir::Down   => {
        for _ in 0..num {
                if sc.cursy >= sc.maxy as u16 || sc.cursy as usize >= buf.len(){
                    break;
                }
                sc.cursy = sc.cursy + 1;
                
                let len = match buf.get(sc.liney as usize - 1) {
                    Some(i) => i.len(),
                    None    => 0,
                } as u16;              
                sc.cursx = min(sc.cursx, len + 1);
            }
        },

        Dir::Up     => {
        for _ in 0..num {
                if sc.cursy <= sc.miny {
                    break;
                }
                sc.cursy = sc.cursy - 1;
                
                let len = match buf.get(sc.liney as usize - 1) {
                    Some(i) => i.len(),
                    None    => 0,
                } as u16;
                sc.cursx = min(sc.cursx, len + 1);
             }
        },
    }
}

//move the line position  num times
pub fn move_line(sc: &mut Screen, buf: &Vec<String>, num: u16, dir: Dir) {
    
    match dir {
        Dir::Left => {
            for _ in 0..num {
                if sc.linex <= 1 {
                    break;
                }
                sc.linex = sc.linex - 1;
            };
        },

        Dir::Right => {
            let len = match buf.get(sc.liney as usize - 1) {
                Some(i) => i.len(),
                None    => 0,
            } as u16;
            for _ in 0..num {
                if sc.linex >= len + 1 {
                    break;
                }
                sc.linex = sc.linex + 1;
            };
        },

        Dir::Down => {
            for _ in 0..num {
                if sc.liney >= buf.len()  as u16{
                    break;
                }
                sc.liney = sc.liney + 1;
                
                let len = match buf.get(sc.liney as usize - 1) {
                    Some(i) => i.len(),
                    None    => 0,
                } as u16;              
                sc.linex = min(sc.linex, len + 1);
            }
        },

        Dir::Up => {
            for _ in 0..num {
                if sc.liney <= 1 {
                    break;
                }
                sc.liney = sc.liney - 1;
                
                let len = match buf.get(sc.liney as usize - 1) {
                    Some(i) => i.len(),
                    None    => 0,
                } as u16;
                sc.linex = min(sc.linex, len + 1);
             }
        }
    }
}
