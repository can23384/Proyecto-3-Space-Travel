mod framebuffer;
mod triangle;
mod vertex;
mod fragment;
mod shaders;
mod obj;
mod matrix;


use framebuffer::Framebuffer;
use shaders::vertex_shader;
use obj::Obj;
use raylib::prelude::*;
use std::time::Duration;
use std::thread;
use std::f32::consts::PI;
use crate::matrix::new_matrix4;
use rand::Rng;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub shader_type: u32,
    pub base_color1: Vector3,
    pub base_color2: Vector3,
    pub light_intensity: f32,
    pub ambient_strength: f32,
    pub emission_strength: f32,
}

// Combina traslación + escala + rotación
fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_x = new_matrix4(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let rotation_y = new_matrix4(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let rotation_z = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,   1.0, 0.0,
        0.0,    0.0,   0.0, 1.0,
    );

    let rotation = rotation_z * rotation_y * rotation_x;

    let scale_matrix = new_matrix4(
        scale, 0.0,  0.0,  0.0,
        0.0,  scale, 0.0,  0.0,
        0.0,  0.0,  scale, 0.0,
        0.0,  0.0,  0.0,  1.0,
    );

    let translation_matrix = new_matrix4(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );

    scale_matrix * rotation * translation_matrix
}

// Pipeline: vertex → triángulos → fragments → framebuffer (con z-buffer)
fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, obj: &Obj) {
    let mut transformed = Vec::with_capacity(obj.vertices.len());
    for v in &obj.vertices {
        transformed.push(vertex_shader(v, uniforms));
    }

    let mut fragments = Vec::new();
    for face in obj.indices.chunks(3) {
        let v0 = &transformed[face[0] as usize];
        let v1 = &transformed[face[1] as usize];
        let v2 = &transformed[face[2] as usize];
        fragments.extend(triangle::triangle(v0, v1, v2));
    }

    for frag in fragments {
        framebuffer.point(
            frag.position.x as i32,
            frag.position.y as i32,
            frag.color,
            frag.depth, // 👈 Aquí usamos la profundidad
        );
    }
}


// Cámara libre sobre el plano eclíptico con zoom
struct Camera {
    pub pos: Vector3,   // posición en espacio
    pub zoom: f32,      // factor de zoom
}

impl Camera {
    fn new() -> Self {
        Self {
            pos: Vector3::new(0.0, 0.0, 0.0),
            zoom: 1.0,
        }
    }

    fn update(&mut self, window: &RaylibHandle) {
        let move_speed = 6.0;
        let zoom_speed = 0.02;

        // Movimiento horizontal (izquierda / derecha)
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.pos.x -= move_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.pos.x += move_speed;
        }

        // Movimiento vertical (arriba / abajo)
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.pos.y -= move_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.pos.y += move_speed;
        }

        // Zoom con flechas arriba/abajo
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.zoom *= 1.0 + zoom_speed; // acercar
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.zoom *= 1.0 - zoom_speed; // alejar
        }

        // Limita el zoom
        self.zoom = self.zoom.clamp(0.3, 3.0);
    }
}


struct Star {
    pos: Vector3,   // posición espacial (x, y, z)
    color: Vector3, // color de la estrella
}

fn draw_skybox(framebuffer: &mut Framebuffer, stars: &Vec<Star>, camera: &Camera) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    for star in stars {
        // posición relativa a la cámara (solo movimiento, sin zoom)
        let rel = star.pos - camera.pos;

        // proyección sin aplicar zoom
        let x_screen = width / 2.0 + rel.x;
        let y_screen = height / 2.0 + rel.y;

        if x_screen >= 0.0 && x_screen < width && y_screen >= 0.0 && y_screen < height {
            framebuffer.point(x_screen as i32, y_screen as i32, star.color, 10_000.0);

            // opcional: algunas estrellas más brillantes
            if rand::random::<f32>() < 0.1 {
                framebuffer.point(x_screen as i32 + 1, y_screen as i32, star.color, 10_000.0);
                framebuffer.point(x_screen as i32, y_screen as i32 + 1, star.color, 10_000.0);
            }
        }
    }
}





