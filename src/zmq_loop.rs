//! The hardcoded implementation of the ZMQ loop.
//!
//! Not pretty, but it should work. Designed to be "modular"/replacable.
pub mod zmq_loop {
    use crate::zmq_connector::{ zmq_connector::start_zmq, zmq_connector::ZmqClient };
    use zmq::{ Context, Socket };

    /// Tries to start ZMQ. If it works, it'll get data until stopped.
    pub fn start(address_string: String, port: i32) -> Option<ZmqClient> {
        let subscriber = start_zmq(address_string, port);

        // wait until subscriber gets data

        // when it does, return 
    }

    /// Terminates the ZMQ connection.
    pub fn stop(subscriber: ZmqClient) -> Option<()> {
        None
    }
}
