// xdr:wsDr
use super::ConnectionShape;
use super::GraphicFrame;
use super::OneCellAnchor;
use super::Picture;
use super::Shape;
use super::TwoCellAnchor;
use crate::helper::const_str::*;
use crate::reader::driver::*;
use crate::structs::raw::RawRelationships;
use crate::structs::Chart;
use crate::structs::Image;
use crate::structs::OleObjects;
use crate::traits::AdjustmentCoordinate;
use crate::traits::AdjustmentCoordinateWithSheet;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct WorksheetDrawing {
    image_collection: ThinVec<Image>,
    chart_collection: ThinVec<Chart>,
    one_cell_anchor_collection: ThinVec<OneCellAnchor>,
    two_cell_anchor_collection: ThinVec<TwoCellAnchor>,
}

impl WorksheetDrawing {
    #[inline]
    pub fn get_image_collection(&self) -> &[Image] {
        &self.image_collection
    }

    #[inline]
    pub fn get_image_collection_mut(&mut self) -> &mut ThinVec<Image> {
        &mut self.image_collection
    }

    #[inline]
    pub fn add_image(&mut self, value: Image) -> &mut Self {
        self.image_collection.push(value);
        self
    }

    #[inline]
    pub fn get_image(&self, col: &u32, row: &u32) -> Option<&Image> {
        self.image_collection
            .iter()
            .find(|&image| image.get_col() == &(col - 1) && image.get_row() == &(row - 1))
    }

    #[inline]
    pub fn get_image_mut(&mut self, col: &u32, row: &u32) -> Option<&mut Image> {
        self.image_collection
            .iter_mut()
            .find(|image| image.get_col() == &(col - 1) && image.get_row() == &(row - 1))
    }

    pub fn get_images(&self, col: &u32, row: &u32) -> Vec<&Image> {
        self.image_collection
            .iter()
            .filter(|image| image.get_col() == &(col - 1) && image.get_row() == &(row - 1))
            .collect()
    }

    pub fn get_images_mut(&mut self, col: &u32, row: &u32) -> Vec<&mut Image> {
        self.image_collection
            .iter_mut()
            .filter(|image| image.get_col() == &(col - 1) && image.get_row() == &(row - 1))
            .collect()
    }

    #[inline]
    pub fn get_chart_collection(&self) -> &[Chart] {
        &self.chart_collection
    }

    #[inline]
    pub fn get_chart_collection_mut(&mut self) -> &mut ThinVec<Chart> {
        &mut self.chart_collection
    }

    #[inline]
    pub fn add_chart_collection(&mut self, value: Chart) -> &mut Self {
        self.chart_collection.push(value);
        self
    }

