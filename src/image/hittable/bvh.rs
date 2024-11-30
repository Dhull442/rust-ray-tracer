use crate::image::hittable::aabb::AABB;
use crate::image::hittable::{HitRecord, Hittable, HittableObjects};
use crate::image::ray::Ray;
use crate::image::util;
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
pub struct BvhNode {
    hittable: Option<HittablePtr>,
    bbox: AABB,
    left: Option<BvhNodePtr>,
    right: Option<BvhNodePtr>,
}
type BvhNodePtr = Rc<RefCell<BvhNode>>;
type HittablePtr = Rc<RefCell<Hittable>>;
impl BvhNode {
    pub fn empty() -> Self {
        Self {
            hittable: None,
            bbox: AABB::empty(),
            left: None,
            right: None,
        }
    }
    pub fn new(world: &HittableObjects) -> Self {
        let mut world_objects = world.objects.clone();
        Self::new_from_objects(&mut world_objects, 0, world.objects.len())
    }

    pub fn new_from_object(hittable: Hittable) -> Self {
        Self {
            bbox: hittable.bounding_box(),
            left: None,
            hittable: Option::from(Rc::from(RefCell::from(hittable))),
            right: None,
        }
    }

    pub fn sort_slice(
        list: &mut Vec<Hittable>,
        start: usize,
        end: usize,
        comp: fn(&Hittable, &Hittable) -> Ordering,
    ) {
        let object_span = end - start;
        if object_span <= 1 {
            return;
        } else if object_span == 2 {
            let left = &list[start].clone();
            let right = &list[start + 1].clone();
            if comp(left, right) == Ordering::Less {
                list.swap(start, start + 1);
            }
        } else {
            let mid = start + object_span / 2;
            Self::sort_slice(list, start, mid, comp);
            Self::sort_slice(list, mid, end, comp);
        }
    }
    pub fn new_from_objects(list: &mut Vec<Hittable>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::empty();
        for idx in start..end {
            bbox = AABB::new_from_aabb(&bbox, &list[idx].bounding_box());
        }
        let axis = bbox.longest_axis();
        let left: BvhNode;
        let right: BvhNode;
        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };
        let object_span = end - start;
        if object_span == 1 {
            return Self::new_from_object(list[start].clone());
        } else if object_span == 2 {
            left = Self::new_from_object(list[start].clone());
            right = Self::new_from_object(list[start + 1].clone());
        } else {
            Self::sort_slice(list, start, end, comparator);
            let mid = start + object_span / 2;
            left = Self::new_from_objects(list, start, mid);
            right = Self::new_from_objects(list, mid, end);
        }
        Self {
            hittable: None,
            left: Option::from(Rc::from(RefCell::from(left))),
            right: Option::from(Rc::from(RefCell::from(right))),
            bbox,
        }
    }

    pub fn box_compare(a: &Hittable, b: &Hittable, axis_index: u64) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }
    pub fn box_x_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: &Hittable, b: &Hittable) -> Ordering {
        Self::box_compare(a, b, 2)
    }
    pub fn hit(&self, ray: &Ray, ray_t: &mut util::Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }

        if let Some(hittable) = self.hittable.as_ref() {
            return hittable.borrow().hit(&ray, *ray_t, rec);
        }

        let mut hit_left = false;
        let mut hit_right = false;

        if let Some(left) = &self.left {
            hit_left = left.borrow().hit(&ray, ray_t, rec);
        }

        if let Some(right) = &self.right {
            hit_right = right.borrow().hit(
                &ray,
                &mut util::Interval::new(ray_t.min, {
                    if hit_left {
                        rec.t
                    } else {
                        ray_t.max
                    }
                }),
                rec,
            );
        }

        hit_left || hit_right
    }

    pub fn bounding_box(&self) -> AABB {
        self.bbox
    }

    pub fn debug(&self) {
        println!("{}", self.bbox.debug());
        if let Some(left) = &self.left {
            println!("Left");
            left.borrow().debug();
        }
        if let Some(right) = &self.right {
            println!("Right");
            right.borrow().debug();
        }
    }
}
