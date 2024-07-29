mod args;
mod common;
mod comparison;
mod flow;
mod plushie;
mod rocket_server;
mod ws_sim;
extern crate nalgebra as na;
#[macro_use]
extern crate rocket;
use flow::ergoflow::ErgoFlow;
use flow::simple_flow::SimpleFlow;

use self::args::*;
use self::ws_sim::plushie_sim::PlushieSimulation;
use crate::flow::pest_parser::Pattern;
use crate::plushie::examples;
use crate::plushie::Params;
use crate::plushie::{Plushie, Pointcloud};
use crate::ws_sim::serve_websocket;
use std::fs;
use std::io::Write;

fn main() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let args = Args::from_args();
    use Command::*;
    match args.cmd {
        WebSocket(args) => {
            let mut plushie = examples::ergogrzib();
            plushie.params = Params::handpicked_for_grzib();
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim, format!("127.0.0.1:{}", args.port).as_str());
        }
        Inspect(args) => {
            // inspect population
            let population_file = args.popfile;
            let content = fs::read_to_string(population_file).unwrap();
            let genomes: Vec<Vec<u8>> = serde_json::from_str(&content).unwrap();
            let index = args.index;
            let genome = &genomes[index];
            let actions = flow::genetic::v1::express_genes(genome);
            let flow = SimpleFlow::new(actions);
            let plushie = Plushie::from_flow(flow, Params::handpicked_for_pillar()).unwrap();
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim, "127.0.0.1:8080");
        }
        Dev { num } => {
            match num {
                1 => {
                    let d = Params::default();
                    let s = serde_json::to_string_pretty(&d).unwrap();
                    println!("{s}");
                }
                2 => {
                    let plushie = examples::ergogrzib();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                3 => {
                    // viewing vertexes in an STL
                    let plushie = Pointcloud::from_stl("models/grzib40.stl");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                4 => {
                    // viewing a pregenerated pointcloud
                    let plushie =
                        Pointcloud::from_points_file("model_preprocessing/models/pillar.json");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                5 => {
                    // visually compare a pointcloud to an example
                    let mut primary = examples::pillar();
                    primary.params = Params::handpicked_for_grzib();
                    let secondary =
                        Pointcloud::from_points_file("model_preprocessing/models/pillar.json");
                    let sim = PlushieSimulation::with_secondary(primary, secondary);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                // start the ranking server
                8 => rocket_server::main(),
                9 => {
                    // see an evolved individual in action
                    let genome: Vec<u8> = serde_json::from_str("[0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 2, 1, 0, 1, 1, 1, 2, 1, 2, 1, 0, 1, 2, 0, 1, 0, 1, 1, 2, 0, 0, 1, 1, 1, 2, 0, 2, 0, 1, 2, 1, 1, 0, 1, 0, 2, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 2, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 2, 2, 0, 2, 2, 0, 1, 2, 1, 2, 0, 0, 1, 2, 1, 0, 2, 1, 2, 0, 0, 0, 1, 2, 0, 1, 1, 0, 2, 1, 1, 2, 1, 0, 2, 0, 0, 2, 1, 1, 2, 2, 2, 0, 1, 1, 0, 2, 2, 0, 1, 1, 1, 0, 0, 2, 1, 0]").unwrap();
                    let actions = flow::genetic::v1::express_genes(&genome);
                    let plushie =
                        Plushie::from_flow(ErgoFlow::from(actions), Params::handpicked_for_grzob())
                            .unwrap();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                10 => {}
                11 => {
                    let mut plushie = examples::ergogrzob();
                    plushie.params = Params::handpicked_for_grzob();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                _ => {}
            }
            println!(":)");
            println!(":)");
        }
        Genetic(genetic) => {
            let suite = &genetic.suite;
            println!("Selected suite: {suite}");
            unimplemented!();
            // run_benchmark(&suite, &genetic);
        }
        FromPattern { pattern, stl, ws } => {
            let pattern = {
                let content = fs::read_to_string(&pattern).unwrap();
                match Pattern::parse(&content) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{e}");
                        return;
                    }
                }
            };
            let mut params: Params = Default::default();
            params.update(&pattern.meta);
            let plushie = Plushie::from_flow(pattern, params).unwrap();

            if stl.is_some() && ws || stl.is_none() && !ws {
                println!("use either --stl or --ws");
                return;
            }

            if let Some(_stl_path) = stl {
                unimplemented!()
                // plushie.animate();
                // save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
            } else if ws {
                let sim = PlushieSimulation::from(plushie);
                serve_websocket(sim, "127.0.0.1:8080");
            }
        }
    }
}
