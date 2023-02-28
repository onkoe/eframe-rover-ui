//! Turns a ZMQ call into a jpeg image, as required by the current
//! implementation of the SoRo "video" protocols. It also calls
//! ZMQ itself. 
mod video {
    use image::*;
    use serde_json;
    use zmq_connector;

    // Attempts to turn input into a jpeg
    // 
    // but you know. for ZMQ...
    fn grab_json() {
        
    }

    // https://crates.io/crates/image
    // pub fn 
}