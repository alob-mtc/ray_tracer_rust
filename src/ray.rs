use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn ray(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub fn origin(self) -> Vec3 {
        self.a
    }

    pub fn direction(self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_origin() {}

    #[test]
    fn test_ray_direction() {}

    #[test]
    fn test_ray_point_at_parameter() {}
}
