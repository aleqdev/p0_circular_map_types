pub trait CircleDescriptor {
    type Position;
    type Data;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Circle<T: CircleDescriptor> {
    position: T::Position,
    data: T::Data,
}

impl<T: CircleDescriptor> Circle<T> {
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

impl<T: CircleDescriptor> Default for Circle<T>
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

impl<T: CircleDescriptor> std::fmt::Debug for Circle<T>
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
