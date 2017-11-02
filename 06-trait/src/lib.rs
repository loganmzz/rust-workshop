#![allow(dead_code)]

fn format(ident: &Ident) -> String {
    format!("[{:?}] {:?}", ident.ident(), ident)
}

struct Entity { id: u64, name: String, }

#[cfg(test)]
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

struct Singleton;

#[cfg(test)]
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
