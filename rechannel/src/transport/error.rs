use std::{error::Error, fmt, io};

use crate::error::RechannelError;
use renetcode::NetcodeError;

#[derive(Debug)]
pub enum NetcodeTransportError {
    Netcode(NetcodeError),
    Rechannel(RechannelError),
    IO(io::Error),
}

impl Error for NetcodeTransportError {}

impl fmt::Display for NetcodeTransportError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NetcodeTransportError::Netcode(ref err) => err.fmt(fmt),
            NetcodeTransportError::Rechannel(ref err) => err.fmt(fmt),
            NetcodeTransportError::IO(ref err) => err.fmt(fmt),
        }
    }
}

impl From<renetcode::NetcodeError> for NetcodeTransportError {
    fn from(inner: renetcode::NetcodeError) -> Self {
        NetcodeTransportError::Netcode(inner)
    }
}

impl From<renetcode::TokenGenerationError> for NetcodeTransportError {
    fn from(inner: renetcode::TokenGenerationError) -> Self {
        NetcodeTransportError::Netcode(renetcode::NetcodeError::TokenGenerationError(inner))
    }
}

impl From<RechannelError> for NetcodeTransportError {
    fn from(inner: RechannelError) -> Self {
        NetcodeTransportError::Rechannel(inner)
    }
}

impl From<std::io::Error> for NetcodeTransportError {
    fn from(inner: std::io::Error) -> Self {
        NetcodeTransportError::IO(inner)
    }
}
