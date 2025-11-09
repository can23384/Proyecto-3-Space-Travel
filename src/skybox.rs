use rand::Rng;
use crate::framebuffer::Framebuffer;

pub struct Skybox {
    stars: Vec<(f32, f32, u32)>, // (x, y, color)
}

impl Skybox {
    pub fn new(width: usize, height: usize, num_stars: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::with_capacity(num_stars);

        for _ in 0..num_stars {
            let x = rng.gen_range(0.0..width as f32);
            let y = rng.gen_range(0.0..height as f32 * 0.9); // más denso hacia el horizonte
            let color = match rng.gen_range(0..5) {
                0 => 0xAAAAFF, // azulada
                1 => 0xFFFFFF, // blanca
                2 => 0xFFD080, // amarillenta
                3 => 0xC0C0FF, // tenue
                _ => 0x8080FF, // fría
            };
            stars.push((x, y, color));
        }

        Self { stars }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        for (x, y, color) in &self.stars {
            let xi = *x as isize;
            let yi = *y as isize;
            if xi >= 0 && yi >= 0 && xi < framebuffer.width as isize && yi < framebuffer.height as isize {
                let index = yi as usize * framebuffer.width + xi as usize;
                if index < framebuffer.buffer.len() {
                    framebuffer.buffer[index] = *color;
                }
            }
        }
    }
}
