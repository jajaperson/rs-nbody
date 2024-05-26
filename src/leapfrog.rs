use std::fmt::Display;

use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Body {
    pub position: Point3,
    pub velocity: Vec3,
    pub mass: f64,
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
        // Integrate velocities
        for body in &mut self.bodies {
            // x[n+1] = x[n] + δt * v[1/2 + n]
            body.position += body.velocity * tick_duration;
        }
        self.time += tick_duration;
        // Calculate and integrate accelerations
        // a[n + 1] = f(r[n + 1])
        for i in 0..self.bodies.len() {
            let acceleration: Vec3 = self
                .bodies
                .iter()
                .enumerate()
                .filter_map(|(j, b)| (i != j).then(|| self.bodies[i].acceleration(b)))
                .sum();
            // v[1/2 + n + 1] = v[1/2 + n] + δt * a[n+1]
            self.bodies[i].velocity += tick_duration * acceleration
        }
    }

    pub fn half_tick_velocity(&mut self, tick_duration: f64) {
        // Calculate and integrate accelerations
        for i in 0..self.bodies.len() {
            let acceleration: Vec3 = self
                .bodies
                .iter()
                .enumerate()
                .filter_map(|(j, b)| (i != j).then(|| self.bodies[i].acceleration(b))) // forces from all bodies except itself
                .sum();
            self.bodies[i].velocity += acceleration * tick_duration / 2.;
        }
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
        }
    }
}
