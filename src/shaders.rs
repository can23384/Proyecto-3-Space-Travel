use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;

// ---------- Utilidades matemáticas seguras ----------
fn v3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
}

fn dot(a: Vector3, b: Vector3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn normalize(v: Vector3) -> Vector3 {
    let len2 = dot(v, v);
    if len2 > 0.0 {
        let inv = len2.sqrt().recip();
        v3(v.x * inv, v.y * inv, v.z * inv)
    } else {
        v
    }
}
// ----------------------------------------------------

fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

// Iluminación Lambert simple
fn lambert_light(normal: Vector3, light_dir: Vector3) -> Vector3 {
    let n = normalize(normal);
    let l = normalize(light_dir);
    let intensity = dot(n, l).max(0.0);
    v3(intensity, intensity, intensity)
}

// Star: c_core y c_outer vienen de uniforms.base_color1/2
fn shade_star(pos: Vector3, c_core: Vector3, c_outer: Vector3) -> Vector3 {
    // r^2 en espacio de modelo (para esfera unitaria suele rondar 1.0 en la superficie)
    let r2 = pos.x * pos.x + pos.y * pos.y + pos.z * pos.z;
    let glow = (1.5 - r2).clamp(0.0, 1.0); // caída suave y más brillante
    c_core * (0.85 + 0.15 * glow) + c_outer * 0.1
}

// Rocky: c_base y c_dark desde uniforms
fn hash(x: f32) -> f32 {
    ((x * 34.0).sin() * 143758.5453).fract().abs()
}
fn shade_rocky(pos: Vector3, c_base: Vector3, c_dark: Vector3) -> Vector3 {
    // ruido baratito
    let h = hash(pos.x + hash(pos.y + hash(pos.z)));
    // mezcla entre base y oscuro en función del ruido
    c_base * (0.8 + 0.2 * h) + c_dark * (0.4 * h)
}

// Gas Giant: bandas mezclando c1/c2
fn shade_gas_giant(pos: Vector3, c1: Vector3, c2: Vector3) -> Vector3 {
    let bands = (pos.y * 10.0).sin() * 0.5 + 0.5;
    c1 * bands + c2 * (1.0 - bands)
}

fn shade_cyber(pos: Vector3, c1: Vector3, c2: Vector3) -> Vector3 {
    let grid = ((pos.x * 15.0).sin().abs() > 0.95 ||
                (pos.y * 15.0).sin().abs() > 0.95 ||
                (pos.z * 15.0).sin().abs() > 0.95) as i32;

    // Si coincide con la "rejilla", usa color neon
    if grid == 1 {
        c2 * 1.5 // brillante
    } else {
        c1 // metal oscuro
    }
}

fn shade_magma(pos: Vector3, lava: Vector3, rock: Vector3) -> Vector3 {
    let h = hash(pos.x + hash(pos.y + hash(pos.z)));

    // lava donde el ruido es grande
    let mix = (h * 2.0 - 0.9).clamp(0.0, 1.0);
    lava * mix + rock * (1.0 - mix)
}

fn shade_flat(_pos: Vector3, color: Vector3) -> Vector3 {
    color
}

fn shade_ring(pos: Vector3, inner: Vector3, outer: Vector3) -> Vector3 {
    // distancia radial
    let r = (pos.x * pos.x + pos.z * pos.z).sqrt();

    // degradado del color entre interior y exterior del anillo
    let t = ((r - 0.6) * 2.0).clamp(0.0, 1.0);

    inner * (1.0 - t) + outer * t
}

pub fn vertex_shader(v: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Transformación de posición
    let pos4 = Vector4::new(v.position.x, v.position.y, v.position.z, 1.0);
    let t = multiply_matrix_vector4(&uniforms.model_matrix, &pos4);
    let transformed = if t.w != 0.0 {
        v3(t.x / t.w, t.y / t.w, t.z / t.w)
    } else {
        v3(t.x, t.y, t.z)
    };

    let world_normal = v.normal;
let light_dir = v3(0.0, 0.0, -1.0);

let lambert = lambert_light(world_normal, light_dir);

let c1 = uniforms.base_color1;
let c2 = uniforms.base_color2;

let base_color = match uniforms.shader_type {
    0 => shade_star(v.position, c1, c2),
    1 => shade_rocky(v.position, c1, c2),
    2 => shade_gas_giant(v.position, c1, c2),
    3 => shade_cyber(v.position, c1, c2),      // ✅ nuevo
    4 => shade_magma(v.position, c1, c2),      // ✅ nuevo
    5 => shade_flat(v.position, c1),           // ✅ nuevo
    6 => shade_ring(v.position, c1, c2),
    _ => c1,
};

// Para estrella: EMISIÓN pura
let color = if uniforms.shader_type == 0 {
    base_color * uniforms.emission_strength
} else {
    // Para rocoso/gaseoso: Ambiente + Difusa
    let lambert = lambert_light(world_normal, light_dir);
    // color_final = base*(ambiente) + base*(difusa*intensidad)
    base_color * uniforms.ambient_strength + base_color * (lambert * uniforms.light_intensity)
};

// SOLO para estrella: emisivo (no multiplicar por Lambert)
let color = base_color;

    Vertex {
        position: v.position,
        normal: v.normal,
        tex_coords: v.tex_coords,
        color,
        transformed_position: transformed,
        transformed_normal: world_normal,
    }
}