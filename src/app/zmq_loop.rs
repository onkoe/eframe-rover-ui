use zmq_connector;

//! The hardcoded implementation of the ZMQ loop. 
//! 
//! Not pretty, but it should work. Designed to be modular/replacable. 
mod zmq_loop {
    /// Starts the ZMQ connection.
    pub fn start() {
        zmq_connector::start_zmq();
    }

    /// Terminates the ZMQ connection.
    pub fn stop() {

    }

    
}