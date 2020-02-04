use zmq::{self, Context as ZmqContext};

use crate::{comm::Receiver, poll::ZmqPoller};

/// Create a builder for a PULL socket.
pub fn pull(context: &ZmqContext) -> PullBuilder {
    PullBuilder::new(context)
}

impl_builder!(PULL, PullBuilder, PullBuilderBound);

pub struct PullBuilderBound {
    socket: zmq::Socket,
}

impl PullBuilderBound {
    pub fn finish(self) -> crate::Result<Pull> {
        Ok(Pull {
            inner: Receiver::new(ZmqPoller::from_zmq_socket(self.socket)?),
        })
    }
}

/// Asynchronous PULL socket.
pub struct Pull {
    inner: Receiver,
}
impl_wrapper!(Pull, Receiver, inner);
impl_wrapper_stream!(Pull, inner);
impl_buffered!(Pull, inner);
