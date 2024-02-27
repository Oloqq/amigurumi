use crate::benchmark::run_benchmark;
use crate::genetic::common::Token;
#[allow(unused)]
use crate::meshes_sandbox::*;
use crate::{common::*, ws_sim::serve_websocket};

extern crate nalgebra as na;

mod args;
mod benchmark;
mod common;
mod genetic;
mod meshes_sandbox;
mod pattern;
mod plushie;
mod ws_sim;

use args::*;
use pattern::Pattern;
use pattern::Stitch;
use plushie::examples;
use plushie::Plushie;
use std::io::Write;
use ws_sim::plushie_sim::PlushieSimulation;

fn main() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let args = Args::from_args();
    use Command::*;
    match args.cmd {
        WebSocket {} => {
            let plushie = examples::bigball();
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim);
        }
        Dev { num } => exec_dev_action(num),
        Genetic(genetic) => {
            let suite = &genetic.suite;
            println!("Selected suite: {suite}");
            run_benchmark(&suite, &genetic);
        }
        FromPattern { pattern, stl, ws } => {
            let pattern = Pattern::from_file(pattern);
            let mut plushie = Plushie::from_pattern(pattern);

            if stl.is_some() && ws.is_some() {
                unimplemented!("use either --stl or --ws");
            }

            if let Some(stl_path) = stl {
                plushie.animate();
                save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
            } else if ws.is_some() {
                let sim = PlushieSimulation::from(plushie);
                serve_websocket(sim);
            }
        }
    }
}

fn exec_dev_action(num: usize) {
    fn generate(name: &str, func: fn() -> Plushie) {
        let mut plushie = func();
        // println!(
        //     "{:?}",
        //     plushie.points.iter().map(|a| a.y).collect::<Vec<_>>()
        // );
        plushie.animate();
        // println!(
        //     "{:?}",
        //     plushie.points.iter().map(|a| a.y).collect::<Vec<_>>()
        // );
        save_mesh(name, plushie.to_mesh());
    }

    println!("dev action {num}");
    match num {
        2 => generate("generated/pillar.stl", examples::pillar),
        3 => generate("generated/ball.stl", examples::ball),
        4 => generate("generated/bigball.stl", examples::bigball),
        _ => println!("no such action"),
    }
}
