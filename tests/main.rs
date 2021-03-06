#[cfg(test)]
#[allow(unused_must_use)]
mod winmouse {
    use mouse_rs::types::keys::{MIDDLE, RIGHT};
    use mouse_rs::winmouse::*;

    #[test]
    fn move_and_press() {
        let mouse = WinMouse::new();
        mouse.move_to(500, 500);
        mouse.press(RIGHT).expect("Unable to press button");
        mouse.release(RIGHT).expect("Something went wrong");
    }

    #[test]
    fn scroll_wheel() {
        let mouse = WinMouse::new();
        mouse.wheel(1);
    }

    #[test]
    fn press_button() {
        let mouse = WinMouse::new();
        mouse.press(MIDDLE);
        mouse.release(MIDDLE);
    }

    #[test]
    fn print_post() {
        let mouse = WinMouse::new();
        println!("{:?}, {:?}", mouse.get_position().unwrap().y, mouse.get_position().unwrap().x);
    }
}
