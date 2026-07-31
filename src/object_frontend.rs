use crate::constants_and_such::*;
use crate::methods::*;

impl Object2l {
    pub fn update(&mut self, dt: f32) {
        self.integrate_linear(dt); //first function
        self.integrate_angular(dt); //second function
        self.calculate_collision_mesh(); //third function
    }
}
