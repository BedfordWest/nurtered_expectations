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

impl<T> CollisionObjectData<T> {
    pub fn new(entity_type: T, velocity: Option<Vector2<f64>>) -> CollisionObjectData<T> {
        let init_velocity;
        if let Some(velocity) = velocity {
            init_velocity = Some(Cell::new(velocity))
        }

        else {
            init_velocity = None
        }

        CollisionObjectData {
            entity_type: entity_type,
            velocity: init_velocity
        }
    }
}
