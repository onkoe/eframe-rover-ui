//! Turns a ZMQ call into a jpeg image, as required by the current
//! implementation of the SoRo "video" protocols. It also calls
//! ZMQ itself. 
pub mod video {
    use image::*;
    use serde_json;
    use crate::zmq_connector::zmq_connector::{start_zmq, ZmqClient};
    //use zmq::{ Context, Socket, Message };
    use futures::StreamExt;
    use tmq::{subscribe, Context, Result};
    use std::env;

    pub async fn take_zmq_data() -> Option<DynamicImage> {
        //let mut client = crate::zmq_connector::zmq_connector::start_zmq("127.0.0.1".to_string(), 1234)?;
        
        let mut zmq_client = start_zmq("127.0.0.1".to_string(), 1234).await.unwrap();
        
        println!("client intialized [take_zmq_data()]");
        //loop {
            // let buffer = get_next_image; AS DYNAMICIMAGE
            // reset
            // go again until stopped

            if let Some(msg) = get_data(&mut zmq_client).await {
                return Some(image::load_from_memory(msg.as_bytes()).unwrap());
            }
            

        //}

        //Some(())
        None
    }

    // Attempts to turn input into a jpeg
    // 
    // but you know. for ZMQ...
    async fn get_data(zmq_client: &mut ZmqClient) -> Option<Vec<u8>> {
        let socket = &mut zmq_client.socket;

        if let Some(mut msg) = socket.next().await {
            match &mut msg {
                Ok(parts) => {let msg = parts;},
                Err(error) => return None,
            }

            let mut buf = Vec::new();
            for item in msg.unwrap().iter() {
                buf.extend_from_slice(item.as_bytes());
            }
            println!("Received data: {:?}", buf);
            return Some(buf);
        }

        None
    }

    // https://crates.io/crates/image
}