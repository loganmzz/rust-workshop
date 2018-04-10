#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
struct TuplePoint(i64, i64);

impl TuplePoint {
    fn x(self) -> i64 {
        self.0
    }

    fn y(self) -> i64 {
        self.1
    }
}

mod tuple_point_should {
    use super::*;
    
    #[test]
    fn be_created_as_tuple_of_i64() {
        let point = TuplePoint(1i64, 2i64);
    }

    #[test]
    fn have_access_to_first_element_through_x_method() {
        let point = TuplePoint(4, 8);
        assert_eq!(4i64, point.x());
    }

    #[test]
    fn have_access_to_second_element_through_y_method() {
        let point = TuplePoint(16, 32);
        assert_eq!(32i64, point.y());
    }

    #[test]
    fn support_debug() {
        assert_eq!(String::from("TuplePoint(64, 128)"), format!("{:?}", TuplePoint(64, 128)))
    }

    #[test]
    fn consider_equal_256_512_against_256_512() {
        assert_eq!(TuplePoint(256, 512), TuplePoint(256, 512));
    }

    #[test]
    fn consider_not_equal_256_512_against_512_256() {
        assert_ne!(TuplePoint(256, 512), TuplePoint(512, 256));
    }

    #[test]
    fn consider_not_equal_1024_2048_against_1024_4096() {
        assert_ne!(TuplePoint(1024, 2048), TuplePoint(1024, 4096));
    }

    #[test]
    fn consider_not_equal_1024_2048_against_4096_2048() {
        assert_ne!(TuplePoint(1024, 2048), TuplePoint(1024, 4096));
    }
}

#[derive(Debug)]
struct NamedPoint {
    x: i64,
    y: i64,
    name: String,
}

impl NamedPoint {
    fn new(x: i64, y: i64, name: &str) -> NamedPoint {
        NamedPoint { x, y, name: String::from(name) }
    }

    fn copy(base: NamedPoint, name: &str) -> NamedPoint {
        Self::new(base.x, base.y, name)
    }

    fn x(self) -> i64 {
        self.x
    }

    fn y(self) -> i64 {
        self.y
    }
}

mod named_point_should {
    use super::*;
    
    #[test]
    fn be_created_as_struct() {
        let point = NamedPoint { x: 1i64, y: 2i64, name: String::from("standard init") };
    }

    #[test]
    fn be_created_through_constructor() {
        let point = NamedPoint::new(4, 8, "constructor init");
        assert_eq!(4, point.x);
        assert_eq!(8, point.y);
        assert_eq!(String::from("constructor init"), point.name);
    }

    #[test]
    fn be_created_through_copy_constructor() {
        let base = NamedPoint { x: 16, y: 32, name: String::from("base") };
        let point = NamedPoint::copy(base, "new name");
        assert_eq!(16, point.x);
        assert_eq!(32, point.y);
        assert_eq!(String::from("new name"), point.name);
    }

    #[test]
    fn have_access_to_x_though_getter() {
        let point = NamedPoint { x: 64, y: 128, name: String::from("get_x") };
        assert_eq!(64, point.x());
    }

    #[test]
    fn have_access_to_second_element_through_y_method() {
        let point = NamedPoint { x: 16, y: 32, name: String::from("get_y") };
        assert_eq!(32i64, point.y());
    }

    #[test]
    fn support_debug() {
        let point = NamedPoint { x: 64, y: 128, name: String::from("debug") };
        assert_eq!(String::from("NamedPoint { x: 64, y: 128, name: \"debug\" }"), format!("{:?}", point));
    }
}
