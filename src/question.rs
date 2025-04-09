use bytes::{ Buf, BufMut, BytesMut };
use std::{ io::Cursor };

#[derive(Debug)]
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

    pub fn from_bytes(mut buf: impl Buf) -> Self {
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

        Self::new_atype_inclass(labels)
    }

    pub fn from_bytes_with_compression_advance_buffer(cursor: &mut Cursor<&[u8]>) -> Self {
        let mut labels = Vec::<String>::new();

        let mut read_all_labels = false;

        'outer: while !read_all_labels {
            let pos = cursor.position() as usize;
            let byte = cursor.get_ref()[pos];
            let is_compressed = (byte >> 6) == 0x03;

            if !is_compressed {
                let len = cursor.get_u8();
                if len == 0 {
                    read_all_labels = true;
                    break 'outer;
                }
                let label_bytes = cursor.copy_to_bytes(len.into());
                let label = String::from_utf8_lossy(&label_bytes);
                labels.push(label.to_string());
            } else {
                let byte = cursor.get_ref()[pos];
                let offset = (((byte & 0x3f) as u16) << 6) | (cursor.get_ref()[pos + 1] as u16);
                cursor.set_position(cursor.position() + 2);
                let offset = offset as usize;
                let len = cursor.get_ref()[offset];
                let label_bytes = &cursor.get_ref()[offset + 1..offset + 1 + (len as usize)];
                let label = String::from_utf8_lossy(label_bytes);
                labels.push(label.to_string());
                break 'outer;
            }
        }
        Self::new_atype_inclass(labels)
    }
}
