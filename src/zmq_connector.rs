//! Runs a ZMQ client to receieve Rover information.
//!
//! Acts as an abstraction over the ZMQ setup so the UI is more focused towards
//! a good user experience.
pub mod zmq_connector {
    use tmq::{subscribe, subscribe::Subscribe, Context, Result};

    /// A wrapper for tmq's Context and Subscribe to use later.
    /// Allows for easier usage of tmq.
    pub struct ZmqClient {
        pub context: Option<Context>,
        pub socket: Option<Subscribe>,
    }

    /// Creates a new zmq (tmq) client.
    /// Waits for a successful connection.
    pub async fn start_zmq(address_string: String, port: i32) -> Result<ZmqClient> {
        

        let context = Context::new();
        let socket = subscribe(&context)
            .connect(format!("tcp://{address_string}:{port}").as_str())?
            .subscribe(b"topic")?;

        Ok(ZmqClient {
            context: Some(context),
            socket: Some(socket),
        })
    }
}
