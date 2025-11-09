use raylib::prelude::*;


pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub image: Image,
    pub background_color: Vector3,
    pub depth_buffer: Vec<f32>, // ⭐ Z-buffer
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let image = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        let size = (width * height) as usize;

        Self {
            width,
            height,
            image,
            background_color: Vector3::new(0.0, 0.0, 0.0),
            depth_buffer: vec![f32::INFINITY; size], // Inicializamos todo "infinitamente lejos"
        }
    }

    pub fn set_background_color(&mut self, color: Vector3) {
        self.background_color = color;
    }

    /// Limpia el framebuffer y el z-buffer
    pub fn clear(&mut self) {
        let bg = Color::new(
            (self.background_color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (self.background_color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (self.background_color.z.clamp(0.0, 1.0) * 255.0) as u8,
            255,
        );

        self.image.clear_background(bg);
        self.depth_buffer.fill(f32::INFINITY);
    }

    /// Dibuja un punto con test de profundidad
    pub fn point(&mut self, x: i32, y: i32, color: Vector3, depth: f32) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        if !depth.is_finite() {
            return;
        }

        let idx = (y as u32 * self.width + x as u32) as usize;

        // ⭐ Solo pintamos si el fragmento está más cerca que lo que ya había
        if depth < self.depth_buffer[idx] {
            self.depth_buffer[idx] = depth;

            let c = Color::new(
                (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                255,
            );

            self.image.draw_pixel(x, y, c);
        }
    }

    /// Guarda el framebuffer en disco
    pub fn save_image(&mut self, path: &str) {
        self.image.export_image(path);
    }
}