use wgpu::{util::StagingBelt, CommandEncoder, Device, TextureFormat, TextureView};
use wgpu_glyph::{ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, Section};

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

    pub fn draw<'a>(
        &mut self,
        device: &Device,
        target: &TextureView,
        encoder: &mut CommandEncoder,
        sections: Vec<Section<'a>>,
    ) {
        sections
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
}
