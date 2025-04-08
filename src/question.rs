use bytes::{ Buf, BufMut, BytesMut };

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
}
