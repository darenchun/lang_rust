// Trait for our behavior
trait Sawtooth{
    fn sawtooth (&self) -> Self;
}

// extending the trait for builtin f6 type
impl Sawtooth for f64 {
    fn sawtooth (&self)-> f64 {return self - self.floor();}
}