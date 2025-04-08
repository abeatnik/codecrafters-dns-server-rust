use bytes::{ BufMut, BytesMut };

#[derive(Debug)]
pub struct DNSQuestion {
    name: Vec<String>, //labels
    r#type: u16, //0x0001
    class: u16, //0x0001
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

    pub fn new_atype_inclass(labels: Vec<String>) -> Self {
        Self {
            name : labels,
            r#type: 0x0001,
            class: 0x0001,
        }
    }
}
