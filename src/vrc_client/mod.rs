use crate::vrc_client::traits::Input;

pub mod traits;
pub mod client;

#[derive(Clone, Debug)]
pub struct Action {
    pub duration: u64,            // Duration to perform action
    pub movement: Option<String>, // Movement direction (Forward, Backward, Left, Right)
    pub look: Option<String>,     // Look direction (Left, Right)
    pub run: Option<bool>,        // Whether to run (True, False)
    pub jump: Option<bool>,       // Whether to jump (True, False)
}

