pub use quaternion::*;
pub use vecmath::*;

pub type Vector2 = vecmath::Vector2<f64>;
pub fn vec2(x: f64, y: f64) -> Vector2 {
    [x, y]
}

pub const VECTOR2_UP: Vector2 = [0.0, -1.0];
pub const VECTOR2_RIGHT: Vector2 = [1.0, 0.0];
pub const VECTOR2_ZERO: Vector2 = [0.0, 0.0];
pub const VECTOR2_ONE: Vector2 = [1.0, 1.0];

pub type Vector3 = vecmath::Vector3<f64>;
pub fn vec3(x: f64, y: f64, z: f64) -> Vector3 {
    [x, y, z]
}

pub const VECTOR3_UP: Vector3 = [0.0, 1.0, 0.0];
pub const VECTOR3_RIGHT: Vector3 = [1.0, 0.0, 0.0];
pub const VECTOR3_FORWARD: Vector3 = [0.0, 0.0, 1.0];
pub const VECTOR3_ZERO: Vector3 = [0.0, 0.0, 0.0];
pub const VECTOR3_ONE: Vector3 = [1.0, 1.0, 1.0];

pub type Quaternion = quaternion::Quaternion<f64>;

pub struct Ray {
    pub origin: Vector3,
    /// The user is responsible of keeping this vector normal
    pub direction: Vector3,
}
