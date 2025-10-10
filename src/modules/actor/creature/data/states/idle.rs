prelude!();

use crate::modules::dev::StateDebug;
use std::fmt::Write;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct CreatureStateIdle;

impl StateDebug for CreatureStateIdle {
    fn debug(&self) -> String {
        let mut output = String::new();
        output.write_str("State: Idle").ok();

        output
    }
}
