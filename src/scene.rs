use crate::{Ray, Hittable, hittable::HitRecord, utils::Interval};
pub struct Scene{
    objects: Vec<Box<dyn Hittable>>
}
impl Scene{
    pub fn new() -> Scene{
        let objects: Vec<Box<dyn Hittable>> = Vec::new();
        return Scene{objects};
    }
    pub fn push(&mut self, obj: Box<dyn Hittable>){
        self.objects.push(obj);
    }
}
impl Hittable for Scene{
    fn intersect(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut nearest: f64 = ray_t.max;

        for o in self.objects.iter(){
            let ray_interval = Interval{min: ray_t.min, max: nearest};
            let record = o.intersect(r, &ray_interval);
            match record{
                Some(r) => {
                    nearest = r.t;
                    hit_record = Some(r);
                }
                None => {}
            }
        }

        return hit_record;
    }
}