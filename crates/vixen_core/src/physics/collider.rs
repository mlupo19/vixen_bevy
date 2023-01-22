use bevy::{prelude::{Vec3, IVec3, Component}, math::vec3};

use crate::player::PLAYER_SIZE;

pub trait SweptCollider<T: BroadphaseCollider<T>> {
    fn swept_collide(&self, other: &T, movement: &mut Movement) -> Option<(f32, Vec3)>;
}

pub trait BroadphaseCollider<T> {
    fn might_collide(&self, other: &T, movement: &Movement) -> bool;
}

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3)-> Self {
        Self { min, max }
    }

    pub fn from_block(position: &IVec3) -> Self {
        AABB::new(position.as_vec3(), (*position + IVec3::ONE).as_vec3())
    }

    pub fn from_player(position: Vec3) -> Self {
        let min = Vec3::new(position.x - PLAYER_SIZE.0 / 2., position.y, position.z - PLAYER_SIZE.2 / 2.);
        let max = Vec3::new(position.x + PLAYER_SIZE.0 / 2., position.y + PLAYER_SIZE.1, position.z + PLAYER_SIZE.2 / 2.);
        AABB::new(min, max)
    }

    #[inline]
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x < other.max.x && self.max.x > other.min.x &&
        self.min.y < other.max.y && self.max.y > other.min.y &&
        self.min.z < other.max.z && self.max.z > other.min.z
    }

    pub fn extend(&self, extension: &Vec3) -> Self {
        let (min, max) = (self.min.min(self.min + *extension), self.max.max(self.max + *extension));
        AABB::new(min, max)
    }
}

impl SweptCollider<AABB> for AABB {
    fn swept_collide(&self, other: &AABB, movement: &mut Movement) -> Option<(f32, Vec3)> {
        if !self.might_collide(other, movement) {
            return None;
        }

        // Get collision times/distances for each axis
        let tx = collide_time_axis(Axis {min: self.min.x, max: self.max.x, velo: movement.delta.x}, Axis {min: other.min.x, max: other.max.x, velo: 0.0});
        let ty = collide_time_axis(Axis {min: self.min.y, max: self.max.y, velo: movement.delta.y}, Axis {min: other.min.y, max: other.max.y, velo: 0.0});
        let tz = collide_time_axis(Axis {min: self.min.z, max: self.max.z, velo: movement.delta.z}, Axis {min: other.min.z, max: other.max.z, velo: 0.0});
        
        // Collision occurs at the time when all three axes are colliding
        let entry_time = tx.0.max(ty.0).max(tz.0);
        let exit_time = tx.1.min(ty.1).min(tz.1);
        let collide_time;
        let _normal: Vec3;
        let mut tangent: Vec3 = movement.delta;

        // Check if a collision will occur
        if entry_time > exit_time || (tx.0 < 0.0 && ty.0 < 0.0 && tz.0 < 0.0) || tx.0 > 1.0 || ty.0 > 1.0 || tz.0 > 1.0 {
            None
        } else {
            collide_time = entry_time;
            _normal = match entry_time {
                t if t == tx.0 => {tangent.x = 0.0; vec3(-1.0, 0.0, 0.0) * tx.2.signum()},
                t if t == ty.0 => {tangent.y = 0.0; vec3(0.0, -1.0, 0.0) * ty.2.signum()},
                t if t == tz.0 => {tangent.z = 0.0; vec3(0.0, 0.0, -1.0) * tz.2.signum()},
                _ => unreachable!("entry_time != x, y, or z entry time"),
            };
            Some((collide_time, tangent))
        }
    }
}


impl BroadphaseCollider<AABB> for AABB {
    #[inline]
    fn might_collide(&self, other: &AABB, movement: &Movement) -> bool {
        let extended_aabb = AABB::new(self.min.min(self.min + movement.delta), self.max.max(self.max + movement.delta));
        extended_aabb.intersects(other)
    }
}


fn collide_time_axis(object: Axis, other: Axis) -> (f32, f32, f32, f32) {
    let (entry, exit);
    if object.velo > 0. {
        entry = other.min - object.max;
        exit = other.max - object.min;
    } else {
        entry = other.max - object.min;
        exit = other.min - object.max;
    }

    let (t_entry, t_exit);
    if object.velo == 0. {
        (t_entry, t_exit) = (f32::NEG_INFINITY, f32::INFINITY);
    } else {
        t_entry = entry / object.velo;
        t_exit = exit / object.velo;
    }

    (t_entry, t_exit, entry, exit)
}

struct Axis {
    min: f32,
    max: f32,
    velo: f32
}

#[derive(Component, Default)]
pub struct Movement {
    pub velocity: Vec3,
    pub delta: Vec3,
}

#[cfg(test)]
mod tests {
    use bevy::math::{ivec3, vec3};

    use super::{AABB, SweptCollider, Movement};

    #[test]
    fn test_aabb() {
        let box1 = AABB::from_player(vec3(0.0,0.0,0.0));
        let box2 = AABB::from_block(&ivec3(2,0,0));

        let mut mvmt = Movement::default();
        mvmt.velocity = vec3(4.0, 0.0, 0.0);
        mvmt.delta = vec3(4.0, 0.0, 0.0);
        let x = box1.swept_collide(&box2, &mut mvmt);
        assert_eq!(x.expect("Should collide").1, vec3(0.0,0.0,0.0))
    }
}