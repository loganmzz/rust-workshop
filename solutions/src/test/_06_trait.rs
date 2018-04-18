#![allow(dead_code)]

use std::fmt::Debug;
trait Ident: Debug {
  fn ident(&self) -> String;
}

trait Value {
   type Output;
   fn value(&self) -> Self::Output;
}

fn format(ident: &Ident) -> String {
    format!("[{:?}] {:?}", ident.ident(), ident)
}

#[derive(Debug)]
struct Entity { id: u64, name: String, }

impl Ident for Entity {
    fn ident(&self) -> String {
        self.id.to_string()
    }
}

impl Value for Entity {
    type Output = String;
    fn value(&self) -> Self::Output {
        self.name.clone()
    }
}

mod entity_should {
    use super::*;

    #[test]
    fn impl_ident_trait() {
        let entity = Entity { id: 42, name: String::from("an entity"), };
        assert_eq!("[\"42\"] Entity { id: 42, name: \"an entity\" }", format(&entity));
    }

    #[test]
    fn impl_value_trait() {
        let entity = Entity { id: 314, name: String::from("Pi"), };
        let value: String = <Entity as Value>::value(&entity);
        assert_eq!("Pi", &value);
    }
}

#[derive(Debug)]
struct Singleton;

impl Ident for Singleton {
    fn ident(&self) -> String {
        String::from("S")
    }
}

impl Value for Singleton {
    type Output = ();
    fn value(&self) -> Self::Output {
        ()
    }
}

mod singleton_should {
    use super::*;

    #[test]
    fn impl_ident_trait() {
        let singleton = Singleton;
        assert_eq!("[\"S\"] Singleton", format(&singleton));
    }

    #[test]
    fn impl_value_trait() {
        let singleton = Singleton;
        let value: () = <Singleton as Value>::value(&singleton);
        assert_eq!((), value);
    }
}
