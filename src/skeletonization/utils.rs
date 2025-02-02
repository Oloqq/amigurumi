use crate::common::*;
use std::{collections::HashSet, f32::consts::PI};

/// Nodes in relaxed plushie have max distance of ~1.4
const CLUSTER_DISTANCE_THRESHOLD: f32 = 1.4;
pub struct Connectivity {
    backward: Vec<Vec<usize>>,
    forward: Vec<Vec<usize>>,
}
// const GLOBAL_THRESHOLD: f32 = 1.5 * CLUSTER_DISTANCE_THRESHOLD;

impl Connectivity {
    pub fn new(edges: &Vec<Vec<usize>>) -> Self {
        let mut forward: Vec<Vec<usize>> = vec![vec![]; edges.len()];
        for i in 0..edges.len() {
            for j in &edges[i] {
                forward[*j].push(i);
            }
        }

        Self {
            backward: edges.clone(),
            forward,
        }
    }
}

fn get_connected(edges: &Connectivity, node: usize) -> HashSet<usize> {
    edges.backward[node]
        .clone()
        .into_iter()
        .chain(edges.forward[node].clone())
        .collect()
}

fn filter_connected(seed: usize, nodes: HashSet<usize>, edges: &Connectivity) -> Vec<usize> {
    let mut result: HashSet<usize> = HashSet::with_capacity(nodes.len());
    let mut frontier: HashSet<usize> = HashSet::with_capacity(nodes.len());
    let mut closed: HashSet<usize> = HashSet::with_capacity(nodes.len());
    frontier.insert(seed);

    while frontier.len() > 0 {
        let elem = frontier.iter().next().unwrap().clone();
        frontier.remove(&elem);
        closed.insert(elem);

        let connected: HashSet<usize> = get_connected(edges, elem)
            .intersection(&nodes)
            .cloned()
            .collect();
        result.extend(&connected);
        frontier.extend(connected.difference(&closed));
    }

    assert!(
        result.len() <= nodes.len(),
        "{} <= {}",
        result.len(),
        nodes.len()
    );
    Vec::from_iter(result.into_iter())
}

fn get_inliers(
    cloud: &Vec<Point>,
    edges: &Connectivity,
    threshold: f32,
    seed: usize,
    normal_offset: &V,
) -> Vec<usize> {
    let d = normal_offset.dot(&cloud[seed].coords);
    let close_to_plane: HashSet<usize> = cloud
        .iter()
        .enumerate()
        .filter_map(|(i, p)| ((normal_offset.dot(&p.coords) - d).abs() <= threshold).then_some(i))
        .collect();
    filter_connected(seed, close_to_plane, edges)
}

fn orient_cost(normals: &Vec<V>, inliers: &Vec<usize>, normal_offset: &V) -> f32 {
    inliers
        .iter()
        .map(|i| normal_offset.dot(&normals[*i]).abs())
        .sum::<f32>()
        / inliers.len() as f32
}

#[derive(Debug, Clone)]
pub struct Orientation(pub f32, pub f32);

pub fn find_best_plane(
    cloud: &Vec<Point>,
    normals: &Vec<V>,
    connectivity: &Connectivity,
    seed: usize,
    considered_normals: &Vec<(V, Orientation)>,
) -> (Orientation, Vec<usize>, f32) {
    let mut candidates: Vec<(Orientation, f32)> = Vec::with_capacity(considered_normals.len());
    let mut debug_inliers: Vec<Vec<usize>> = Vec::with_capacity(candidates.capacity());
    for (normal, angles) in considered_normals {
        let inliers = get_inliers(
            cloud,
            connectivity,
            CLUSTER_DISTANCE_THRESHOLD,
            seed,
            normal,
        );
        let cost = orient_cost(normals, &inliers, normal);
        candidates.push((angles.clone(), cost));
        debug_inliers.push(inliers);
    }

    let (index, best_orientation, best_cost) = candidates
        .into_iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.1.total_cmp(&b.1))
        .and_then(|(i, candidate)| Some((i, candidate.0, candidate.1)))
        .unwrap();

    (
        best_orientation,
        debug_inliers.swap_remove(index),
        best_cost,
    )
}

pub fn orient_planes(
    cloud: &Vec<Point>,
    normals: &Vec<V>,
    connectivity: &Connectivity,
    seeds: &Vec<usize>,
) -> Vec<(Orientation, Vec<usize>, f32)> {
    const ANGULAR_INTERVAL: f32 = PI / 6.0;
    const THETA_STEPS: usize = 12;
    const PHI_STEPS: usize = 4;
    let mut considered_normals: Vec<(V, Orientation)> = Vec::with_capacity(THETA_STEPS * PHI_STEPS);
    for theta in (0..THETA_STEPS).map(|t| t as f32 * ANGULAR_INTERVAL) {
        for phi in (0..PHI_STEPS).map(|p| p as f32 * ANGULAR_INTERVAL) {
            considered_normals.push((
                V::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos()),
                Orientation(theta, phi),
            ));
        }
    }

    seeds
        .iter()
        .map(|seed| find_best_plane(cloud, normals, connectivity, *seed, &considered_normals))
        .collect()
}
