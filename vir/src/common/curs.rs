extern crate num_traits;
use common::curs::num_traits::sign::signum;

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

//move the cursor num times. use sign to indicate direction
pub fn move_cx(sc: &mut Screen, buf: &Vec<String>, num: i32) {
    //needs to handle tabs
}

pub fn move_cy(sc: &mut Screen, buf: &Vec<String>, num: i32) {

}

//move the 'line x' num times. use sign to indicate direction
pub fn move_l(sc: &mut Screen, buf: &Vec<String>, num: u16, dir: XDir) {
    
    match dir {
        XDir::Left => {
            for _ in 0..num {
                if sc.linex <= 1 {
                    break;
                }
                sc.linex = sc.linex - 1;
            };
        },
        XDir::Right => {
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
    }   
}

//move the 'line y' num times. use sign to indicate direction
pub fn move_ly(sc: &mut Screen, buf: &Vec<String>, num: i32) {

}
