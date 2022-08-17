use std::fmt;
use std::fmt::Display;

#[derive(Hash,Debug)]
pub struct Point<T> {
   pub x:T,
   pub y:T
}


impl<T> fmt::Display for Point<T> where T:Display{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}