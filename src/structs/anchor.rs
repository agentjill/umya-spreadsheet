#[derive(Default, Debug, Clone)]
pub struct Anchor {
    left_column: u32,
    left_offset: u32,
    top_row: u32,
    top_offset: u32,
    right_column: u32,
    right_offset: u32,
    bottom_row: u32,
    bottom_offset: u32,
}

impl Anchor {
    #[inline]
    pub fn get_left_column(&self) -> &u32 {
        &self.left_column
    }

    #[inline]
    pub fn set_left_column(&mut self, value: u32) {
        self.left_column = value;
    }

    #[inline]
    pub fn get_left_offset(&self) -> &u32 {
        &self.left_offset
    }

    #[inline]
    pub fn set_left_offset(&mut self, value: u32) {
        self.left_offset = value;
    }

    #[inline]
    pub fn get_top_row(&self) -> &u32 {
        &self.top_row
    }

    #[inline]
    pub fn set_top_row(&mut self, value: u32) {
        self.top_row = value;
    }

    #[inline]
    pub fn get_top_offset(&self) -> &u32 {
        &self.top_offset
    }

    #[inline]
    pub fn set_top_offset(&mut self, value: u32) {
        self.top_offset = value;
    }

    #[inline]
    pub fn get_right_column(&self) -> &u32 {
        &self.right_column
    }

    #[inline]
    pub fn set_right_column(&mut self, value: u32) {
        self.right_column = value;
    }

    #[inline]
    pub fn get_right_offset(&self) -> &u32 {
        &self.right_offset
    }

    #[inline]
    pub fn set_right_offset(&mut self, value: u32) {
        self.right_offset = value;
    }

    #[inline]
    pub fn get_bottom_row(&self) -> &u32 {
        &self.bottom_row
    }

    #[inline]
    pub fn set_bottom_row(&mut self, value: u32) {
        self.bottom_row = value;
    }

    #[inline]
    pub fn get_bottom_offset(&self) -> &u32 {
        &self.bottom_offset
    }

    #[inline]
    pub fn set_bottom_offset(&mut self, value: u32) {
        self.bottom_offset = value;
    }

    #[inline]
    pub(crate) fn _adjustment_insert_row(&mut self, num_rows: &u32) {
        self.top_row += num_rows;
        self.bottom_row += num_rows;
    }

    #[inline]
    pub(crate) fn _adjustment_insert_column(&mut self, num_cols: &u32) {
        self.left_column += num_cols;
        self.right_column += num_cols;
    }

    #[inline]
    pub(crate) fn _adjustment_remove_row(&mut self, num_rows: &u32) {
        self.top_row = self.top_row.saturating_sub(*num_rows).max(1);
        self.bottom_row = self.bottom_row.saturating_sub(*num_rows).max(1);
    }

    #[inline]
    pub(crate) fn _adjustment_remove_column(&mut self, num_cols: &u32) {
        self.left_column = self.left_column.saturating_sub(*num_cols).max(1);
        self.right_column = self.right_column.saturating_sub(*num_cols).max(1);
    }
}
