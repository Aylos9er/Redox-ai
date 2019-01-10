use super::Instruction;

pub use self::rectangle::Rectangle;
mod rectangle;

pub trait Shape2D {
    fn instructions(&self) -> &[Instruction];
}