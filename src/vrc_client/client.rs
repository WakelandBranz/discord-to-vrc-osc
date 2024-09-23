// Import the rosc library
extern crate rosc;

// rosc encoder
use rosc::encoder;
// rosc types
use rosc::{OscMessage, OscPacket, OscType};
// Import from traits.rs
use crate::vrc_client::traits::{Data, Input, Avatar};

use log::{debug, info, warn, error};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::{thread, time};
use tokio::time::sleep as t_sleep; // TODO: Fix this awful practice

fn sleep (ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}
// TODO: Also fix this awful thing

/// Capitalizes the first letter of a string and makes everything else lowercase
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
    }
}

#[derive(Debug)]
pub struct Client {
    pub receive_addr: SocketAddrV4,
    pub receive_addr_str: String,
    pub transmit_addr: SocketAddrV4,
    pub transmit_addr_str: String,
    pub sock: UdpSocket,

    // Some future ideas:
    // - Implement velocity tracking in a mutex so that multiple threads can access it
    // - Modify velocity by intercepting when it changes and then hardcoding it to be 3.0 (max velocity in xyz)

    // Movement recorder
}

impl Data for Client {
    fn send_data(&self, param_name: &str, param_arg: Vec<OscType>) {
        // Create OSC/1.0 Message buffer with parameter name and parameter value/arg
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: param_name.to_string(),
            args: param_arg,
        }))
            .unwrap();

        // Send the encoded Message buffer to VRChat on the specified port (default 9000)
        // send_to requires a String as its address:port
        self.sock.send_to(&msg_buf, self.transmit_addr_str.clone()).unwrap();
    }

    // Reads the next available OSC packet from specified rx port
    fn recv_data(&self) -> Option<(String, Vec<OscType>)> {

        // Create/allocate buffer on the stack with a size of MTU
        let mut buf = [0u8; rosc::decoder::MTU];

        // Set the socket to non-blocking mode
        self.sock.set_nonblocking(true).unwrap();

        /*
            Receive OSC data length in var "buffer_len". Address of origin data in "a".
            Write the data received to the buffer on the stack "buf".
        */
        let (buffer_len, _addr) = match self.sock.recv_from(&mut buf) {
            Ok((buffer_len, _addr)) => (buffer_len, _addr),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available, continue the loop
                return None;
            }
            Err(e) => {
                // Handle other errors
                error!("Error receiving data: {}", e);
                return None;
            }
        };
        /*
            Check that the packet is greater than 0 bytes.
            If the packet length is <= 0, the recv loop is restarted.
            The received buffer is then decoded and parsed.
            If the decoded packet "pkt" is of OSC type Message,
            the OSC address and OSC args are printed to the CLI.
        */
        if buffer_len <= 0 { None }
        else {
            let pkt = match rosc::decoder::decode_udp(&buf) {
                Ok(pkt) => pkt,
                Err(_e) => {
                    error!("{}", "!!! Invalid OSC buffer !!!");
                    panic!("Failed to read OSC buffer")
                },
            };
            match pkt.1 {
                OscPacket::Message(msg) => {
                    //debug!("OSC -> Address: {}\nArgument(s): {:?}", msg.addr, msg.args);
                    Some((msg.addr, msg.args))
                },
                _ => { None }
            }
        }
    }
}

impl Input for Client {
    /*
     * AXES
     */

