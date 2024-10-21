use bevy::prelude::*;
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub fn ray_intersects_aabb(ray: Ray, aabb: AABB) -> bool {
    // Handles division with checks for zero direction components
    fn safe_div(numerator: f32, denominator: f32) -> f32 {
        if denominator.abs() < f32::EPSILON {
            if numerator > 0.0 {
                f32::INFINITY
            } else {
                f32::NEG_INFINITY
            }
        } else {
            numerator / denominator
        }
    }

    // X axis checks
    let (mut tmin, mut tmax) = if ray.direction.x.abs() < f32::EPSILON {
        // Ray is parallel to x axis, so check if origin is within x bounds of AABB
        if ray.origin.x < aabb.min.x || ray.origin.x > aabb.max.x {
            return false;
        }
        (f32::NEG_INFINITY, f32::INFINITY)
    } else {
        let tmin = (aabb.min.x - ray.origin.x) / ray.direction.x;
        let tmax = (aabb.max.x - ray.origin.x) / ray.direction.x;
        if tmin > tmax { (tmax, tmin) } else { (tmin, tmax) }
    };

    // Y axis checks
    let (tymin, tymax) = if ray.direction.y.abs() < f32::EPSILON {
        // Ray is parallel to y axis, so check if origin is within y bounds of AABB
        if ray.origin.y < aabb.min.y || ray.origin.y > aabb.max.y {
            return false;
        }
        (f32::NEG_INFINITY, f32::INFINITY)
    } else {
        let tymin = (aabb.min.y - ray.origin.y) / ray.direction.y;
        let tymax = (aabb.max.y - ray.origin.y) / ray.direction.y;
        if tymin > tymax { (tymax, tymin) } else { (tymin, tymax) }
    };

    // Check overlap in tmin and tmax
    if tmin > tymax || tymin > tmax {
        return false;
    }
    tmin = tmin.max(tymin);
    tmax = tmax.min(tymax);

    // Z axis checks
    let (tzmin, tzmax) = if ray.direction.z.abs() < f32::EPSILON {
        // Ray is parallel to z axis, so check if origin is within z bounds of AABB
        if ray.origin.z < aabb.min.z || ray.origin.z > aabb.max.z {
            return false;
        }
        (f32::NEG_INFINITY, f32::INFINITY)
    } else {
        let tzmin = (aabb.min.z - ray.origin.z) / ray.direction.z;
        let tzmax = (aabb.max.z - ray.origin.z) / ray.direction.z;
        if tzmin > tzmax { (tzmax, tzmin) } else { (tzmin, tzmax) }
    };

    // Final overlap check
    if tmin > tzmax || tzmin > tmax {
        return false;
    }

    true
}
