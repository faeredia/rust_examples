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

pub fn move_cursor(sc: &mut Screen, buf: &Vec<String>, num: u16, dir: Dir) {
    match dir {
        Dir::Left   => {
        },

        Dir::Right  => {
        },

        Dir::Down   => {
        },

        Dir::Up     => {
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