    // Forward and backward movement, more precise than input_move
    // vertical takes f32 from -1 to 1
    fn input_vertical(&self, velocity: f32) {
        let param_name: String = "/input/Vertical".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // Left and right movement, more precise than input_move
    // horizontal takes f32 from -1 to 1
    fn input_horizontal(&self, velocity: f32) {
        let param_name: String = "/input/Horizontal".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // Forward and backward movement for a held object
    // Takes f32 from -1 to 1
    fn input_move_hold(&self, velocity: f32) {
        let param_name: String = "/input/MoveHoldFB".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // Clockwise and counter-clockwise movement for a held object
    // Takes f32 from -1 to 1
    fn input_spin_hold_cw(&self, velocity: f32) {
        let param_name: String = "/input/SpinHoldCwCcw".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // Up and down movement for a held object
    // Takes f32 from -1 to 1
    fn input_spin_hold_vertical(&self, velocity: f32) {
        let param_name: String = "/input/SpinHoldUD".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // Left and right movement for a held object
    // Takes f32 from -1 to 1
    fn input_spin_hold_horizontal(&self, velocity: f32) {
        let param_name: String = "/input/SpinHoldLR".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    /*
     * BUTTONS
     */

    // Directions are 'Forward', 'Backward', 'Left', 'Right'
    fn input_move(&self, direction: &str, toggle: bool) {
        let param_name: String = format!("/input/Move{}", capitalize(direction));
        let param_arg: OscType = OscType::Bool(toggle);
        self.send_data(&param_name, vec![param_arg])
    }

    // Directions are 'Left' and 'Right'
    fn input_look(&self, direction: &str, toggle: bool) {
        let param_name: String = format!("/input/Look{}", capitalize(direction));
        let param_arg: OscType = OscType::Bool(toggle);
        self.send_data(&param_name, vec![param_arg])
    }

    // Jump takes ints 1 and 0 -> 1 is activated, 0 is reset
    fn input_jump(&self) {
        let param_name: String = "/input/Jump".to_string();
        self.send_data(&param_name, vec![OscType::Int(1)]); // Activate jump
        sleep(10); // Required sleep time for "keypresses" to register
        self.send_data(&param_name, vec![OscType::Int(0)]) // Reset jump
    }

    // Run takes ints 1 and 0 -> 1 is activated, 0 is inactive
    fn input_run(&self, toggle: i32) {
        let param_name: String = "/input/Run".to_string();
        let param_arg: OscType = OscType::Int(toggle);
        self.send_data(&param_name, vec![param_arg]) // 1 = running | 0 = walking
    }

    // Takes inputs s b n
    // s = chatbox text | can be sent as a raw string
    // b = don't open keyboard (post straight to chatbox)
    // n = don't play notification sound
    fn chatbox_message(&self, message: &str) {
        let verified_message: &str = &message;

        debug!("Sent '{}'", &verified_message);

        let param_name: &str = "/chatbox/input";
        let param_arg: Vec<OscType> = vec![
            OscType::String(verified_message.to_string()), // Chatbox text
            OscType::Bool(true), // Don't open keyboard (post straight to chatbox)
            OscType::Bool(false)]; // Don't play notification sound
        self.send_data(param_name, param_arg)
    }
}

impl Client {
    // Requires two ports to bind to, first is the reception port, second is the query port
    pub fn new(receiver_port: u16, transmitter_port: u16) -> Self {

        // Always listen to port 9001 by default
        // Always query to port 9000 by default
        let receive_addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), receiver_port);
        let transmit_addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), transmitter_port);

        let socket = match UdpSocket::bind(&receive_addr) {
            Ok(success) =>  {
                debug!("Successfully bound to {:?}", &receive_addr);
                success
            }
            Err(e) =>  {
                error!("Failed to bind to {:?}, is your VRChat client open? Is the port already occupied?", &receive_addr);
                panic!("Error: {:?}", e);
            }
        };

        debug!("Binding to {} | Info will be sent to {}", &receive_addr, &transmit_addr);

        Client {
            receive_addr,
            transmit_addr,
            receive_addr_str: format!("{}:{}", receive_addr.ip(), receive_addr.port()),
            transmit_addr_str: format!("{}:{}", transmit_addr.ip(), transmit_addr.port()),
            sock: socket,
        }
    }

    pub fn test_socket(&self) -> bool {
        todo!();
    }

    // Ensure that you can run and jump before moving. The necessity of this is uncertain.
    pub fn input_button_init(&self) {
        self.send_data("/input/Jump", vec![OscType::Int(0)]); // Initialize jump to 0
        sleep(10);
        self.send_data("/input/Run", vec![OscType::Int(0)]); // Initialize run to 0

        debug!("Initialized jump and run inputs")
    }

    // This is all super hardcoded. It's just a demonstration, and it's kind of cool in public lobbies.
    pub fn input_test(&self) {
        // Moving left/right for 1750ms ~= 360 degrees

        debug!("!!! Check OSC debug menu to ensure that these actions are functional !!!");

        self.input_button_init(); // Ensure that you can run and jump before moving

        self.chatbox_message("calibrating movement..."); // No, it does not actually calibrate movement.

        sleep(500);

        self.chatbox_message("calibrating movement -> Left");
        // 360 degrees left
        self.input_look("Left", true);
        self.input_move("Forward", true);
        sleep(1775);
        self.input_move("Forward", false);
        self.input_look("Left", false);

        self.chatbox_message("calibrating movement -> Right");
        // 360 degrees right
        self.input_look("Right", true);
        self.input_move("Backward", true);
        sleep(1775);
        self.input_move("Backward", false);
        self.input_look("Right", false);

        self.chatbox_message("calibrating movement -> Forward");
        self.input_run(1);
        self.input_vertical(0.5);
        sleep(1000);
        self.input_vertical(0.0);
        self.input_run(0);

        self.chatbox_message("calibrating movement -> Backward");
        self.input_run(1);
        self.input_vertical(-0.5);
        sleep(1000);
        self.input_vertical(0.0);
        self.input_run(0);

        sleep(100);
        self.chatbox_message("calibrating movement -> Jumping");
        self.input_jump();
        sleep(500);
        self.input_jump();
        sleep(500);
        self.input_jump();

        self.chatbox_message("bless up 🙏");

        sleep(3000);
    }

    // Cool spinning effect for picked up items
    pub fn input_rotate_axis_left(&self) {
        // Spacing in timing as to not exceed rate limit
        self.input_spin_hold_cw(-0.5);

        self.input_spin_hold_vertical(-0.5);

        self.input_spin_hold_horizontal(-0.5);
    }

    // Cool spinning effect for picked up items
    pub fn input_rotate_axis_right(&self) {
        // Spacing in timing as to not exceed rate limit
        self.input_spin_hold_cw(0.5);

        self.input_spin_hold_vertical(0.5);

        self.input_spin_hold_horizontal(0.5);
    }
}

