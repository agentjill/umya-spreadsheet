use crate::reader::driver::*;
use crate::structs::TrueFalseBlankValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ResizeWithCells {
    value: TrueFalseBlankValue,
}

impl ResizeWithCells {
    #[inline]
    pub fn get_value(&self) -> Option<&bool> {
        self.value.get_value()
    }

    #[inline]
    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.value.set_value_string(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"x:SizeWithCells" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "x:SizeWithCells")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:SizeWithCells
        if self.value.has_value() {
            write_start_tag(writer, "x:SizeWithCells", vec![], false);
            write_text_node(writer, self.value.get_value_string2());
            write_end_tag(writer, "x:SizeWithCells");
        } else {
            write_start_tag(writer, "x:SizeWithCells", vec![], true);
        }
    }
}
