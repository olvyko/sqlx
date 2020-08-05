mod socket;

#[cfg(any(
    feature = "tokio-tls",
    feature = "async-std-tls",
))]
mod tls;

pub use socket::Socket;

#[cfg(any(
    feature = "tokio-tls",
    feature = "async-std-tls",
))]
pub use tls::MaybeTlsStream;
