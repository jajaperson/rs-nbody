use std::fmt::Display;

use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Body {
    pub position: Point3,
    pub velocity: Vec3,
    pub mass: f64,
    next_velocity: Vec3,
}

impl Body {
    fn acceleration(&self, from: &Self) -> Vec3 {
        let r: Vec3 = from.position - self.position;

        (from.mass / r.length().powf(3.0)) * r
    }

    pub fn new(position: Point3, velocity: Vec3, mass: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
            next_velocity: velocity,
        }
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "r = [{:e}], v = [{:e}], Gm = {:e}",
            self.position, self.velocity, self.mass
        )
    }
}

#[derive(Debug)]
pub struct World {
    bodies: Vec<Body>,
    time: f64,
}

impl World {
    pub fn new(bodies: Vec<Body>) -> Self {
        Self { bodies, time: 0. }
    }

    pub fn tick(&mut self, tick_duration: f64) {
        // Calculate and integrate accelerations
        for i in 0..self.bodies.len() {
            let acceleration: Vec3 = self
                .bodies
                .iter()
                .enumerate()
                .filter_map(|(j, b)| (i != j).then(|| self.bodies[i].acceleration(b))) // forces from all bodies except itself
                .sum();
            self.bodies[i].next_velocity += acceleration * tick_duration;
        }
        // Integrate velocities and then update
        for body in &mut self.bodies {
            body.position += body.velocity * tick_duration;
            body.velocity = body.next_velocity;
        }
        self.time += tick_duration;
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }

    /// Converts into the Galilean rest frame of the body and index i
    pub fn into_rest_frame(&mut self, i: usize) {
        let (r_position, r_velocity) = self
            .bodies()
            .get(i)
            .map(|r| (r.position, r.velocity))
            .unwrap_or((Point3::ZERO, Vec3::ZERO));
        for body in &mut self.bodies {
            body.position -= r_position;
            body.velocity -= r_velocity;
            body.next_velocity -= r_velocity;
        }
    }
}
