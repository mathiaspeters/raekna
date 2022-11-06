use wgpu_glyph::{Section, Text};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    coordinator::{content::Content, dimensions::Dimensions},
    graphics::controls::Controls,
};

use crate::{
    constants::{TEXT_COLOR, TEXT_PADDING, TEXT_SCALING},
    coordinator::text_buffer::entry,
};

use super::renderer::Renderer;

pub struct RenderManager {
    pub size: winit::dpi::PhysicalSize<u32>,
    renderer: Renderer,
}

impl RenderManager {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let renderer = Renderer::new(window).await;
        Self { size, renderer }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.renderer.resize(new_size);
        }
    }

    pub fn render(
        &mut self,
        controls: &Controls,
        content: &Content,
        dimensions: &Dimensions,
    ) -> Result<(), wgpu::SurfaceError> {
        let buffers = controls.get_as_buffers(&self.renderer.device);
        let sections = Self::make_sections(content, dimensions);

        self.renderer.render(buffers, sections)?;

        Ok(())
    }

    fn make_sections<'a>(content: &'a Content, dimensions: &Dimensions) -> Vec<Section<'a>> {
        let mut y_offset = TEXT_PADDING - content.text_buffer.current_scroll;
        let result_offset = (3_f32 * TEXT_PADDING)
            + (dimensions.glyph_width() * dimensions.content_columns() as f32);
        let mut output = vec![];
        content
            .text_buffer
            .entries
            .iter()
            .enumerate()
            .for_each(|(i, entry)| {
                entry
                    .content
                    .iter()
                    .zip(entry.results.iter())
                    .for_each(|(c, r)| {
                        let is_hidden = y_offset > dimensions.window_height()
                            || y_offset + dimensions.glyph_height() < 0_f32;
                        if !is_hidden {
                            output.extend_from_slice(&Self::line_to_sections(
                                content
                                    .calculator
                                    .get_line(i)
                                    .expect("i cannot be out of bounds"),
                                y_offset,
                                result_offset,
                                c,
                                r,
                                dimensions,
                            ));
                        }
                        y_offset += TEXT_PADDING + dimensions.glyph_height();
                    })
            });
        output
    }

    fn line_to_sections<'a>(
        text: (&'a str, &'a str),
        y_offset: f32,
        result_offset: f32,
        c: &entry::Content,
        r: &entry::Result,
        dimensions: &Dimensions,
    ) -> Vec<Section<'a>> {
        let (cl, rl) = text;
        let mut output = vec![];

        let start_offset = c.segments.get(0).map(|s| s.start).unwrap_or(0);
        c.segments.iter().for_each(|s| {
            let text = &cl[s.start..s.end];
            let x_offset =
                TEXT_PADDING + ((s.start - start_offset) as f32 * dimensions.glyph_width());
            output.push(Self::build_section(text, x_offset, y_offset));
        });
        match r {
            entry::Result::None => {}
            entry::Result::Elipsis => {
                let chars = dimensions.result_columns() - 3;
                output.push(Self::build_section(&rl[..chars], result_offset, y_offset));
                let x_offset = result_offset + (chars as f32 * dimensions.glyph_width());
                output.push(Self::build_section("...", x_offset, y_offset));
            }
            entry::Result::Full => {
                let padding_chars = dimensions.result_columns() - rl.len();
                let padding = (padding_chars as f32 * dimensions.glyph_width()) / 2_f32;
                let x_offset = result_offset + padding;
                output.push(Self::build_section(rl, x_offset, y_offset));
            }
        }

        output
    }

    fn build_section(text: &str, x_offset: f32, y_offset: f32) -> Section {
        Section {
            screen_position: (x_offset, y_offset),
            text: vec![Text::new(text)
                .with_color(TEXT_COLOR)
                .with_scale(TEXT_SCALING)],
            ..Section::default()
        }
    }
}
