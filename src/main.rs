mod forward_euler;
mod leapfrog;
mod symplectic_euler;
mod vec3;

use std::fs::File;

use clap::{command, Parser, ValueHint};
use serde::{Deserialize, Serialize};
use vec3::{Point3, Vec3};

/// Basic implementation of an N-body simulator.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File containing simulation initial conditions. Must contain headers `pos_x`, `pos_y`,
    /// `pos_z`, `vel_x`, `vel_y`, `vel_z`, `mass`.
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    file: String,
    /// Tick duration.
    #[arg(short, long, default_value_t = 1e-3)]
    tick: f64,
    /// Simulation method.
    #[clap(short, long, default_value_t, value_enum)]
    sim: SimType,
    /// Duration of simulation.
    #[arg(short, long)]
    dur: f64,
    /// If specified, the final output will be presented in the rest frame of the body at this
    /// index. If the index is out of bounds, the default frame is used.
    #[clap(short, long)]
    rest_frame: Option<usize>,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum SimType {
    /// Direct integration
    #[default]
    ForwardEuler,
    SymplecticEuler,
    Leapfrog,
}

#[derive(Deserialize, Debug)]
struct CsvBody {
    pos_x: f64,
    pos_y: f64,
    pos_z: f64,
    vel_x: f64,
    vel_y: f64,
    vel_z: f64,
    mass: f64,
}

impl From<CsvBody> for forward_euler::Body {
    fn from(value: CsvBody) -> Self {
        Self::new(
            Point3::new(value.pos_x, value.pos_y, value.pos_z),
            Vec3::new(value.vel_x, value.vel_y, value.vel_z),
            value.mass,
        )
    }
}

impl From<CsvBody> for symplectic_euler::Body {
    fn from(value: CsvBody) -> Self {
        Self::new(
            Point3::new(value.pos_x, value.pos_y, value.pos_z),
            Vec3::new(value.vel_x, value.vel_y, value.vel_z),
            value.mass,
        )
    }
}

impl From<CsvBody> for leapfrog::Body {
    fn from(value: CsvBody) -> Self {
        Self::new(
            Point3::new(value.pos_x, value.pos_y, value.pos_z),
            Vec3::new(value.vel_x, value.vel_y, value.vel_z),
            value.mass,
        )
    }
}

fn read_csv<B>(file: File) -> Result<Vec<B>, csv::Error>
where
    B: From<CsvBody>,
{
    let mut reader = csv::Reader::from_reader(file);
    reader
        .deserialize()
        .map(|res| res.map(|row: CsvBody| row.into()))
        .collect()
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.file).expect("Unable to open the specified file");
    match args.sim {
        SimType::ForwardEuler => {
            let bodies: Vec<forward_euler::Body> =
                read_csv(file).expect("Error parsing the specified file");
            let mut world = forward_euler::World::new(bodies);
            while world.time() < args.dur {
                world.tick(args.tick)
            }
            if let Some(rest_frame) = args.rest_frame {
                world.into_rest_frame(rest_frame);
            }
            println!("Simulation time: {}", world.time());
            world
                .bodies()
                .iter()
                .for_each(|body| println!("{}, speed = {}", body, body.velocity.length()))
        }
        SimType::SymplecticEuler => {
            let bodies: Vec<symplectic_euler::Body> =
                read_csv(file).expect("Error parsing the specified file");
            let mut world = symplectic_euler::World::new(bodies);
            while world.time() < args.dur {
                world.tick(args.tick)
            }
            if let Some(rest_frame) = args.rest_frame {
                world.into_rest_frame(rest_frame);
            }
            println!("Simulation time: {}", world.time());
            world
                .bodies()
                .iter()
                .for_each(|body| println!("{}, speed = {}", body, body.velocity.length()))
        }
        SimType::Leapfrog => {
            let bodies: Vec<leapfrog::Body> =
                read_csv(file).expect("Error parsing the specified file");
            let mut world = leapfrog::World::new(bodies);
            world.half_tick_velocity(args.tick);
            while world.time() < args.dur {
                world.tick(args.tick)
            }
            if let Some(rest_frame) = args.rest_frame {
                world.into_rest_frame(rest_frame);
            }
            println!("Simulation time: {}", world.time());
            world
                .bodies()
                .iter()
                .for_each(|body| println!("{}, speed = {}", body, body.velocity.length()))
        }
    }
}
