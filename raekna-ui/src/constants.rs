pub const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.95,
    g: 0.95,
    b: 0.95,
    a: 1.0,
};
pub const TEXT_COLOR: [f32; 4] = [0.05, 0.05, 0.05, 1.0];
pub const SEPARATOR_COLOR: [f32; 3] = [0.6, 0.6, 0.6];

pub const SCROLLBAR_WIDTH_MULTIPLIER: f32 = 1.5;
pub const SCROLLBAR_BACKGROUND_COLOR: [f32; 3] = [0.8, 0.8, 0.8];
pub const SCROLLBAR_BASE_COLOR: [f32; 3] = [0.6, 0.6, 0.6];
pub const SCROLLBAR_HOVER_COLOR: [f32; 3] = [0.4, 0.4, 0.4];
pub const SCROLLBAR_CLICK_COLOR: [f32; 3] = [0.2, 0.2, 0.2];

pub const TEXT_SCALING: f32 = 30.0;
pub const TEXT_PADDING: f32 = 8.0;
pub const SEPARATOR_WIDTH: f32 = 1.0;

pub const CARET_WIDTH: f32 = 2.0;
pub const CARET_COLOR: [f32; 3] = [0.05, 0.05, 0.05];
pub const CARET_PERIOD: usize = 750;

pub const SELECTION_COLOR: [f32; 3] = [0.4, 0.6, 1.0];

pub const MULTI_CLICK_DELAY: u128 = 300;
