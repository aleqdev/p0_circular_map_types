pub trait RoomDescriptor {
    type Position;
    type Data;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Room<T: RoomDescriptor> {
    position: T::Position,
    data: T::Data,
}

impl<T: RoomDescriptor> Room<T> {
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

impl<T: RoomDescriptor> Default for Room<T>
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

impl<T: RoomDescriptor> std::fmt::Debug for Room<T>
where
    T::Position: std::fmt::Debug,
    T::Data: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("position", &self.position)
            .field("data", &self.data)
            .finish()
    }
}
