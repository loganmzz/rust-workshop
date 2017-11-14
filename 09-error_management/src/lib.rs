#![allow(dead_code)]

enum InputError {
    Invalid,
}

#[cfg(test)]
mod double_result_should {
    use super::*;

    #[test]
    fn return_err_zero_when_ok_0() {
        assert_eq!(Err(DoubleError::Zero), double_result(Ok(0)));
    }
    
    #[test]
    fn return_ok_2_when_ok_1() {
        assert_eq!(Ok(2), double_result(Ok(1)));
    }
    
    #[test]
    fn return_err_invalidinput_when_err() {
        assert_eq!(Err(DoubleError::InvalidInput), double_result(Err(InputError::Invalid)));
    }
}

#[cfg(test)]
mod checked_division_should {
    use super::*;

    #[test]
    fn return_some_2_when_4_and_2() {
        assert_eq!(Some(2), checked_division(4, 2));
    }

    #[test]
    fn return_none_when_divising_by_0() {
        assert_eq!(None, checked_division(42, 0));
    }
}

#[cfg(test)]
mod open_box_with_should {
    use super::*;

    #[test]
    fn return_kind_message_when_some_value() {
        assert_eq!("Oh ! I like banana !", &open_box_with(Some("banana")));
    }

    #[test]
    fn return_disappointed_message_when_none() {
        assert_eq!("Oh... I'm so sad...", &open_box_with(None));
    }
}

#[cfg(test)]
mod monkey_should {
    use super::*;

    #[test]
    fn say_it_prefers_beer_when_giving_peanut_food() {
        assert_eq!("peanut is not so bad, but I prefer to drink beer", &Monkey.give(Some(Present::Food(String::from("peanut")))));
    }

    #[test]
    fn say_it_prefers_beer_when_giving_orange_juice() {
        assert_eq!("orange juice doesn't make me reach Balmer effect. Give me a beer", &Monkey.give(Some(Present::Drink(String::from("orange juice")))));
    }

    #[test]
    fn say_it_prefers_two_beer_when_giving_a_beer() {
        assert_eq!("Only one beer ? Give me another one", &Monkey.give(Some(Present::Beer)));
    }

    #[test]
    fn ask_for_a_beer_when_giving_none() {
        assert_eq!("Can I have a beer, please ?", &Monkey.give(None));
    }

}


#[cfg(test)]
mod division_should {
    use super::*;

    #[test]
    fn return_2_when_4_and_2() {
        assert_eq!(2, division(4, 2));
    }

    #[test]
    #[should_panic]
    fn panic_when_divising_by_0() {
        division(42, 0);
    }
}
