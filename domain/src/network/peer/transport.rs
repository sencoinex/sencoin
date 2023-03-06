use crate::network::{NetworkAddress, PeerOperatorId};
use futures::{future::Future, stream::Stream};

/// A Transport is responsible for establishing connections with remote Peers.
///
/// Connections are established either by [listening](Transport::listen_on)
/// or [dialing](Transport::dial) on a [`Transport`]. A peer that
/// obtains a connection by listening is often referred to as the *listener* and the
/// peer that initiated the connection through dialing as the *dialer*.
pub trait Transport {
    /// The result of establishing a connection.
    ///
    /// Generally this would include a socket-like streams which allows for sending and receiving
    /// of data through the connection.
    type Output;

    /// The Error type of errors which can happen while establishing a connection.
    type Error: ::std::error::Error + Send + Sync + 'static;

    /// A stream of [`Inbound`](Transport::Inbound) connections and the address of the dialer.
    ///
    /// An item should be produced whenever a connection is received at the lowest level of the
    /// transport stack. Each item is an [`Inbound`](Transport::Inbound) future
    /// that resolves to an [`Output`](Transport::Output) value once all protocol upgrades
    /// have been applied.
    type Listener: Stream<Item = Result<(Self::Inbound, NetworkAddress), Self::Error>>
        + Send
        + Unpin;

    /// A pending [`Output`](Transport::Output) for an inbound connection,
    /// obtained from the [`Listener`](Transport::Listener) stream.
    ///
    /// After a connection has been accepted by the transport, it may need to go through
    /// asynchronous post-processing (i.e. protocol upgrade negotiations). Such
    /// post-processing should not block the `Listener` from producing the next
    /// connection, hence further connection setup proceeds asynchronously.
    /// Once a `Inbound` future resolves it yields the [`Output`](Transport::Output)
    /// of the connection setup process.
    type Inbound: Future<Output = Result<Self::Output, Self::Error>> + Send;

    /// A pending [`Output`](Transport::Output) for an outbound connection,
    /// obtained from [dialing](Transport::dial) stream.
    type Outbound: Future<Output = Result<Self::Output, Self::Error>> + Send;

    /// Listens on the given [`NetworkAddress`], returning a stream of incoming connections.
    ///
    /// The returned [`NetworkAddress`] is the actual listening address, this is done to take into
    /// account OS-assigned port numbers (e.g. listening on port 0).
    fn listen_on(
        &self,
        addr: NetworkAddress,
    ) -> Result<(Self::Listener, NetworkAddress), Self::Error>
    where
        Self: Sized;

    /// Dials the given [`NetworkAddress`], returning a future for a pending outbound connection.
    fn dial(
        &self,
        peer_id: PeerOperatorId,
        addr: NetworkAddress,
    ) -> Result<Self::Outbound, Self::Error>
    where
        Self: Sized;
}
