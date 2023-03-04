//! Turns a ZMQ call into a jpeg image, as required by the current
//! implementation of the SoRo "video" protocols. It also calls
//! ZMQ itself. 
pub mod video {
    use image::*;
    use serde_json;
    use crate::zmq_connector::zmq_connector::{start_zmq, ZmqClient};
    use zmq::{ Context, Socket, Message };

    pub fn take_zmq_data() -> Option<DynamicImage> {
        let mut client = crate::zmq_connector::zmq_connector::start_zmq("127.0.0.1".to_string(), 1234)?;
        println!("client intialized (takezmqdata)");
        //loop {
            // let buffer = get_next_image; AS DYNAMICIMAGE
            // reset
            // go again until stopped

            return Some(get_next_image(&mut client.socket, &mut client.context).unwrap());
            

        //}

        //Some(())
    }

    // Attempts to turn input into a jpeg
    // 
    // but you know. for ZMQ...
    fn get_next_image(socket: &mut Socket, _context: &mut Context) -> ImageResult<DynamicImage> {
        let message = socket.recv_msg(zmq::NOBLOCK).unwrap(); // TODO: get rid of this 
        println!("message found! or not.. idk");

        image::load_from_memory(message.as_bytes())
    }


    // https://crates.io/crates/image
    // pub fn 
}