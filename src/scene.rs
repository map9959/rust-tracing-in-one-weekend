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
    fn intersect(&self, r: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>) {
        let mut hit_record: Option<HitRecord> = None;
        let mut hit_anything: bool = false;

        for o in self.objects.iter(){
            let (hit, record) = o.intersect(r, ray_t);
            if hit{
                if hit_record.is_none() || hit_record.unwrap().t > record.unwrap().t{
                    hit_anything = true;
                    hit_record = record;
                }
            }
        }

        return (hit_anything, hit_record);
    }
}