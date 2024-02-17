use serde_derive::Serialize;

use self::stuffing::{per_round_stuffing, Rounds};

use super::common::*;

mod construction;
mod conversions;
pub mod examples;
mod stuffing;

#[allow(unused)]
#[derive(Clone, Serialize)]
pub enum Stuffing {
    None,
    PerRound,
}

#[derive(Clone, Serialize)]
pub struct Plushie {
    fixed_num: usize, // treat first N elements of `points` as fixed
    rounds: Rounds,
    pub points: Vec<Point>, // TODO need this pub?
    edges: Vec<Vec<usize>>,
    stuffing: Stuffing,
    desired_stitch_distance: f32,
    _gravity: f32,
}

impl Plushie {
    fn add_link_forces(&self, displacement: &mut Vec<V>) {
        for i in 0..self.points.len() {
            let this = self.points[i];
            for neibi in &self.edges[i] {
                let neib = self.points[*neibi];
                let diff: V = attract(this, neib, self.desired_stitch_distance);
                displacement[i] += diff;
                displacement[*neibi] -= diff;
            }
        }
    }

    fn add_stuffing_force(&mut self, displacement: &mut Vec<V>) {
        match &self.stuffing {
            Stuffing::None => (),
            Stuffing::PerRound => per_round_stuffing(
                &mut self.rounds,
                &self.points,
                self.desired_stitch_distance,
                displacement,
            ),
        }
    }

    fn apply_forces(&mut self, displacement: &Vec<V>, time: f32) {
        let mut total = V::zeros();
        for i in self.fixed_num..self.points.len() {
            total += displacement[i];
            self.points[i] += displacement[i] * time;
            // self.points[i].y = (self.points[i].y - self.gravity * time).max(0.0);
        }
        self.points[1].y += displacement[1].y * time;
    }

    pub fn step(&mut self, time: f32) {
        let mut displacement: Vec<V> = vec![V::zeros(); self.points.len()];

        self.add_link_forces(&mut displacement);
        self.add_stuffing_force(&mut displacement);

        self.apply_forces(&displacement, time);
    }

    pub fn animate(&mut self) {
        for _ in 0..1000 {
            self.step(1.0);
        }
    }
}

fn attract(this: Point, other: Point, desired_distance: f32) -> V {
    let diff = this - other;
    let x = diff.magnitude();
    let d = desired_distance;

    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    -diff.normalize() * fx
}
