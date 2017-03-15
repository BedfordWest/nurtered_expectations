/// The collision detection pipelines

use na::{Point2, Isometry2, Vector2};
use ncollide::world::CollisionWorld;
use std::cell::Cell;

pub struct Collision {
    world: CollisionWorld<Point2<f64>, Isometry2<f64>, ()>,
}

impl Collision {
    pub fn new() -> Self {
        Collision {
            world: CollisionWorld::new(0.02, false),
        }
    }
}

struct CollisionObjectData<T> {
    entity_type: T,
    velocity: Option<Cell<Vector2<f64>>>,
    
}
