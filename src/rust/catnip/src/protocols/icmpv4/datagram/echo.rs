use super::{
    Icmpv4Datagram, Icmpv4DatagramMut, Icmpv4Type, ICMPV4_HEADER_SIZE,
};
use crate::prelude::*;
use byteorder::{ByteOrder, NetworkEndian};
use std::convert::TryFrom;

pub enum Icmpv4EchoType {
    Request,
    Reply,
}

pub struct Icmpv4Echo<'a>(Icmpv4Datagram<'a>);

impl<'a> Icmpv4Echo<'a> {
    pub fn icmpv4(&self) -> &Icmpv4Datagram<'a> {
        &self.0
    }

    pub fn r#type(&self) -> Icmpv4EchoType {
        match self.0.header().r#type().unwrap() {
            Icmpv4Type::EchoRequest => Icmpv4EchoType::Request,
            Icmpv4Type::EchoReply => Icmpv4EchoType::Reply,
        }
    }

    pub fn id(&self) -> u16 {
        NetworkEndian::read_u16(&self.0.payload()[..2])
    }

    pub fn seq_num(&self) -> u16 {
        NetworkEndian::read_u16(&self.0.payload()[2..4])
    }
}

impl<'a> TryFrom<Icmpv4Datagram<'a>> for Icmpv4Echo<'a> {
    type Error = Fail;

    fn try_from(datagram: Icmpv4Datagram<'a>) -> Result<Self> {
        let r#type = datagram.header().r#type()?;
        assert!(
            r#type == Icmpv4Type::EchoRequest
                || r#type == Icmpv4Type::EchoReply
        );
        Ok(Icmpv4Echo(datagram))
    }
}

pub struct Icmpv4EchoMut<'a>(Icmpv4DatagramMut<'a>);

impl<'a> Icmpv4EchoMut<'a> {
    pub fn new_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.resize(ICMPV4_HEADER_SIZE, 0);
        bytes
    }

    pub fn from_bytes(bytes: &'a mut [u8]) -> Result<Self> {
        Ok(Icmpv4EchoMut(Icmpv4DatagramMut::from_bytes(bytes)?))
    }

    pub fn icmpv4(&mut self) -> &mut Icmpv4DatagramMut<'a> {
        &mut self.0
    }

    pub fn r#type(&mut self, value: Icmpv4EchoType) {
        match value {
            Icmpv4EchoType::Request => {
                self.0.header().r#type(Icmpv4Type::EchoRequest)
            }
            Icmpv4EchoType::Reply => {
                self.0.header().r#type(Icmpv4Type::EchoReply)
            }
        }
    }

    pub fn id(&mut self, value: u16) {
        NetworkEndian::write_u16(&mut self.0.payload()[..2], value)
    }

    pub fn seq_num(&mut self, value: u16) {
        NetworkEndian::write_u16(&mut self.0.payload()[2..4], value)
    }

    #[allow(dead_code)]
    pub fn unmut(self) -> Result<Icmpv4Echo<'a>> {
        Ok(Icmpv4Echo::try_from(self.0.unmut()?)?)
    }

    pub fn seal(self) -> Result<Icmpv4Echo<'a>> {
        Ok(Icmpv4Echo::try_from(self.0.seal()?)?)
    }
}

impl<'a> From<Icmpv4DatagramMut<'a>> for Icmpv4EchoMut<'a> {
    fn from(datagram: Icmpv4DatagramMut<'a>) -> Self {
        Icmpv4EchoMut(datagram)
    }
}
