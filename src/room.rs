pub trait RoomSign {
    type Position;
    type Data;
}

pub type PositionOfRoomSign<T> = <T as RoomSign>::Position;
pub type DataOfRoomSign<T> = <T as RoomSign>::Data;

pub type RoomOfSign<T> = Room<T>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Room<T: RoomSign> {
    position: T::Position,
    data: T::Data,
}

impl<T: RoomSign> Room<T> {
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

impl<T: RoomSign> Default for Room<T>
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

impl<T: RoomSign> std::fmt::Debug for Room<T>
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
