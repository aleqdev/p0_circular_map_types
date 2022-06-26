pub trait CircleSign {
    type Position;
    type Data;
}

pub type PositionOfCircleSign<T> = <T as CircleSign>::Position;
pub type DataOfCircleSign<T> = <T as CircleSign>::Data;

pub type CircleOfSign<T> = Circle<T>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Circle<T: CircleSign> {
    position: T::Position,
    data: T::Data,
}

impl<T: CircleSign> Circle<T> {
    pub fn new(pos: T::Position, data: T::Data) -> Self {
        Self {
            position: pos,
            data,
        }
    }

    pub fn position(&self) -> &T::Position {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut T::Position {
        &mut self.position
    }
}

impl<T: CircleSign> Default for Circle<T>
where
    T::Position: Default,
    T::Data: Default,
{
    fn default() -> Self {
        Self {
            position: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T: CircleSign> std::fmt::Debug for Circle<T>
where
    T::Position: std::fmt::Debug,
    T::Data: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Circle")
            .field("position", &self.position)
            .field("data", &self.data)
            .finish()
    }
}
