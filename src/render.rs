use crate::ray::Ray;
use crate::vec3::Vec3;

///
pub fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc: Vec3 = r.origin() - center;

    let a: f32 = r.direction().dot(r.direction());
    let b: f32 = oc.dot(r.direction()) * 2.0;
    let c: f32 = oc.dot(&oc) - radius * radius;
    let discriminant: f32 = b * b - 4. * a * c;

    if discriminant < 0. {
        return -1.;
    } else {
        return (-b - discriminant.sqrt()) / (2. * a);
    }
}

///
pub fn color(r: &Ray) -> Vec3 {
    let t: f32 = hit_sphere(&Vec3(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n: Vec3 = (r.point_at_parameter(t) - Vec3(0., 0., 1.)).unit_vector();
        return Vec3(n.x() + 1., n.y() + 1., n.z() + 1.) * 0.5;
    }

    let unit_direction: Vec3 = r.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    // blends white and blue, depending on the up/downess of the y
    Vec3(1., 1., 1.) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}
