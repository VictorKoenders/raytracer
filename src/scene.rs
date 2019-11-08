use crate::math::Vector3;
use crate::{Color, Hit, Ray};

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub background_color: Color,
    pub ambient_lights: Vec<Color>,
    pub directional_lights: Vec<(Vector3, Color)>,
}

impl Scene {
    pub fn new(background_color: Color) -> Scene {
        Scene {
            objects: Vec::new(),
            background_color,
            ambient_lights: Vec::new(),
            directional_lights: Vec::new(),
        }
    }

    pub fn calculate_hit(&self, ray: &Ray, exclude: &Option<&dyn Object>) -> Option<Hit> {
        let mut nearest_hit: Option<(Hit, f32)> = None;

        // Get the pointer to the exclude object, or std::ptr::null() if no
        // exclude object is provided. This is completely safe, we're just
        // using this to compare the pointer to another pointer later As long as
        // we don't dereference this, everything is fine

        // We need to cast all these pointers to *const u8, because std::ptr::eq
        // also compares traits implementations, and we only care about the pointer.
        let exclude_ptr: *const u8 = exclude.map_or(std::ptr::null::<u8>(), |e| {
            e as *const dyn Object as *const u8
        });

        for obj in &self.objects {
            let obj_ptr = &**obj as *const dyn Object as *const u8;
            // This is the object we're meant to ignore, so ignore it
            if obj_ptr == exclude_ptr {
                continue;
            }

            if let Some(hit) = obj.intersect(ray) {
                let distance_squared = (hit.position - ray.start).length_squared();
                if let Some((_, previous_distance_squared)) = &nearest_hit {
                    if *previous_distance_squared < distance_squared {
                        continue;
                    }
                }
                nearest_hit = Some((hit, distance_squared));
            }
        }

        nearest_hit.map(|(hit, _)| hit)
    }

    pub fn add<T: Object + 'static>(&mut self, obj: T) {
        debug_assert!(
            std::mem::size_of::<T>() > 0,
            "Because of an optimalization, all scene items must have a size"
        );
        self.objects.push(Box::new(obj));
    }
}

pub trait Object: std::fmt::Debug + Sync {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Hit<'a>>;
    fn color(&self) -> Color;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3,
    pub size: f32,
    pub color: Color,
}

impl Object for Sphere {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Hit<'a>> {
        let distance_to_center = self.center - ray.start;
        let v = distance_to_center.dot(ray.direction);
        if v < 0.0 {
            return None;
        }
        let disc = self.size.powf(2.0) - (distance_to_center.dot(distance_to_center) - v.powf(2.0));

        if disc < 0.0 {
            return None;
        }

        let d = disc.sqrt();
        let intersect_at = ray.start + (v - d) * ray.direction;
        let normal = (self.center - intersect_at).normalize();

        Some(Hit {
            position: intersect_at,
            normal,
            object: self,
        })
    }
    fn color(&self) -> Color {
        self.color
    }
}
