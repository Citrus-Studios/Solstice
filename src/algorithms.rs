use bevy::math::Vec3;

pub fn distance_vec3(v1: Vec3, v2: Vec3) -> f32 {
    (((v1.x - v2.x) * (v1.x - v2.x)) + ((v1.y - v2.y) * (v1.y - v2.y)) + ((v1.z - v2.z) * (v1.z - v2.z))).sqrt()
}