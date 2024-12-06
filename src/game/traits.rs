use super::components::Card;

pub trait Shufflable{
    fn shuffle(&mut self);
}

pub trait Dealable{
    fn deal(&mut self) -> Card;
}