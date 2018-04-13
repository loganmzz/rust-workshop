#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Color { red: u8, green: u8, blue: u8, }

mod color_should {
    use super::*;

    #[test]
    fn support_equality_comparison() {
        assert_eq!(Color { red: 32u8, green: 64u8, blue: 128u8 }, Color { red: 32u8, green: 64u8, blue: 128u8 });
    }

    #[test]
    fn be_a_copy_type() {
        let original = Color { red: 32u8, green: 64u8, blue: 128u8 };
        let copy = original;
        assert_eq!(original, copy);
    }
}

struct Car {
    brand: String,
    model: String,
}

impl Car {
    fn brand(&self) -> &String {
        &self.brand
    }

    fn model(&self) -> &String {
        &self.model
    }

    fn set_brand(&mut self, brand: &str) {
        self.brand = String::from(brand);
    }

    fn set_model(&mut self, model: &str) {
        self.model = String::from(model);
    }
}

mod car_should {
    use super::*;
    
    #[test]
    fn have_brand_getter_which_doesnt_consume_self() {
        let car = Car { brand: String::from("Dacia"), model: String::from("Logan") };
        assert_eq!(String::from("Dacia"), *car.brand());
        assert_eq!(String::from("Logan"), *car.model());
    }
    
    #[test]
    fn have_model_getter_which_doesnt_consume_self() {
        let car = Car { brand: String::from("Audi"), model: String::from("R8") };
        assert_eq!(String::from("R8"), *car.model());
        assert_eq!(String::from("Audi"), *car.brand());
    }
    
    #[test]
    fn have_brand_setter_which_doesnt_consume_self() {
        let mut car = Car { brand: String::from("Mercedes"), model: String::from("W176") };
        car.set_model("A-Class");
        assert_eq!(String::from("Mercedes"), *car.brand());
        assert_eq!(String::from("A-Class"), *car.model());
    }
}

struct CarBuilder {
    brand: String,
    model: String,
}

impl CarBuilder {
    
    fn new() -> Self {
        CarBuilder { brand: String::new(), model: String::new(), }
    }

    fn brand(mut self, brand: &str) -> Self {
        self.brand = String::from(brand);
        self
    }

    fn model(mut self, model: &str) -> Self {
        self.model = String::from(model);
        self
    }

    fn build(&self) -> Car {
        Car { brand: self.brand.clone(), model: self.model.clone() }
    }
}

mod car_builder_should {
    use super::*;

    #[test]
    fn build_car_with_empty_names_by_default() {
        let builder = CarBuilder::new();
        let car = builder.build();

        assert_eq!(String::from(""), *car.brand());
        assert_eq!(String::from(""), *car.model());
    }

    #[test]
    fn build_car_using_a_fluent_interface() {
        let builder = CarBuilder::new();
        let car = builder.brand("Tesla").model("Model S").build();

        assert_eq!(String::from("Tesla"), *car.brand());
        assert_eq!(String::from("Model S"), *car.model());
    }
}
