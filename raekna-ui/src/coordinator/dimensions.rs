use wgpu_glyph::ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use winit::dpi::PhysicalSize;

use crate::constants::{SEPARATOR_WIDTH, TEXT_PADDING, TEXT_SCALING};

#[derive(Debug)]
pub struct Dimensions {
    pub minimum_window_size: PhysicalSize<u32>,
    pub window_size: PhysicalSize<u32>,
    pub glyph_size: PhysicalSize<f32>,
    pub result_x_offset: f32,
    pub columns: usize,
    pub lines: u16,
    pub separator_x_offset: f32,
    pub x_pixel_size: f32,
    pub y_pixel_size: f32,
    actual_scroll: f32,
    current_scroll: f32,
    pub scroll_ratio: f32,
}

impl Dimensions {
    pub fn new() -> Self {
        let glyph_size = Self::get_glyph_size();
        let columns = 32;
        let lines = 10;
        let minimum_window_size = Self::get_window_size(glyph_size, columns, lines);
        let window_size = minimum_window_size;
        let result_x_offset = Self::get_result_x_offset(glyph_size, columns);
        let separator_x_offset =
            (2.0 * TEXT_PADDING) + (glyph_size.width * ((columns - 16) as f32));
        let x_pixel_size = 2.0 / (window_size.width as f32);
        let y_pixel_size = 2.0 / (window_size.height as f32);
        let actual_scroll = 0.0;
        let current_scroll = 0.0;
        let scroll_ratio = 0.0;

        Self {
            minimum_window_size,
            window_size,
            glyph_size,
            result_x_offset,
            columns,
            lines,
            separator_x_offset,
            x_pixel_size,
            y_pixel_size,
            actual_scroll,
            current_scroll,
            scroll_ratio,
        }
    }

    pub fn update(&mut self, new_window_size: PhysicalSize<u32>) -> bool {
        let space_needed = self.window_height() / self.scroll_ratio;
        if self.update_window_size(new_window_size) {
            let width = new_window_size.width as f32;
            let height = new_window_size.height as f32;

            self.result_x_offset = {
                let result_size = (16.0 * self.glyph_width()) + TEXT_PADDING;
                width - result_size
            };
            self.columns = {
                let total_padding = 4.0 * TEXT_PADDING + SEPARATOR_WIDTH;
                let usable_width = width - total_padding;
                (usable_width / self.glyph_width()).floor() as usize
            };
            self.lines = {
                let usable_height = height - TEXT_PADDING;
                (usable_height / (self.glyph_height() + TEXT_PADDING)).floor() as u16
            };
            self.separator_x_offset = self.result_x_offset - TEXT_PADDING - SEPARATOR_WIDTH;
            self.x_pixel_size = 2.0 / width;
            self.y_pixel_size = 2.0 / height;

            self.scroll_ratio = height / space_needed;
            let max_scroll = space_needed - height;
            if max_scroll > 0.0 && max_scroll < self.actual_scroll {
                self.current_scroll = max_scroll;
            }
            true
        } else {
            false
        }
    }

    pub fn content_columns(&self) -> usize {
        self.columns - 16
    }

    pub fn result_columns(&self) -> usize {
        16
    }

    pub fn as_x_vertex(&self, pixel_value: f32) -> f32 {
        -1.0 + (self.x_pixel_size * pixel_value)
    }

    pub fn as_y_vertex(&self, pixel_value: f32) -> f32 {
        1.0 - (self.y_pixel_size * pixel_value)
    }

    pub fn window_width(&self) -> f32 {
        self.window_size.width as f32
    }

    pub fn window_height(&self) -> f32 {
        self.window_size.height as f32
    }

    pub fn glyph_width(&self) -> f32 {
        self.glyph_size.width
    }

    pub fn glyph_height(&self) -> f32 {
        self.glyph_size.height
    }

    pub fn scroll(&self) -> f32 {
        if self.scroll_ratio < 1.0 {
            self.current_scroll
        } else {
            0.0
        }
    }

    pub fn set_scroll(&mut self, scroll: f32) {
        self.current_scroll = scroll;
        self.actual_scroll = scroll;
    }

    fn get_glyph_size() -> PhysicalSize<f32> {
        let inconsolata = FontArc::try_from_slice(include_bytes!(
            "../rendering/resources/Inconsolata-Regular.ttf"
        ))
        .unwrap();
        let scaled = inconsolata.as_scaled(PxScale::from(TEXT_SCALING));

        let (glyph_width, glyph_height) = (scaled.h_advance(scaled.glyph_id('M')), scaled.height());
        PhysicalSize::new(glyph_width, glyph_height)
    }

    fn get_window_size(
        glyph_size: PhysicalSize<f32>,
        columns: usize,
        lines: u16,
    ) -> PhysicalSize<u32> {
        let content_width = glyph_size.width * (columns as f32);
        let scrollbar_width = glyph_size.width;
        let width =
            (4.0 * TEXT_PADDING) + SEPARATOR_WIDTH + content_width + (scrollbar_width * 1.5);

        let content_height = (glyph_size.height + TEXT_PADDING) * (lines as f32);
        let height = (2.0 * TEXT_PADDING) + content_height;

        PhysicalSize::new(width as u32, height as u32)
    }

    fn update_window_size(&mut self, mut new_window_size: PhysicalSize<u32>) -> bool {
        let min_width = self.minimum_window_size.width;
        let min_height = self.minimum_window_size.height;
        if new_window_size.width < min_width {
            new_window_size.width = min_width;
        }
        if new_window_size.height < min_height {
            new_window_size.height = min_height;
        }
        self.window_size = new_window_size;
        true
    }

    fn get_result_x_offset(glyph_size: PhysicalSize<f32>, columns: usize) -> f32 {
        let content_columns = columns - 16;
        (3.0 * TEXT_PADDING) + SEPARATOR_WIDTH + (glyph_size.width * (content_columns as f32))
    }
}
