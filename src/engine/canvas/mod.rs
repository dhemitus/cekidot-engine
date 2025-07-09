use anyhow::Result;

pub trait Canvas {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub trait RenderableCanvas: Canvas {
    fn on_render(&mut self) -> Result<()>;
    fn on_resize(&mut self) -> Option<(u32, u32)>;

    fn on_start(&mut self) -> Result<()>;
    fn on_end(&mut self) -> Result<()>;
}
