#![allow(dead_code)]

///
/// _Note: May not work in all cases but must work if values are in same scope._
/// 
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

mod longest_should {
    use super::*;

    #[test]
    fn return_aa_when_a_and_aa() {
        let a = String::from("a");
        let aa = String::from("aa");
        assert_eq!("aa", longest(&a, &aa));
    }

    #[test]
    fn return_bb_when_bb_and_b() {
        let b = String::from("b");
        let bb = String::from("bb");
        assert_eq!("bb", longest(&bb, &b));
    }
}
