use super::font::Font;
use super::fill::Fill;
use super::borders::Borders;
use super::alignment::Alignment;
use super::number_format::NumberFormat;
use super::protection::Protection;

#[derive(Clone, Debug)]
pub struct Style {
    font: Option<Font>,
    fill: Option<Fill>,
    borders: Option<Borders>,
    alignment: Option<Alignment>,
    number_format: Option<NumberFormat>,
    protection: Option<Protection>,
    xf_id: usize,
    quote_prefix: bool,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            font: None,
            fill: None,
            borders: None,
            alignment: None,
            number_format: None,
            protection: None,
            xf_id: 0,
            quote_prefix: false
        }
    }
}
impl Style {
    pub fn get_font(&self) -> &Option<Font> {
        &self.font
    }
    pub fn get_font_mut(&mut self) -> &mut Font {
        match &self.font {
            Some(_) => return self.font.as_mut().unwrap(),
            None => {}
        }
        self.set_font(Font::get_defalut_value());
        self.font.as_mut().unwrap()
    }
    pub(crate) fn set_font(&mut self, value:Font) {
        self.font = Some(value);
    }
    pub fn get_fill(&self) -> &Option<Fill> {
        &self.fill
    }
    pub fn get_fill_mut(&mut self) -> &mut Fill {
        match &self.fill {
            Some(_) => return self.fill.as_mut().unwrap(),
            None => {}
        }
        self.set_fill(Fill::get_defalut_value());
        self.fill.as_mut().unwrap()
    }
    pub(crate) fn set_fill(&mut self, value:Fill) {
        self.fill = Some(value);
    }
    pub fn get_borders(&self) -> &Option<Borders> {
        &self.borders
    }
    pub fn get_borders_mut(&mut self) -> &mut Borders {
        match &self.borders {
            Some(_) => return self.borders.as_mut().unwrap(),
            None => {}
        }
        self.set_borders(Borders::get_defalut_value());
        self.borders.as_mut().unwrap()
    }
    pub(crate) fn set_borders(&mut self, value:Borders) {
        self.borders = Some(value);
    }
    pub fn get_alignment(&self) -> &Option<Alignment> {
        &self.alignment
    }
    pub fn get_alignment_mut(&mut self) -> &mut Alignment {
        match &self.alignment {
            Some(_) => return self.alignment.as_mut().unwrap(),
            None => {}
        }
        self.set_alignment(Alignment::default());
        self.alignment.as_mut().unwrap()
    }
    pub(crate) fn set_alignment(&mut self, value:Alignment) {
        self.alignment = Some(value);
    }
    pub fn get_number_format(&self) -> &Option<NumberFormat> {
        &self.number_format
    }
    pub fn get_number_format_mut(&mut self) -> &mut NumberFormat {
        match &self.number_format {
            Some(_) => return self.number_format.as_mut().unwrap(),
            None => {}
        }
        self.set_number_format(NumberFormat::default());
        self.number_format.as_mut().unwrap()
    }
    pub(crate) fn set_number_format(&mut self, value:NumberFormat) {
        self.number_format = Some(value);
    }
    pub fn get_protection(&self) -> &Option<Protection> {
        &self.protection
    }
    pub(crate) fn set_protection(&mut self, value:Protection) {
        self.protection = Some(value);
    }
    pub fn get_xf_id(&self) -> &usize {
        &self.xf_id
    }
    pub(crate) fn set_xf_id(&mut self, value:usize) {
        self.xf_id = value;
    }
    pub fn get_quote_prefix(&self) -> &bool {
        &self.quote_prefix
    }
    pub(crate) fn set_quote_prefix(&mut self, value:bool) {
        self.quote_prefix = value;
    }
    pub(crate) fn get_defalut_value() -> Style {
        let def = Style::default();
        def
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}{}",
            match &self.font {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
            match &self.fill {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
            match &self.borders {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
            match &self.alignment {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
            match &self.number_format {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
        )))
    }
}