#![allow(dead_code)]

enum Event {
    Mouse { x: u64, y: u64 },
    Keyboard(u64),
}

fn diagnose(event: Event) -> String {
    String::from("")
}

#[cfg(test)]
mod event_should {
    use super::*;

    #[test]
    fn support_diagnose_for_mouse() {
        let event = Event::Mouse { x: 42, y: 36 };
        assert_eq!("Mouse(42, 36)", &diagnose(event))
    }

    #[test]
    fn support_diagnose_for_keyboard() {
        let event = Event::Keyboard(64);
        assert_eq!("Keycode: 64", &diagnose(event))
    }
}
