use anyhow::Error;
use bytes::{ Bytes, Buf, BufMut, BytesMut };
use std::io::Cursor;

use crate::header::DNSHeader;
use crate::question::DNSQuestion;
use crate::answer::DNSAnswer;

#[derive(Debug)]
pub struct DNSPacket {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSAnswer>,
}

impl DNSPacket {
    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::new();

        buf.put(self.header.to_bytes());

        for question in self.questions.clone() {
            buf.put(question.to_bytes());
        }
        for answer in self.answers.clone() {
            buf.put(answer.to_bytes());
        }
        buf
    }

    pub fn merge(packets: Vec<DNSPacket>) -> Self {
        let mut header: DNSHeader = packets[0].header.clone();
        let mut questions = Vec::<DNSQuestion>::new();
        let mut answers = Vec::<DNSAnswer>::new();

        for packet in packets {
            questions.extend(packet.questions);
            answers.extend(packet.answers);
        }
        header.qd_count = answers.len() as u16;
        header.an_count = answers.len() as u16;
        Self { header, questions, answers }
    }

    //helper for question and answer section
    pub fn read_labels_at(cursor_bytes: &[u8], offset: usize, labels: &mut Vec<String>) -> usize {
        let mut pos = offset;
        loop {
            let byte = cursor_bytes[pos];
            let is_compressed = (byte >> 6) == 0x03;
            if is_compressed {
                let pointer = (((byte & 0x3f) as usize) << 8) | (cursor_bytes[pos + 1] as usize);
                DNSPacket::read_labels_at(cursor_bytes, pointer, labels);
                return pos + 2;
            } else if byte == 0 {
                return pos + 1;
            } else {
                let len = byte as usize;
                pos += 1;
                let label = &cursor_bytes[pos..pos + len];
                labels.push(String::from_utf8_lossy(label).to_string());
                pos += len;
            }
        }
    }

    pub fn from_bytes(bytes: &mut Bytes) -> Result<Self, Error> {
        let mut cursor = Cursor::new(bytes);
        let header = DNSHeader::from_bytes(&mut cursor)?;
        let mut questions: Vec<DNSQuestion> = vec![];
        let mut answers: Vec<DNSAnswer> = vec![];

        for _ in 0..header.qd_count {
            if cursor.remaining() >= 6 {
                let question = DNSQuestion::from_bytes_with_compression_advance_buffer(
                    &mut cursor
                )?;
                questions.push(question);
            }
        }

        for _ in 0..header.an_count {
            if cursor.remaining() >= 8 {
                match DNSAnswer::from_bytes_with_compression_advance_buffer(&mut cursor) {
                    Some(answer) => answers.push(answer),
                    None => println!("no answer in dns packet"),
                }
            }
        }

        Ok(DNSPacket {
            header,
            questions,
            answers,
        })
    }
}
