use bytes::{ BufMut, BytesMut };

pub struct DNSHeader {
    id: u16,
    flags: DNSFlags,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(12);

        buf.put_u16(self.id);
        buf.put_u16(self.flags.construct_flag_bits());
        buf.put_u16(self.qd_count);
        buf.put_u16(self.an_count);
        buf.put_u16(self.ns_count);
        buf.put_u16(self.ar_count);

        buf
    }

    pub fn new(
        id: u16,
        flags: DNSFlags,
        qd_count: u16,
        an_count: u16,
        ns_count: u16,
        ar_count: u16
    ) -> Self {
        Self {
            id,
            flags,
            qd_count,
            an_count,
            ns_count,
            ar_count,
        }
    }
}

pub struct DNSFlags {
    qr: bool,
    opcode: u8, //will become 4 bits later, so max is 0xF
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8, //will become 3 bits later, so max is 0x7
    rcode: u8, //will become 4 bits later, so max is 0xF
}

impl DNSFlags {
    pub fn new(
        qr: bool,
        opcode: u8,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        z: u8,
        rcode: u8
    ) -> Self {
        Self {
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
        }
    }
    pub fn construct_flag_bits(&self) -> u16 {
        ((self.qr as u16) << 15) |
            (((self.opcode as u16) & 0xf) << 11) | // 11-14
            ((self.aa as u16) << 10) |
            ((self.tc as u16) << 9) |
            ((self.rd as u16) << 8) |
            ((self.ra as u16) << 7) |
            (((self.z as u16) & 0x7) << 4) | //  4-6
            ((self.rcode as u16) & 0xf) // 0-4
    }
}
