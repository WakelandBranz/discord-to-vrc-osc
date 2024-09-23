// Required for OSC functionality
use rosc::OscType;

// Thank you https://github.com/SutekhVRC/VRCOSCExample

pub trait Data {
    fn send_data(&self, param_name: &str, param_arg: Vec<OscType>);
    fn recv_data(&self) -> Option<(String, Vec<OscType>)>;
}

/// Input traits for VRChat client
pub trait Input {
    /*
     * AXES
     */

    /// Forward and backward movement, more precise than input_move
    /// Takes f32 from -1 to 1
    fn input_vertical(&self, velocity: f32);

    /// Left and right movement, more precise than input_move
    /// Takes f32 from -1 to 1
    fn input_horizontal(&self, velocity: f32);

    /// Forward and backward movement for a held object
    /// Takes f32 from -1 to 1
    fn input_move_hold(&self, velocity: f32);

    /// Clockwise and counter-clockwise movement for a held object
    /// Takes f32 from -1 to 1
    fn input_spin_hold_cw(&self, velocity: f32);

    /// Up and down movement for a held object
    /// Takes f32 from -1 to 1
    fn input_spin_hold_vertical(&self, velocity: f32);

    /// Left and right movement for a held object
    /// Takes f32 from -1 to 1
    fn input_spin_hold_horizontal(&self, velocity: f32);

    /*
     * BUTTONS
     */

    /// Directions are 'Forward', 'Backward', 'Left', 'Right'
    fn input_move(&self, direction: &str, toggle: bool);

    /// Directions are 'Left' and 'Right'
    fn input_look(&self, direction: &str, toggle: bool);

    /// Jump takes i32 1 and 0 -> 1 is activated, 0 is reset
    fn input_jump(&self);

    /// Run takes i32 1 and 0 -> 1 is activated, 0 is inactive
    fn input_run(&self, toggle: i32);

    /// Takes inputs s b n
    /// s = chatbox text | can be sent as a raw string
    /// b = don't open keyboard (post straight to chatbox)
    /// n = don't play notification sound
    fn chatbox_message(&self, message: &str);
}

// Output traits for client (receiving data from surroundings)
pub trait Avatar {}