    #[inline]
    pub fn get_chart(&self, col: &u32, row: &u32) -> Option<&Chart> {
        self.chart_collection
            .iter()
            .find(|&chart| chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1))
    }

    #[inline]
    pub fn get_chart_mut(&mut self, col: &u32, row: &u32) -> Option<&mut Chart> {
        self.chart_collection
            .iter_mut()
            .find(|chart| chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1))
    }

    pub fn get_charts(&self, col: &u32, row: &u32) -> Vec<&Chart> {
        self.chart_collection
            .iter()
            .filter(|chart| chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1))
            .collect()
    }

    pub fn get_charts_mut(&mut self, col: &u32, row: &u32) -> Vec<&mut Chart> {
        self.chart_collection
            .iter_mut()
            .filter(|chart| chart.get_col() == &(col - 1) && chart.get_row() == &(row - 1))
            .collect()
    }

    #[inline]
    pub fn get_one_cell_anchor_collection(&self) -> &[OneCellAnchor] {
        &self.one_cell_anchor_collection
    }

    #[inline]
    pub fn get_one_cell_anchor_collection_mut(&mut self) -> &mut ThinVec<OneCellAnchor> {
        &mut self.one_cell_anchor_collection
    }

    #[inline]
    pub fn add_one_cell_anchor_collection(&mut self, value: OneCellAnchor) -> &mut Self {
        self.one_cell_anchor_collection.push(value);
        self
    }

    #[inline]
    pub fn get_two_cell_anchor_collection(&self) -> &[TwoCellAnchor] {
        &self.two_cell_anchor_collection
    }

    #[inline]
    pub fn get_two_cell_anchor_collection_mut(&mut self) -> &mut ThinVec<TwoCellAnchor> {
        &mut self.two_cell_anchor_collection
    }

    #[inline]
    pub fn add_two_cell_anchor_collection(&mut self, value: TwoCellAnchor) -> &mut Self {
        self.two_cell_anchor_collection.push(value);
        self
    }

    #[inline]
    pub fn has_drawing_object(&self) -> bool {
        !self.chart_collection.is_empty()
            || !self.image_collection.is_empty()
            || !self.one_cell_anchor_collection.is_empty()
            || !self.two_cell_anchor_collection.is_empty()
    }

    pub fn get_graphic_frame_collection(&self) -> Vec<&GraphicFrame> {
        self.two_cell_anchor_collection
            .iter()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_graphic_frame())
            .collect()
    }

    pub fn get_graphic_frame_collection_mut(&mut self) -> Vec<&mut GraphicFrame> {
        self.two_cell_anchor_collection
            .iter_mut()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_graphic_frame_mut())
            .collect()
    }

    pub fn get_shape_collection(&self) -> Vec<&Shape> {
        self.two_cell_anchor_collection
            .iter()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_shape())
            .collect()
    }

    pub fn get_shape_collection_mut(&mut self) -> Vec<&mut Shape> {
        self.two_cell_anchor_collection
            .iter_mut()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_shape_mut())
            .collect()
    }

    pub fn get_connection_shape_collection(&self) -> Vec<&ConnectionShape> {
        self.two_cell_anchor_collection
            .iter()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_connection_shape())
            .collect()
    }

    pub fn get_connection_shape_collection_mut(&mut self) -> Vec<&mut ConnectionShape> {
        self.two_cell_anchor_collection
            .iter_mut()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_connection_shape_mut())
            .collect()
    }

    pub fn get_picture_collection(&self) -> Vec<&Picture> {
        self.two_cell_anchor_collection
            .iter()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_picture())
            .collect()
    }

    pub fn get_one_cell_anchor_all_list(&mut self) -> Vec<&mut OneCellAnchor> {
        self.one_cell_anchor_collection
            .iter_mut()
            .chain(
                self.image_collection
                    .iter_mut()
                    .filter_map(|image| image.get_one_cell_anchor_mut()),
            )
            .collect()
    }

    pub fn get_two_cell_anchor_all_list(&mut self) -> Vec<&mut TwoCellAnchor> {
        self.two_cell_anchor_collection
            .iter_mut()
            .chain(
                self.chart_collection
                    .iter_mut()
                    .map(|chart| chart.get_two_cell_anchor_mut()),
            )
            .chain(
                self.image_collection
                    .iter_mut()
                    .filter_map(|image| image.get_two_cell_anchor_mut()),
            )
            .collect()
    }

    pub fn get_picture_collection_mut(&mut self) -> Vec<&mut Picture> {
        self.two_cell_anchor_collection
            .iter_mut()
            .filter_map(|two_cell_anchor| two_cell_anchor.get_picture_mut())
            .collect()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
        ole_objects: &mut OleObjects,
    ) {
        let mut ole_index = 0;
        let mut is_alternate_content = false;

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"mc:AlternateContent" => {
                        is_alternate_content = true;
                    }
                    b"xdr:oneCellAnchor" => {
                        if is_alternate_content {
                            continue;
                        }
                        let mut obj = OneCellAnchor::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        if obj.is_image() {
                            let mut image = Image::default();
                            image.set_one_cell_anchor(obj);
                            self.add_image(image);
                        } else {
                            self.add_one_cell_anchor_collection(obj);
                        }
                    }
                    b"xdr:twoCellAnchor" => {
                        let os = ole_objects.get_ole_object_mut();
                        if is_alternate_content && !os.is_empty() {
                            os[ole_index]
                                .get_two_cell_anchor_mut()
                                .set_is_alternate_content(true);
                            os[ole_index].get_two_cell_anchor_mut().set_attributes(
                                reader,
                                e,
                                drawing_relationships,
                            );
                            ole_index += 1;
                            continue;
                        }
                        let mut obj = TwoCellAnchor::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        if obj.is_support() {
                            if obj.is_chart() {
                                let mut chart = Chart::default();
                                chart.set_two_cell_anchor(obj);
                                self.add_chart_collection(chart);
                            } else if obj.is_image() {
                                let mut image = Image::default();
                                image.set_two_cell_anchor(obj);
                                self.add_image(image);
                            } else {
                                self.add_two_cell_anchor_collection(obj);
                            }
                        }
                    }
                    _ => (),
                }
            },

            Event::End(ref e) => {
                match e.name().into_inner() {
                    b"mc:AlternateContent" => {
                        is_alternate_content = false;
                    }
                    b"xdr:wsDr" => return,
                    _ => (),
                }
            },

            Event::Eof => panic!("Error: Could not find {} end element", "xdr:wsDr")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        ole_objects: &OleObjects,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:wsDr
        write_start_tag(
            writer,
            "xdr:wsDr",
            vec![
                ("xmlns:xdr", SHEET_DRAWING_NS),
                ("xmlns:a", DRAWINGML_MAIN_NS),
            ],
            false,
        );

        // xdr:twoCellAnchor
        for chart in &self.chart_collection {
            chart.get_two_cell_anchor().write_to(writer, rel_list, &0);
        }
        for image in &self.image_collection {
            image.write_to(writer, rel_list);
        }
        for two_cell_anchor in &self.two_cell_anchor_collection {
            two_cell_anchor.write_to(writer, rel_list, &0);
        }

        // xdr:oneCellAnchor
        for one_cell_anchor in &self.one_cell_anchor_collection {
            one_cell_anchor.write_to(writer, rel_list);
        }

        // mc:AlternateContent
        let mut ole_id = 1000 + 25;
        for ole_object in ole_objects.get_ole_object() {
            ole_object
                .get_two_cell_anchor()
                .write_to(writer, rel_list, &0);
            ole_id += 1;
        }

        write_end_tag(writer, "xdr:wsDr");
    }
}
impl AdjustmentCoordinate for WorksheetDrawing {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for anchor in &mut self.one_cell_anchor_collection {
            anchor.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        for anchor in &mut self.two_cell_anchor_collection {
            anchor.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        for chart in &mut self.chart_collection {
            chart.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        for image in &mut self.image_collection {
            image.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        &mut self.one_cell_anchor_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for anchor in &mut self.one_cell_anchor_collection {
            anchor.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        &mut self.two_cell_anchor_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for anchor in &mut self.two_cell_anchor_collection {
            anchor.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        &mut self.chart_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for chart in &mut self.chart_collection {
            chart.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        &mut self.image_collection.retain(|k| {
            !(k.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for image in &mut self.image_collection {
            image.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }
}
impl AdjustmentCoordinateWithSheet for WorksheetDrawing {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // chart
        for chart in &mut self.chart_collection {
            chart.adjustment_insert_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // chart
        for chart in &mut self.chart_collection {
            chart.adjustment_remove_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }
}
