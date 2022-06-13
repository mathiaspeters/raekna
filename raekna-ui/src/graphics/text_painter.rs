use wgpu::{util::StagingBelt, CommandEncoder, Device, TextureFormat, TextureView};
use wgpu_glyph::{ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, Section, Text};

use crate::{
    constants::{TEXT_COLOR, TEXT_PADDING, TEXT_SCALING},
    coordinator::{content::Content, dimensions::Dimensions, text_buffer::entry},
};

pub struct TextPainter {
    window_size: (u32, u32),
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
}

impl TextPainter {
    pub fn new(
        size: &winit::dpi::PhysicalSize<u32>,
        device: &Device,
        render_format: TextureFormat,
    ) -> Self {
        let window_size = (size.width, size.height);

        let inconsolata =
            FontArc::try_from_slice(include_bytes!("./resources/Inconsolata-Regular.ttf")).unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(inconsolata).build(device, render_format);

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self {
            window_size,
            glyph_brush,
            staging_belt,
        }
    }

    pub fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        self.window_size = (new_size.width, new_size.height);
    }

    pub fn draw(
        &mut self,
        device: &Device,
        target: &TextureView,
        encoder: &mut CommandEncoder,
        content: &Content,
        dimensions: &Dimensions,
    ) {
        Self::make_sections(content, dimensions)
            .into_iter()
            .for_each(|section| self.glyph_brush.queue(section));

        let (width, height) = self.window_size;
        self.glyph_brush
            .draw_queued(
                device,
                &mut self.staging_belt,
                encoder,
                target,
                width,
                height,
            )
            .expect("Draw queued");
        self.staging_belt.finish();
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
