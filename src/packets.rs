use pnet::packet::tcp::TcpPacket as PnetTcpPacket;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Invalid buffer")]
    InvalidBuffer
}

pub struct TcpPacket<'a> {
    packet: PnetTcpPacket<'a>
}

impl<'a> TryFrom<&'a [u8]> for TcpPacket<'a> {
    type Error = PacketError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let packet =  match PnetTcpPacket::new(value) {
            Some(packet) => packet,
            None => return Err(PacketError::InvalidBuffer),
        };

        let tcp_packet = TcpPacket {
            packet
        };

        Ok(tcp_packet)
    }
}