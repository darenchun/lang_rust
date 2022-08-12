use std::cmp::PartialOrd;

pub trait Max<T> {
    fn max(&self) -> T;
}


pub struct TwoTuple<T> {
    pub first: T,
    pub second: T,
}

pub struct ThreeTuple<T> {
    pub first: T,
    pub second: T,
    pub third: T,
}

// PrtialOrd enables comparing
impl <T: PartialOrd + Copy> Max<T> for TwoTuple<T> {
    fn max(&self) -> T {
        if self.first >= self.second {
            return self.first;
        } else {
            return self.second;
        }
    }
}


// Same "Max" implementation can be different upon different tuples
impl <T: PartialOrd + Copy> Max<T> for ThreeTuple<T> {
    fn max(&self) -> T {
        if self.first >= self.second && self.first >= self.third {
            return self.first;
        } else if self.second >= self.first && self.second >= self.third {
            return self.second;
        } else {
            return self.third;
        }
    }
}



