//! Turns a ZMQ call into a jpeg image, as required by the current
//! implementation of the SoRo "video" protocols. It also calls
//! ZMQ itself.
pub mod video {
    use crate::zmq_connector::zmq_connector::{start_zmq, ZmqClient};
    use futures::StreamExt;
    use image::*;

    pub async fn take_zmq_data(zmq_client: &mut ZmqClient) -> Option<DynamicImage> {
        println!("client intialized [take_zmq_data()]");
        //loop {
        // let buffer = get_next_image; AS DYNAMICIMAGE
        // reset
        // go again until stopped

        if let Some(msg) = get_data(zmq_client).await {
            return Some(image::load_from_memory(msg.as_bytes()).unwrap());
        }

        //}

        //Some(())
        None
    }

    /// Returns tmq data as a vector of bytes if it gets it.
    async fn get_data(zmq_client: &mut ZmqClient) -> Option<Vec<u8>> {
        // Check to see if socket is real or fake
        match &mut zmq_client.socket {
            None => {
                return None;
            }
            Some(data) => {
                let socket = data;

                // wait for the data, then put it into a u8 vector
                if let Some(mut msg) = socket.next().await {
                    match &mut msg {
                        Ok(parts) => {
                            let msg = parts;
                        }
                        Err(error) => return None,
                    }

                    let mut buf = Vec::new();
                    for item in msg.unwrap().iter() {
                        buf.extend_from_slice(item.as_bytes());
                    }
                    println!("Received data: {:?}", buf);
                    return Some(buf);
                }
            }
        }

        None
    }

    // https://crates.io/crates/image
}
