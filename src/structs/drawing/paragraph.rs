// a:p
use super::ParagraphProperties;
use super::Run;
use super::RunProperties;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct Paragraph {
    paragraph_properties: ParagraphProperties,
    run: ThinVec<Run>,
    end_para_run_properties: Option<Box<RunProperties>>,
}

impl Paragraph {
    #[inline]
    pub fn get_paragraph_properties(&self) -> &ParagraphProperties {
        &self.paragraph_properties
    }

    #[inline]
    pub fn get_paragraph_properties_mut(&mut self) -> &mut ParagraphProperties {
        &mut self.paragraph_properties
    }

    #[inline]
    pub fn set_paragraph_properties(&mut self, value: ParagraphProperties) -> &mut Paragraph {
        self.paragraph_properties = value;
        self
    }

    #[inline]
    pub fn get_run(&self) -> &[Run] {
        &self.run
    }

    #[inline]
    pub fn add_run(&mut self, value: Run) {
        self.run.push(value);
    }

    #[inline]
    pub fn get_end_para_run_properties(&self) -> Option<&RunProperties> {
        self.end_para_run_properties.as_deref()
    }

    #[inline]
    pub fn get_end_para_run_properties_mut(&mut self) -> Option<&mut RunProperties> {
        self.end_para_run_properties.as_deref_mut()
    }

    #[inline]
    pub fn set_end_para_run_properties(&mut self, value: RunProperties) -> &mut Paragraph {
        self.end_para_run_properties = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn remove_end_para_run_properties(&mut self) {
        self.end_para_run_properties = None;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:pPr" => {
                        self.paragraph_properties.set_attributes(reader, e, false);
                    }
                    b"a:r" => {
                        let mut run = Run::default();
                        run.set_attributes(reader, e);
                        self.add_run(run);
                    }
                    b"a:endParaRPr" => {
                        let mut run_properties = RunProperties::default();
                        run_properties.set_attributes(reader, e, false);
                        self.set_end_para_run_properties(run_properties);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:pPr" => {
                        self.paragraph_properties.set_attributes(reader, e, true);
                    }
                    b"a:endParaRPr" => {
                        let mut run_properties = RunProperties::default();
                        run_properties.set_attributes(reader, e, true);
                        self.set_end_para_run_properties(run_properties);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:p" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:p")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:p
        write_start_tag(writer, "a:p", vec![], false);

        // a:pPr
        self.paragraph_properties.write_to(writer);

        // a:r
        for run in &self.run {
            run.write_to(writer);
        }

        // a:endParaRPr
        if let Some(v) = &self.end_para_run_properties {
            v.write_to_end_para_rpr(writer);
        }

        write_end_tag(writer, "a:p");
    }
}
