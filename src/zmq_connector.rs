//! Runs a ZMQ client to receieve Rover information.
//!
//! Acts as an abstraction over the ZMQ setup so the UI is more focused towards
//! a good user experience.

pub mod zmq_connector {
    use tmq::{subscribe, Context, Result, subscribe::Subscribe};
    use zmq::{ /*Context,*/ Socket };
    use futures::StreamExt;

    pub struct ZmqClient {
        pub context: Context,
        pub socket: /*Socket*/Subscribe,
    } 

    /// Attempts to create a new subscriber.
    ///
    /// Returns a Result, along with subscriber if it's created. :)
    pub async fn start_zmq(address_string: String, port: i32) -> Result<ZmqClient> {
        //let address = format!("tcp://{}:{}", address_string, port).as_str();
        let context = Context::new();
        //let socket = context.socket(zmq::SUB);
        let mut socket = subscribe(&context)
            .connect(format!("tcp://{}:{}", address_string, port).as_str())?
            .subscribe(b"topic")?;

        Ok(ZmqClient { context: context, socket: socket })

        /*let socket = socket.unwrap(); // turns that result into a socket

        println!("unwrap to socket successful");
        socket.set_connect_timeout(4);
        
        return Some(ZmqClient {
            context: context,
            socket: socket,
        }); */
    }

}