fn main() {
    let screen_width = 800;
    let screen_height = 600;
    let center_x = (screen_width / 2) as f32;
    let center_y = (screen_height / 2) as f32;

    let (mut window, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Proyecto 3: Space Travel")
        .build();

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Vector3::new(0.01, 0.01, 0.03)); // espacio oscuro

    // Modelos
    let sphere = Obj::load("src/planeta.obj").expect("No se pudo cargar planeta.obj");
    let ring   = Obj::load("src/ring.obj").expect("No se pudo cargar ring.obj");

    // Centro del sistema y plano eclíptico
    let sun_pos = Vector3::new(center_x, center_y, 600.0);
    let ecliptic_y = sun_pos.y;

    // Colores
    let star_color1  = Vector3::new(1.00, 0.90, 0.45);
    let star_color2  = Vector3::new(0.25, 0.18, 0.08);

    let rocky_color1 = Vector3::new(0.22, 0.55, 0.85);
    let rocky_color2 = Vector3::new(0.05, 0.20, 0.10);

    let gas_color1   = Vector3::new(0.92, 0.74, 0.46);
    let gas_color2   = Vector3::new(0.62, 0.52, 0.34);

    let lava_color1  = Vector3::new(0.75, 0.15, 0.05);
    let lava_color2  = Vector3::new(0.10, 0.02, 0.01);

    let cyber_color1 = Vector3::new(0.15, 0.18, 0.22);
    let cyber_color2 = Vector3::new(0.00, 0.75, 1.00);

    let ring_color_inner = Vector3::new(0.65, 0.60, 0.50);
    let ring_color_outer = Vector3::new(0.85, 0.80, 0.70);

    // Iluminación
    let light_intensity: f32   = 1.0;
    let ambient_strength: f32  = 0.18;
    let emission_strength: f32 = 1.2;

    // Radios de órbita
    let rocky_orbit_radius = 320.0;   // cerca del sol
    let gas_orbit_radius   = 700.0;   // bastante más afuera
    let cyber_orbit_radius = 1100.0;  // lejos
    let lava_orbit_radius  = 1500.0;  // el más externo

    // Cámara
    let mut camera = Camera::new();
    let mut t: f32 = 0.0;

    // Estrellas
    let mut rng = rand::thread_rng();
    let mut stars = Vec::new();
    let num_stars = 2000;

    for _ in 0..num_stars {
        let x = rng.gen_range(-3000.0..3000.0);
        let y = rng.gen_range(-1500.0..1500.0);
        let z = rng.gen_range(2000.0..9000.0);
        let brightness: f32 = rng.gen_range(0.5..1.0);
        let color = Vector3::new(
            brightness,
            brightness * rng.gen_range(0.7..1.0),
            brightness * rng.gen_range(0.8..1.0),
        );
        stars.push(Star { pos: Vector3::new(x, y, z), color });
    }

    let mut warp_target: Option<Vector3> = None;

    let mut moon_angle: f32 = 0.0;
let moon_distance: f32 = 120.0;      // distancia desde el planeta
let moon_scale_factor: f32 = 0.35; 

    while !window.window_should_close() {
        t += 0.01;

        // actualizar cámara con teclas (mov/zoom)
        camera.update(&window);

        let base_z = sun_pos.z;

        // =========================
        // POSICIONES MUNDO (sin cámara)
        // =========================

        // Sol (fijo)
        let sun_world = sun_pos;

        // Rocoso
        let rocky_angle = t * 0.9;
        let rocky_world = Vector3::new(
            sun_pos.x + rocky_orbit_radius * rocky_angle.cos(),
            ecliptic_y,
            base_z + rocky_orbit_radius * rocky_angle.sin(),
        );

        // Gaseoso
        let gas_angle = t * 0.55;
        let gas_world = Vector3::new(
            sun_pos.x + gas_orbit_radius * gas_angle.cos(),
            ecliptic_y,
            base_z + gas_orbit_radius * gas_angle.sin(),
        );

        // Cibernético
        let cyber_angle = t * 0.42;
        let cyber_world = Vector3::new(
            sun_pos.x + cyber_orbit_radius * cyber_angle.cos(),
            ecliptic_y,
            base_z + cyber_orbit_radius * cyber_angle.sin(),
        );

        // Lava
        let lava_angle = t * 0.32;
        let lava_world = Vector3::new(
            sun_pos.x + lava_orbit_radius * lava_angle.cos(),
            ecliptic_y,
            base_z + lava_orbit_radius * lava_angle.sin(),
        );

        // =========================
        // INSTANT WARP (usa POS MUNDO)
        // =========================
// --- WARP SUAVE ---
if window.is_key_pressed(KeyboardKey::KEY_ONE) {
    warp_target = Some(Vector3::new(sun_world.x - center_x, sun_world.y - center_y, 0.0));
    camera.zoom = 1.0;
}
if window.is_key_pressed(KeyboardKey::KEY_TWO) {
    warp_target = Some(Vector3::new(rocky_world.x - center_x, rocky_world.y - center_y, 0.0));
    camera.zoom = 1.0;
}
if window.is_key_pressed(KeyboardKey::KEY_THREE) {
    warp_target = Some(Vector3::new(gas_world.x - center_x, gas_world.y - center_y, 0.0));
    camera.zoom = 1.0;
}
if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
    warp_target = Some(Vector3::new(cyber_world.x - center_x, cyber_world.y - center_y, 0.0));
    camera.zoom = 1.0;
}
if window.is_key_pressed(KeyboardKey::KEY_FIVE) {
    warp_target = Some(Vector3::new(lava_world.x - center_x, lava_world.y - center_y, 0.0));
    camera.zoom = 1.0;
}

// Si hay warp pendiente → animar suavemente
if let Some(target) = warp_target {
    let dx = target.x - camera.pos.x;
    let dy = target.y - camera.pos.y;
    let dist = (dx * dx + dy * dy).sqrt();

    // Movimiento lineal interpolado
    camera.pos.x += dx * 0.08;
    camera.pos.y += dy * 0.08;

    // Cuando llega cerca, detener warp
    if dist < 2.0 {
        camera.pos.x = target.x;
        camera.pos.y = target.y;
        warp_target = None;
    }
}

        // =========================
        // RENDER
        // =========================

        framebuffer.clear();
        draw_skybox(&mut framebuffer, &stars, &camera);

        // ☀️ Sol
        let sun_screen = Vector3::new(
            (sun_world.x - camera.pos.x) * camera.zoom,
            (sun_world.y - camera.pos.y) * camera.zoom,
            sun_world.z - camera.pos.z,
        );
        let sun_rot = Vector3::new(0.0, t * 0.4, 0.0);
        let sun_uniforms = Uniforms {
            model_matrix: create_model_matrix(sun_screen, 150.0 * camera.zoom, sun_rot),
            shader_type: 0,
            base_color1: star_color1,
            base_color2: star_color2,
            light_intensity,
            ambient_strength,
            emission_strength,
        };
        render(&mut framebuffer, &sun_uniforms, &sphere);

        // 🌎 Rocoso
        let rocky_screen = Vector3::new(
            (rocky_world.x - camera.pos.x) * camera.zoom,
            (rocky_world.y - camera.pos.y) * camera.zoom,
            rocky_world.z - camera.pos.z,
        );
        let rocky_rot = Vector3::new(0.0, t * 2.0, 0.0);
        let rocky_uniforms = Uniforms {
            model_matrix: create_model_matrix(rocky_screen, 70.0 * camera.zoom, rocky_rot),
            shader_type: 1,
            base_color1: rocky_color1,
            base_color2: rocky_color2,
            light_intensity,
            ambient_strength,
            emission_strength,
        };
        render(&mut framebuffer, &rocky_uniforms, &sphere);

        // 🌕 Luna del planeta rocoso
moon_angle += 0.02; // velocidad de órbita de la luna

// Posición de la luna girando alrededor del planeta rocoso
let moon_world_pos = Vector3::new(
    rocky_world.x + moon_distance * moon_angle.cos(),
    rocky_world.y,
    rocky_world.z + moon_distance * moon_angle.sin(),
);

// Convertir posición de mundo a pantalla según la cámara
let moon_screen_pos = Vector3::new(
    (moon_world_pos.x - camera.pos.x) * camera.zoom,
    (moon_world_pos.y - camera.pos.y) * camera.zoom,
    moon_world_pos.z - camera.pos.z,
);

// Rotación y escala de la luna
let moon_rotation = Vector3::new(0.0, t * 3.0, 0.0);
let moon_scale = 70.0 * moon_scale_factor * camera.zoom; // usa el tamaño del planeta base (70.0)

// Configuración del shader
let moon_uniforms = Uniforms {
    model_matrix: create_model_matrix(moon_screen_pos, moon_scale, moon_rotation),
    shader_type: 1, // mismo shader que el rocoso
    base_color1: Vector3::new(0.7, 0.7, 0.7), // gris claro
    base_color2: Vector3::new(0.3, 0.3, 0.3), // gris oscuro
    light_intensity,
    ambient_strength,
    emission_strength,
};

// Renderizar luna
render(&mut framebuffer, &moon_uniforms, &sphere);

        // ☁️ Gaseoso
        let gas_screen = Vector3::new(
            (gas_world.x - camera.pos.x) * camera.zoom,
            (gas_world.y - camera.pos.y) * camera.zoom,
            gas_world.z - camera.pos.z,
        );
        let gas_rot = Vector3::new(t * 1.8, t * 1.2, t * 0.7);
        let gas_uniforms = Uniforms {
            model_matrix: create_model_matrix(gas_screen, 95.0 * camera.zoom, gas_rot),
            shader_type: 2,
            base_color1: gas_color1,
            base_color2: gas_color2,
            light_intensity,
            ambient_strength,
            emission_strength,
        };
        render(&mut framebuffer, &gas_uniforms, &sphere);

        // 🪐 Anillo gaseoso
        let ring_rot = Vector3::new(0.4 + t * 0.2, t * 1.2, 0.3 + t * 0.7);
        let ring_uniforms = Uniforms {
            model_matrix: create_model_matrix(gas_screen, 80.0 * camera.zoom, ring_rot),
            shader_type: 6,
            base_color1: ring_color_inner,
            base_color2: ring_color_outer,
            light_intensity,
            ambient_strength,
            emission_strength,
        };
        render(&mut framebuffer, &ring_uniforms, &ring);

        // 🤖 Cibernético
        let cyber_screen = Vector3::new(
            (cyber_world.x - camera.pos.x) * camera.zoom,
            (cyber_world.y - camera.pos.y) * camera.zoom,
            cyber_world.z - camera.pos.z,
        );
        let cyber_rot = Vector3::new(t * 2.0, t * 1.2, t * 0.5);
        let cyber_uniforms = Uniforms {
            model_matrix: create_model_matrix(cyber_screen, 75.0 * camera.zoom, cyber_rot),
            shader_type: 3,
            base_color1: cyber_color1,
            base_color2: cyber_color2,
            light_intensity,
            ambient_strength,
            emission_strength,
        };
        render(&mut framebuffer, &cyber_uniforms, &sphere);

        // 🌋 Lava
        let lava_screen = Vector3::new(
            (lava_world.x - camera.pos.x) * camera.zoom,
            (lava_world.y - camera.pos.y) * camera.zoom,
            lava_world.z - camera.pos.z,
        );
        let lava_rot = Vector3::new(t * 3.0, t * 0.7, 0.0);
        let lava_uniforms = Uniforms {
            model_matrix: create_model_matrix(lava_screen, 65.0 * camera.zoom, lava_rot),
            shader_type: 4,
            base_color1: lava_color1,
            base_color2: lava_color2,
            light_intensity,
            ambient_strength,
            emission_strength,
        };
        render(&mut framebuffer, &lava_uniforms, &sphere);

        // Captura
        if window.is_key_pressed(KeyboardKey::KEY_P) {
            framebuffer.save_image("space_render.png");
        }

        // Presentar
        let texture = window
            .load_texture_from_image(&thread, &framebuffer.image)
            .expect("No se pudo crear textura desde framebuffer");

        {
            let mut d = window.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            d.draw_texture(&texture, 0, 0, Color::WHITE);

            d.draw_text(
                &format!(
                    "Cam X: {:.1} | Y: {:.1} | Zoom: {:.2}",
                    camera.pos.x, camera.pos.y, camera.zoom
                ),
                10,
                10,
                20,
                Color::RAYWHITE,
            );

            d.draw_text("1: Ir al Sol",              10, 40, 18, Color::YELLOW);
            d.draw_text("2: Ir Planeta Rocoso",     10, 60, 18, Color::SKYBLUE);
            d.draw_text("3: Ir Planeta Gaseoso",    10, 80, 18, Color::ORANGE);
            d.draw_text("4: Ir Planeta Cibernético",10,100,18, Color::BLUE);
            d.draw_text("5: Ir Planeta de Lava",    10,120,18, Color::RED);
            d.draw_text("W/A/S/D: mover camara",10,150,16, Color::RAYWHITE);
            d.draw_text("UP/DOWN: zoom",            10,170,16, Color::RAYWHITE);
        }

        drop(texture);
        thread::sleep(Duration::from_millis(16));
    }
}

