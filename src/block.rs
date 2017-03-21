/// A terrain block is represented here

use na::{ Scalar, Isometry2, Vector2 };
use graphics::types::SourceRectangle;
use ncollide::bounding_volume::{self, BoundingVolume, AABB2};
use ncollide::shape::Cuboid;

const BLOCK_WIDTH: f64 = 64.0;
const BLOCK_HEIGHT: f64 = 64.0;

pub struct Block {
    position: (f64, f64),
}

impl Block {
    pub fn new() -> Self {
        Block {
            // Position is defined as the center of the block, so care must be taken to
            //   recall this when anchoring sprites and creating bounding boxes for collision
            position: (0.0, 0.0),
        }
    }

    /// Get the bounding box for the block
    pub fn get_bounding(&self) -> AABB2<f64> {
        // Use a cuboid shape for the block
        let block_cuboid: Cuboid<Vector2<f64>> = Cuboid::new(Vector2::new(BLOCK_WIDTH/2.0, BLOCK_HEIGHT/2.0));
        // The box_pos is an Isometry transformation on the block's position
        // This transformation takes the block's position as the translation from world-space to object-space
        //   and takes the additive identity as the rotation so that no rotation is performed.
        let box_pos = Isometry2::new(Vector2::new(self.position.0, self.position.1), ::na::zero());
        // Construct an axis-aligned bounding box with the box_post transformation result as the origin point
        bounding_volume::aabb(&block_cuboid, &box_pos)
    }
    
    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn set_position(&mut self, position: (f64, f64)) {
        self.position = position;
    }
}
