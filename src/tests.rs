#[cfg(test)]
mod point3 {
    use crate::point3::Point3;
    #[test]
    fn add_two_points() {
        let p1: Point3 = Point3{x: 2.5, y: -3.4, z: 0.42};
        let p2: Point3 = Point3{x: -1.3, y: 1.4, z: 8.22};
        let p3: Point3 = Point3{x: 1.2, y: -2.0, z: 8.64};
        assert_eq!(p1 + p2, p3);
    }
    #[test]
    fn point_negation() {
        let p1: Point3 = Point3 { x: 1.0, y: 1.0, z: 1.0 };
        let negation: Point3 = Point3 { x: -1.0, y: -1.0, z: -1.0 };
        assert_eq!(-p1, negation);
    }
}