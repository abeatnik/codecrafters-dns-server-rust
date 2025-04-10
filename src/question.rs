use anyhow::Error;
use bytes::{ Bytes, Buf, BufMut, BytesMut };
use std::io::Cursor;

use crate::packet::DNSPacket;

#[derive(Debug, Clone)]
pub struct DNSQuestion {
    pub name: Vec<String>, //labels
    pub r#type: u16, //0x0001
    pub class: u16, //0x0001
}

impl DNSQuestion {
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::new();

        for label in self.name.iter() {
            let count = label.len() as u8;
            buf.put_u8(count);
            buf.put_slice(label.as_bytes());
        }
        buf.put_u8(0x0);
        buf.put_u16(self.r#type);
        buf.put_u16(self.class);

        buf
    }

    pub fn get_byte_len(&self) -> usize {
        let size = self.name.len() + 1;
        let len: usize = self.name
            .clone()
            .into_iter()
            .map(|x| x.len())
            .sum();
        size + len + 4
    }

    pub fn new_atype_inclass(labels: Vec<String>) -> Self {
        Self {
            name: labels,
            r#type: 0x0001,
            class: 0x0001,
        }
    }

    pub fn from_bytes(mut buf: impl Buf) -> Result<Self, Error> {
        let mut labels = Vec::<String>::new();

        let mut read_all_labels = false;

        while !read_all_labels {
            let size = buf.get_u8();
            if size == 0 {
                read_all_labels = true;
                break;
            }

            let label_bytes = buf.copy_to_bytes(size.into());
            let label = String::from_utf8_lossy(&label_bytes);
            labels.push(label.to_string());
        }
        buf.get_u64();

        Ok(Self::new_atype_inclass(labels))
    }

    pub fn from_bytes_with_compression_advance_buffer(
        cursor: &mut Cursor<&mut Bytes>
    ) -> Result<Self, Error> {
        let mut labels = Vec::<String>::new();

        let mut offset = cursor.position() as usize;
        let full_buf = cursor.get_ref();

        offset = DNSPacket::read_labels_at(full_buf, offset, &mut labels);

        cursor.set_position(offset as u64);

        if cursor.remaining() >= 4 {
            cursor.advance(4);
        }

        Ok(Self::new_atype_inclass(labels))
    }
}
