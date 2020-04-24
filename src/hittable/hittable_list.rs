use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<T> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, item: T) {
        self.objects.push(item);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut list_hit_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = hit_record.t;
                list_hit_record = hit_record;
            }
        }

        if hit_anything {
            Some(list_hit_record)
        } else {
            None
        }
    }
}
