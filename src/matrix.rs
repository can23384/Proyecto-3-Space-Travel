use raylib::prelude::Matrix;

pub fn new_matrix4(
    r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32,
    r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32,
    r2c0: f32, r2c1: f32, r2c2: f32, r2c3: f32,
    r3c0: f32, r3c1: f32, r3c2: f32, r3c3: f32,
) -> Matrix {
    Matrix {
        m0: r0c0, m1: r1c0, m2: r2c0, m3: r3c0,
        m4: r0c1, m5: r1c1, m6: r2c1, m7: r3c1,
        m8: r0c2, m9: r1c2, m10: r2c2, m11: r3c2,
        m12: r0c3, m13: r1c3, m14: r2c3, m15: r3c3,
    }
}
