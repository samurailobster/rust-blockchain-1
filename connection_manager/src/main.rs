#![deny(missing_docs)]

//! Manages connected peers
//!
//! - Handles hole punching between to peers
//! - Decides when and what comes into the next block (not yet implemented)
//! - Decides when a block is written (not yet implemented)

extern crate blockchain_file;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;

mod hole_puncher;

use blockchain_network::event::EventHandler;
use blockchain_network::udp_client::UdpClientBuilder;

/// Starting point
///
/// Registers all needed event handlers and starts a UDP-Listener
fn main() {
    info!("Starting hole puncher!");

    let event_handlers = EventHandler::new()
        .set_register_handler(hole_puncher::register_handler);

    UdpClientBuilder::new()
        .set_port(45000)
        .build(event_handlers)
        .listen();
}