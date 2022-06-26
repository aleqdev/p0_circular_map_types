use crate::{Circle, Room, PositionOfCircleSign, DataOfCircleSign, PositionOfRoomSign, DataOfRoomSign, CircleOfSign, RoomOfSign};

pub trait MapSign {
    type CircleSign: crate::CircleSign;
    type RoomSign: crate::RoomSign;
}

pub type CircleSignOfMapSign<T> = <T as MapSign>::CircleSign;
pub type RoomSignOfMapSign<T> = <T as MapSign>::RoomSign;

pub type PositionOfCircleSignOfMapSign<T> = PositionOfCircleSign<CircleSignOfMapSign<T>>;
pub type DataOfCircleSignOfMapSign<T> = DataOfCircleSign<CircleSignOfMapSign<T>>;

pub type PositionOfRoomSignOfMapSign<T> = PositionOfRoomSign<RoomSignOfMapSign<T>>;
pub type DataOfSignRoomOfSignMap<T> = DataOfRoomSign<RoomSignOfMapSign<T>>;

pub type CircleOfMapSign<T> = CircleOfSign<CircleSignOfMapSign<T>>;
pub type RoomOfMapSign<T> = RoomOfSign<RoomSignOfMapSign<T>>;

pub struct Map<T: MapSign> {
    circles: Vec<CircleOfMapSign<T>>,
    rooms: Vec<RoomOfMapSign<T>>,
}

impl<T: MapSign> Map<T> {
    pub fn circles(&self) -> &Vec<CircleOfMapSign<T>> {
        &self.circles
    }

    pub fn circles_mut(&mut self) -> &mut Vec<CircleOfMapSign<T>> {
        &mut self.circles
    }

    pub fn rooms(&self) -> &Vec<RoomOfMapSign<T>> {
        &self.rooms
    }

    pub fn rooms_mut(&mut self) -> &mut Vec<RoomOfMapSign<T>> {
        &mut self.rooms
    }
}

impl<T: MapSign> Default for Map<T> {
    fn default() -> Self {
        Self {
            circles: Default::default(),
            rooms: Default::default(),
        }
    }
}

impl<T: MapSign> std::fmt::Debug for Map<T>
where
    CircleOfMapSign<T>: std::fmt::Debug,
    RoomOfMapSign<T>: std::fmt::Debug 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map").field("circles", &self.circles).field("rooms", &self.rooms).finish()
    }
}

impl<T: MapSign> Clone for Map<T> 
where
    CircleOfMapSign<T>: Clone,
    RoomOfMapSign<T>: Clone
{
    fn clone(&self) -> Self {
        Self { 
            circles: self.circles.clone(), 
            rooms: self.rooms.clone() 
        }
    }
}
