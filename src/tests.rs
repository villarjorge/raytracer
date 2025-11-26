#[cfg(test)]
mod point3 {
    use crate::{point3::{Point3, cross, dot}};
    #[test]
    fn add_two() {
        let p1: Point3 = Point3{x: 2.5, y: -3.4, z: 0.42};
        let p2: Point3 = Point3{x: -1.3, y: 1.4, z: 8.22};
        let p3: Point3 = Point3{x: 1.2, y: -2.0, z: 8.64};
        assert_eq!(p1 + p2, p3);
    }
    #[test]
    fn subtract_two() {
        let p1: Point3 = Point3{x: 2.5, y: -3.4, z: 0.42};
        let p2: Point3 = Point3{x: -1.3, y: 1.4, z: 8.22};
        let p3: Point3 = Point3{x: 3.8, y: -4.8, z: -7.8};
        let resulting_point: Point3 = (p1 - p2) - p3;
        assert!(resulting_point.x.abs() < 1e-10);
        assert!(resulting_point.y.abs() < 1e-10);
        assert!(resulting_point.z.abs() < 1e-10);
    }
    #[test]
    fn negation() {
        let p1: Point3 = Point3 { x: 1.0, y: 1.0, z: 1.0 };
        let negation: Point3 = Point3 { x: -1.0, y: -1.0, z: -1.0 };
        assert_eq!(-p1, negation);
    }
    #[test]
    fn indexing() {
        let p1: Point3 = Point3 { x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!(p1[0], p1.x);
        assert_eq!(p1[1], p1.y);
        assert_eq!(p1[2], p1.z);
    }
    #[test]
    fn scale() {
        let p1: Point3 = Point3 { x: 1.0, y: 1.0, z: 1.0 };
        let c: f64 = 0.5;
        let p2: Point3 = Point3 { x: 0.5, y: 0.5, z: 0.5 };
        assert_eq!(p1*c, p2);
        assert_eq!(c*p1, p2);
    }
    #[test]
    fn coordinate_wise_multiplication() {
        let p1: Point3 = Point3 { x: 2.0, y: 3.0, z: 4.0 };
        let p2: Point3 = Point3 { x: 0.5, y: 0.5, z: 0.5 };
        let p3: Point3 = Point3 { x: 1.0, y: 1.5, z: 2.0 };
        assert_eq!(p1*p2, p3);
        assert_eq!(p2*p1, p3);
    }
    #[test]
    fn dot_product() {
        let p1: Point3 = Point3 { x: 2.0, y: 3.0, z: 4.0 };
        let p2: Point3 = Point3 { x: 0.5, y: 0.5, z: 0.5 };
        assert_eq!(dot(&p1, &p2), 4.5);
        assert_eq!(p1.dot(p2), 4.5);
        assert_eq!(p2.dot(p1), 4.5);
    }
    #[test]
    fn cross_product() {
        let x: Point3 = Point3 { x: 1.0, y: 0.0, z: 0.0 };
        let y: Point3 = Point3 { x: 0.0, y: 1.0, z: 0.0 };
        let z: Point3 = Point3 { x: 0.0, y: 0.0, z: 1.0 };

        // x -> y -> z -> x -> y -> z -> ...
        assert_eq!(cross(&x, &y), z);
        assert_eq!(cross(&z, &x), y);
        assert_eq!(cross(&y, &z), x);
    }
    #[test]
    fn lenghts() {
        let p1: Point3 = Point3 { x: 3.0, y: 4.0, z: 5.0 };
        assert_eq!(p1.length_squared(), 25.0);
        assert_eq!(p1.length(), 5.0)
    }
}