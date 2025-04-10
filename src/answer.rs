use bytes::{ Bytes, Buf, BufMut, BytesMut };
use crate::packet::DNSPacket;
use std::io::Cursor;

#[derive(Debug, Clone)]
enum RData {
    RDataARecord(u32),
}

#[derive(Debug, Clone)]
pub struct DNSAnswer {
    name: Vec<String>,
    r#type: u16, //0x0001
    class: u16, //0x0001
    ttl: u32,
    rd_length: u16,
    rdata: RData, //IPv4 address for "A" record type
}

impl DNSAnswer {
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::new();

        for label in self.name.iter() {
            let count = label.len() as u8;
            buf.put_u8(count);
            buf.put_slice(label.as_bytes());
        }

        let rdata = match self.rdata {
            RData::RDataARecord(value) => value,
        };

        buf.put_u8(0x0);
        buf.put_u16(self.r#type);
        buf.put_u16(self.class);
        buf.put_u32(self.ttl);
        buf.put_u16(self.rd_length);
        buf.put_slice(&rdata.to_be_bytes());

        buf
    }

    pub fn from_bytes_with_compression_advance_buffer(
        cursor: &mut Cursor<&mut Bytes>
    ) -> Option<Self> {
        let mut labels = Vec::<String>::new();

        let mut offset = cursor.position() as usize;
        let full_buf = cursor.get_ref();

        offset = DNSPacket::read_labels_at(full_buf, offset, &mut labels);

        cursor.set_position(offset as u64);

        Some(
            Self::new(
                labels,
                cursor.get_u16(),
                cursor.get_u16(),
                cursor.get_u32(),
                cursor.get_u16(),
                cursor.get_u32()
            )
        )
    }

    pub fn new(
        labels: Vec<String>,
        r#type: u16,
        class: u16,
        ttl: u32,
        rd_length: u16,
        rdata: u32
    ) -> Self {
        Self {
            name: labels,
            r#type,
            class,
            ttl,
            rd_length,
            rdata: RData::RDataARecord(rdata),
        }
    }
}
