pub mod actions;
pub mod human_flow;
pub mod simple_flow;

#[allow(unused)]
use self::actions::Action;

pub trait Flow {
    fn next(&mut self) -> Option<Action>;
}
