use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Params {
    /// Multiplier to all forces in a single step
    pub timestep: f32,
    /// Set to true if creation is meant to stand on it's own to simulate a flat bottom
    /// Set to false if the creation is carried around, so that the bottom is not flat
    pub floor: bool,
    /// Force pulling the nodes down
    pub gravity: f32,
    /// Distance between nodes that is considered "relaxed"
    pub desired_stitch_distance: f32,
    /// Configuration of centroid stuffing
    pub centroids: CentroidParams,
    /// Configuration of automatic simulation stopping
    pub autostop: AutoStoppingParams,
    /// if true, the whole shape will be translated by displacement of root, so that root stays at (0, 0, 0).
    /// not applicable to LegacyPlushie
    pub keep_root_at_origin: bool,
    /// Multipler for BLO/FLO force. If BLO/FLO behaves incorrectly, probably the sign is wrong.
    /// I assume it has to do with working the plushie clockwise vs counterclockwise.
    /// It has yet to be investigated.
    pub single_loop_force: f32,
    // pub acceptable_displacement_for_adding_new_node: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AutoStoppingParams {
    /// Minimal tension at which the Plushie is considered relaxed
    pub acceptable_tension: f32,
    /// Hard limit on the relaxing process
    pub max_relaxing_iterations: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CentroidParams {
    /// Number of centroids that simulate the stuffing. More centroids = more internal pressure. Bigger shapes need more.
    pub number: usize,
    pub force: f32,
    pub min_nodes_per_centroid: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Initializer {
    OneByOne,
}

impl Params {
    fn unconstrained_floating() -> Self {
        Self {
            timestep: 1.0,
            autostop: Default::default(),
            centroids: Default::default(),
            floor: false,
            gravity: 0.0,
            desired_stitch_distance: 1.0,
            keep_root_at_origin: false,
            single_loop_force: 0.05,
        }
    }

    #[allow(unused)]
    fn rooted_floating() -> Self {
        const THIS_DEFAULT_IS_TRASH: f32 = 0.02;
        Self {
            keep_root_at_origin: true,
            ..Self::unconstrained_floating()
        }
    }

    fn floored() -> Self {
        Self {
            floor: true,
            gravity: 5e-4,
            keep_root_at_origin: true,
            ..Self::unconstrained_floating()
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::floored()
    }
}

impl Default for CentroidParams {
    fn default() -> Self {
        Self {
            number: 2,
            force: 0.05,
            min_nodes_per_centroid: 60,
        }
    }
}

impl Default for AutoStoppingParams {
    fn default() -> Self {
        Self {
            acceptable_tension: 0.02,
            max_relaxing_iterations: 100,
        }
    }
}
