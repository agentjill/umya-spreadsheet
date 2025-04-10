// oddFooter
use crate::reader::driver::*;
use crate::structs::StringValue;
use crate::writer::driver::*;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct OddFooter {
    value: StringValue,
}

impl OddFooter {
    #[inline]
    pub fn get_value(&self) -> &str {
        self.value.get_value_str()
    }

    #[inline]
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn _get_hash_code(&self) -> String {
        format!("{:x}", md5::Md5::digest(self.get_value()))
    }

    #[inline]
    pub(crate) fn has_param(&self) -> bool {
        self.value.has_value()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.set_value(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"oddFooter" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "oddFooter")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.has_param() {
            // oddFooter
            write_start_tag(writer, "oddFooter", vec![], false);
            write_text_node(writer, self.value.get_value_str());
            write_end_tag(writer, "oddFooter");
        }
    }
}
