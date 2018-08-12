extern crate time;
use time::{Timespec, Tm};

struct Todo {
    title: String,
    description: String,
    created_at: i64,
    completed_at: Option<i64>,
}

impl Todo{
    pub fn new(title: String, description: String) -> Todo {
        Todo {  title: title,
                description: description,
                created_at: time::now_utc().to_timespec().sec,
                completed_at: None
        }
    }
    
    
    fn created_at(&self) -> Tm {
        time::at(Timespec::new(self.created_at, 0))
    }

    fn completed_at(&self) -> Option<Tm> {
        match self.completed_at {
            None => None,
            Some(t) => Some(time::at(Timespec::new(t,0))),
        }
    }

    fn is_done(&self) -> bool {
        self.completed_at.is_some()
    }

}

fn main() {
    println!("todo tracker");
    
    let t = Todo::new("hooray".to_string(), "do stuff".to_string());
    println!("Ttitle: {}", t.title);
    println!("Description: {}", t.description);
    println!("Created at: {}", t.created_at);
    println!("Created at (pretty): {}", t.created_at().rfc822()t;
}
