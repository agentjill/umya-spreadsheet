use super::EmbeddedObjectProperties;
use super::StringValue;
use crate::helper::const_str::MC_NS;
use crate::reader::driver::*;
use crate::structs::drawing::spreadsheet::TwoCellAnchor;
use crate::structs::raw::RawRelationships;
use crate::structs::vml::Shape;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct OleObject {
    requires: StringValue,
    prog_id: StringValue,
    object_extension: Box<str>,
    object_data: Option<ThinVec<u8>>,
    embedded_object_properties: EmbeddedObjectProperties,
    two_cell_anchor: TwoCellAnchor,
    shape: Shape,
}

impl OleObject {
    #[inline]
    pub fn get_requires(&self) -> &str {
        self.requires.get_value_str()
    }

    #[inline]
    pub fn set_requires<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.requires.set_value(value);
        self
    }

    #[inline]
    pub fn get_prog_id(&self) -> &str {
        self.prog_id.get_value_str()
    }

    #[inline]
    pub fn set_prog_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prog_id.set_value(value);
        self
    }

    #[inline]
    pub fn get_object_extension(&self) -> &str {
        &self.object_extension
    }

    #[inline]
    pub fn set_object_extension<S: Into<String>>(&mut self, value: S) {
        self.object_extension = value.into().into_boxed_str();
    }

    #[inline]
    pub fn get_object_data(&self) -> Option<&[u8]> {
        self.object_data.as_deref()
    }

    #[inline]
    pub fn get_object_data_mut(&mut self) -> Option<&mut ThinVec<u8>> {
        self.object_data.as_mut()
    }

    #[inline]
    pub fn set_object_data(&mut self, value: impl Into<ThinVec<u8>>) -> &mut Self {
        self.object_data = Some(value.into());
        self
    }

    #[inline]
    pub fn get_embedded_object_properties(&self) -> &EmbeddedObjectProperties {
        &self.embedded_object_properties
    }

    #[inline]
    pub fn get_embedded_object_properties_mut(&mut self) -> &mut EmbeddedObjectProperties {
        &mut self.embedded_object_properties
    }

    #[inline]
    pub fn set_embedded_object_properties(&mut self, value: EmbeddedObjectProperties) -> &mut Self {
        self.embedded_object_properties = value;
        self
    }

    #[inline]
    pub fn get_two_cell_anchor(&self) -> &TwoCellAnchor {
        &self.two_cell_anchor
    }

    #[inline]
    pub fn get_two_cell_anchor_mut(&mut self) -> &mut TwoCellAnchor {
        &mut self.two_cell_anchor
    }

    #[inline]
    pub fn set_two_cell_anchor(&mut self, value: TwoCellAnchor) -> &mut Self {
        self.two_cell_anchor = value;
        self
    }

    #[inline]
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    #[inline]
    pub fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    #[inline]
    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = value;
        self
    }

    #[inline]
    pub(crate) fn is_bin(&self) -> bool {
        &*self.object_extension == "bin"
    }

    #[inline]
    pub(crate) fn is_xlsx(&self) -> bool {
        &*self.object_extension == "xlsx"
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        relationships: &RawRelationships,
    ) {
        let mut alternate_content: &str = "";

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"mc:Choice" => {
                        alternate_content = "Choice";
                        set_string_from_xml!(self, e, requires, "Requires");
                    }
                    b"mc:Fallback" => {
                        alternate_content = "Fallback";
                    }
                    b"oleObject" => {
                        if alternate_content == "Choice" {
                            self.prog_id
                                .set_value_string(get_attribute(e, b"progId").unwrap());

                            let r_id = get_attribute(e, b"r:id").unwrap();
                            let attached_file =
                                relationships.get_relationship_by_rid(&r_id).get_raw_file();
                            self.set_object_extension(attached_file.get_extension());
                            self.set_object_data(attached_file.get_file_data().clone());
                        }
                    }
                    b"objectPr" => {
                        let mut obj = EmbeddedObjectProperties::default();
                        obj.set_attributes(reader, e, relationships);
                        self.set_embedded_object_properties(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"mc:AlternateContent" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "mc:AlternateContent")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        r_id: &usize,
        ole_id: &usize,
    ) {
        // mc:AlternateContent
        write_start_tag(
            writer,
            "mc:AlternateContent",
            vec![("xmlns:mc", MC_NS)],
            false,
        );

        // mc:Choice
        write_start_tag(
            writer,
            "mc:Choice",
            vec![("Requires", self.requires.get_value_str())],
            false,
        );

        // oleObject
        let r_id_str = format!("rId{}", r_id);
        let shape_id_str = format!("{}", ole_id);
        let attributes = vec![
            ("progId", self.prog_id.get_value_str()),
            ("shapeId", &shape_id_str),
            ("r:id", &r_id_str),
        ];
        write_start_tag(writer, "oleObject", attributes, false);

        // objectPr
        self.embedded_object_properties
            .write_to(writer, &(r_id + 1));

        write_end_tag(writer, "oleObject");

        write_end_tag(writer, "mc:Choice");

        // mc:Fallback
        write_start_tag(writer, "mc:Fallback", vec![], false);

        // oleObject
        let r_id_str = format!("rId{}", r_id);
        let attributes = vec![
            ("progId", self.prog_id.get_value_str()),
            ("shapeId", &shape_id_str),
            ("r:id", &r_id_str),
        ];
        write_start_tag(writer, "oleObject", attributes, true);

        write_end_tag(writer, "mc:Fallback");

        write_end_tag(writer, "mc:AlternateContent");
    }
}
