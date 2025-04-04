// a:spcBef
use super::SpacingPercent;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct SpaceBefore {
    spacing_percent: Option<SpacingPercent>,
}

impl SpaceBefore {
    #[inline]
    pub fn get_spacing_percent(&self) -> Option<&SpacingPercent> {
        self.spacing_percent.as_ref()
    }

    #[inline]
    pub fn get_spacing_percent_mut(&mut self) -> Option<&mut SpacingPercent> {
        self.spacing_percent.as_mut()
    }

    #[inline]
    pub fn set_spacing_percent(&mut self, value: SpacingPercent) -> &mut Self {
        self.spacing_percent = Some(value);
        self
    }

    #[inline]
    pub fn remove_spacing_percent(&mut self) {
        self.spacing_percent = None;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:spcPct" {
                    let mut obj = SpacingPercent::default();
                    obj.set_attributes(reader, e);
                    self.set_spacing_percent(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:spcBef" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:spcBef")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:spcBef
        write_start_tag(writer, "a:spcBef", vec![], false);

        // a:spcPct
        if let Some(v) = &self.spacing_percent {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:spcBef");
    }
}
