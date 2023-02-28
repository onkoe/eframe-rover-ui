//! Runs a ZMQ client to receieve Rover information.
//!
//! Acts as an abstraction over the ZMQ setup so the UI is more focused towards
//! a good user experience.

mod zmq_connector {
    use core::time::Duration;
    use libzmq::{prelude::*, Client, ClientBuilder, Error, Period, TcpAddr};

    /// Attempts to connect to a client using a provided IP. Returns Result.
    ///
    /// TcpAddr includes port.
    fn connect(address: TcpAddr) -> Result<Client, Error<()>> {
        println!("Attempting to connect on {}...", address);

        let client = ClientBuilder::new().connect(address).build();

        return client;
    }

    /// Attempts to create a new subscriber.
    ///
    /// Returns a Result, along with subscriber if it's created. :)
    pub fn start_zmq(address_string: String) -> Result<Client, Error> {
        let address: TcpAddr = address_string.try_into()?;
        let subscriber = connect(address)?;

        subscriber.set_recv_timeout(Period::Finite(Duration::new(2, 0)))?;

        return Ok(subscriber);
    }

}
