// Placeholder — wgpu texture loading will go here
// once the renderer is wired up with a wgpu Device

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub path: String,
    // pub inner: wgpu::Texture  <- added once renderer is set up
}

impl Texture {
    pub fn from_path(path: &str) -> Self {
        let img = image::open(path).expect("Failed to load texture");
        Self {
            width: img.width(),
            height: img.height(),
            path: path.to_string(),
        }
    }
}
