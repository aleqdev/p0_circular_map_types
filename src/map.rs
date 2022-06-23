use crate::{Circle, Room};

pub trait MapDescriptor {
    type CircleDescriptor: crate::CircleDescriptor;
    type RoomDescriptor: crate::RoomDescriptor;
}

pub struct Map<T: MapDescriptor> {
    circles: Vec<Circle<T::CircleDescriptor>>,
    rooms: Vec<Room<T::RoomDescriptor>>,
}

impl<T: MapDescriptor> Map<T> {
    pub fn circles(&self) -> &Vec<Circle<T::CircleDescriptor>> {
        &self.circles
    }

    pub fn circles_mut(&mut self) -> &mut Vec<Circle<T::CircleDescriptor>> {
        &mut self.circles
    }

    pub fn rooms(&self) -> &Vec<Room<T::RoomDescriptor>> {
        &self.rooms
    }

    pub fn rooms_mut(&mut self) -> &mut Vec<Room<T::RoomDescriptor>> {
        &mut self.rooms
    }
}

impl<T: MapDescriptor> Default for Map<T> {
    fn default() -> Self {
        Self {
            circles: Default::default(),
            rooms: Default::default(),
        }
    }
}

impl<T: MapDescriptor> std::fmt::Debug for Map<T>
where
    Circle<T::CircleDescriptor>: std::fmt::Debug,
    Room<T::RoomDescriptor>: std::fmt::Debug 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map").field("circles", &self.circles).field("rooms", &self.rooms).finish()
    }
}

impl<T: MapDescriptor> Clone for Map<T> 
where
    Circle<T::CircleDescriptor>: Clone,
    Room<T::RoomDescriptor>: Clone
{
    fn clone(&self) -> Self {
        Self { 
            circles: self.circles.clone(), 
            rooms: self.rooms.clone() 
        }
    }
}
