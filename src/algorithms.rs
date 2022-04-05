use bevy::math::{Vec3, Vec2};

pub fn distance_vec3(v1: Vec3, v2: Vec3) -> f32 {
    (((v1.x - v2.x) * (v1.x - v2.x)) + ((v1.y - v2.y) * (v1.y - v2.y)) + ((v1.z - v2.z) * (v1.z - v2.z))).sqrt()
}

pub fn distance_vec2(v1: Vec2, v2: Vec2) -> f32 {
    (((v1.x - v2.x) * (v1.x - v2.x)) + ((v1.y - v2.y) * (v1.y - v2.y))).sqrt()
}