#![allow(dead_code)]

use std::ops::Add;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
struct Pair<A,B>(A, B);

impl<A: Add<Output=A>, B: Add<Output=B>> Add for Pair<A, B> {
    type Output = Self;
    
    fn add(self, that: Self) -> Self {
        Pair(self.0 + that.0, self.1 + that.1)
    }
}

trait Convert<E> {
    fn convert(&self) -> E;
}

impl<A: Debug, B: Debug> Convert<String> for Pair<A,B> {
    fn convert(&self) -> String {
        format!("{{{:?};{:?}}}", self.0, self.1)
    }
}

impl<A: Clone, B: Clone> Convert<(A,B)> for Pair<A,B> {
    fn convert(&self) -> (A,B) {
        (self.0.clone(), self.1.clone())
    }
}

mod pair_should {
    use super::*;

    #[test]
    fn support_debug_for_u64() {
        let p: Pair<u64, u64> = Pair(42, 128);
        assert_eq!("Pair(42, 128)", &format!("{:?}", p));
    }
    
    #[test]
    fn add_each_components_on_add() {
        let p1 =   Pair(1,  2);
        let p2 =   Pair(4,  8);
        assert_eq!(Pair(5, 10), p1 + p2);
    }

    #[test]
    fn support_convert_into_string() {
        let p = Pair(8, String::from("16"));
        let string: String = <Pair<_,_> as Convert<_>>::convert(&p);
        assert_eq!("{8;\"16\"}", &string);
    }
    
    fn compare_tuple<A: PartialEq + Debug,B: PartialEq + Debug>(actual: (A,B), expected0: A, expected1: B) {
        assert_eq!(expected0, actual.0, "index: 0");
        assert_eq!(expected1, actual.1, "index: 0");
    }
    #[test]
    fn support_convert_into_tuple() {
        let p = Pair(String::from("foobar"), 42);
        compare_tuple(p.convert(), String::from("foobar"), 42);
    }
}
