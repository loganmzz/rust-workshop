#![allow(dead_code)]

fn get_false() {
}

fn get_42i32() {
}

fn get_pi() {
}

fn get_unit() -> usize {
    8
}

fn get_debug(num: i64, debug_string: &str) -> String {
    String::from("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_false_should_return_false_as_bool() {
        let result: bool = get_false();
        assert_eq!(false, result);
    }

    #[test]
    fn get_42i32_should_return_42_as_i32() {
        let result: i32 = get_42i32();
        assert_eq!(42, result);
    }

    #[test]
    fn get_pi_should_return_314_as_f64() {
        let result: f64 = get_pi();
        assert_eq!(3.14, result);
    }

    #[test]
    fn get_unit_should_return_unit() {
        let result: () = get_unit();
        assert_eq!((), result);
    }

    #[test]
    fn add_with_8u8_1024u16_should_return_1032u32() {
        let result: u32 = add(8u8, 1024u16);
        assert_eq!(1032, result);
    }

    #[test]
    fn get_debug_with_42i64_and_some_context_should_return_debug_string() {
        let result: String = get_debug(42i64, "some context");
        assert_eq!(String::from("\"some context\" debug: 42"), result);
    }
}
