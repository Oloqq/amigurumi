use std::sync::{Arc, Mutex};

use crate::{common::Point, genetic::common::Program, plushie::Plushie};

use super::sim::{Data, Simulation};

#[derive(Clone)]
pub struct PlushieControls {
    paused: bool,
    advance: usize,
    need_init: bool,
}

impl PlushieControls {
    fn new() -> Self {
        Self {
            paused: true,
            advance: 0,
            need_init: true,
        }
    }
}

#[derive(Clone)]
pub struct PlushieSimulation {
    controls: PlushieControls,
    plushie: Plushie,
    messages: Arc<Mutex<Vec<String>>>,
}

impl PlushieSimulation {
    pub fn from(plushie: Plushie) -> Self {
        Self {
            controls: PlushieControls::new(),
            plushie,
            messages: Arc::new(Mutex::new(vec![])),
        }
    }

    fn get_update_data(&self) -> serde_json::Value {
        serde_json::json!({
            "key": "upd",
            "dat": serde_json::json!(&self.plushie.points)
        })
    }

    fn get_init_data(&self) -> serde_json::Value {
        serde_json::json!({
            "key": "ini",
            "dat": serde_json::json!(self.plushie)
        })
    }

    fn change_pattern(&mut self, msg: &str) -> Result<(), String> {
        let (_, pattern) = match msg.split_once(" ") {
            Some(x) => x,
            None => return Err("frontend fuckup".into()),
        };
        log::info!("Changing pattern...");
        let stitches = Program::deserialize(pattern)?.tokens;
        self.plushie = Plushie::from_genetic(&(6, &stitches));
        Ok(())
    }
}

impl Simulation for PlushieSimulation {
    fn messages(&self) -> Arc<Mutex<Vec<String>>> {
        self.messages.clone()
    }

    fn step(&mut self, dt: f32) -> Option<Data> {
        if self.controls.need_init {
            self.controls.need_init = false;
            return Some(self.get_init_data().to_string());
        }

        if self.controls.paused && self.controls.advance == 0 {
            None
        } else {
            if self.controls.advance > 0 {
                self.controls.advance -= 1;
            }

            self.plushie.step(dt);

            let serialized = self.get_update_data().to_string();
            // println!("serialized: {serialized}");
            Some(serialized)
        }
    }

    fn react(&mut self, msg: &str) {
        let controls = &mut self.controls;
        if msg.starts_with("pos") {
            let tokens: Vec<&str> = msg.split(" ").collect();
            assert!(tokens.len() == 5);
            let id: usize = tokens[1].parse().unwrap();
            let x: f32 = tokens[2].parse().unwrap();
            let y: f32 = tokens[3].parse().unwrap();
            let z: f32 = tokens[4].parse().unwrap();
            self.plushie.points[id] = Point::new(x, y, z);
        } else if msg.starts_with("pattern") {
            if let Err(error) = self.change_pattern(msg) {
                self.messages.lock().unwrap().push(
                    serde_json::json!({
                        "key": "status",
                        "dat": format!("Couldn't parse: {}", error)
                    })
                    .to_string(),
                )
            } else {
                self.controls.need_init = true;
                self.messages.lock().unwrap().push(
                    serde_json::json!({
                        "key": "status",
                        "dat": "success"
                    })
                    .to_string(),
                )
            }
        } else {
            match msg {
                "pause" => controls.paused = true,
                "resume" => controls.paused = false,
                "advance" => controls.advance += 1,
                _ => log::error!("Unexpected msg: {msg}"),
            }
        }
    }
}
