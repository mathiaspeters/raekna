use wgpu::SurfaceError;
use winit::dpi::PhysicalSize;

use super::{controls::Controls, wgpu_backend::WgpuRenderer};
use crate::coordinator::{content::Content, dimensions::Dimensions};

pub trait RenderBackend {
    fn update(&mut self, controls: &Controls);

    fn render(&mut self, content: &Content, dimensions: &Dimensions) -> Result<(), SurfaceError>;

    fn resize(&mut self, new_size: PhysicalSize<u32>);

    fn size(&self) -> PhysicalSize<u32>;
}

pub struct WgpuRenderBackend<'a> {
    renderer: WgpuRenderer<'a>,
}

impl<'a> WgpuRenderBackend<'a> {
    pub async fn new(window: &'a winit::window::Window, controls: &Controls) -> Self {
        let renderer = WgpuRenderer::new(window, controls).await;
        Self { renderer }
    }
}

impl<'a> RenderBackend for WgpuRenderBackend<'a> {
    fn update(&mut self, controls: &Controls) {
        self.renderer.update(controls);
    }

    fn render(&mut self, content: &Content, dimensions: &Dimensions) -> Result<(), SurfaceError> {
        self.renderer.render(content, dimensions)
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    fn size(&self) -> PhysicalSize<u32> {
        self.renderer.size()
    }
}
