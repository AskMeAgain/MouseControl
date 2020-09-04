use inputbot::{MouseCursor, handle_input_events, MouseButton, KeySequence};
use inputbot::KeybdKey::{RightKey, QKey, SpaceKey, RKey, EKey};
use std::thread::sleep;
use std::time::Duration;
use inputbot::MouseButton::{LeftButton, RightButton};
use enigo::{Enigo, MouseControllable, KeyboardControllable};
use minifb::{WindowOptions, Window};

const MAX_WAIT_UNTIL_ACTIVE: i32 = 30;

const WIDTH: usize = 1;
const HEIGHT: usize = 360;

fn main() {
    SpaceKey.bind(|| {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        let mut window_options = WindowOptions::default();
        window_options.borderless = true;
        window_options.transparency = true;

        let mut window = Window::new(
            "",
            WIDTH,
            HEIGHT,
            window_options,
        )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        main_loop(&mut buffer, &mut window);
    });

    handle_input_events();
}

fn main_loop(buffer: &mut Vec<u32>, window: &mut Window) {
    let mut counter = 0;

    while SpaceKey.is_pressed() {
        counter += 1;
        draw_line(buffer);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    press_button(counter);
}

fn draw_line(buffer: &mut Vec<u32>, x: i32) {

    let i = x;
    while x < buffer.len(){

    }

    for i in buffer.iter_mut() {
        *i = 0xffff0000;
    }
}

fn press_button(counter: i32) {
    let mut enigo = Enigo::new();

    if counter > 0 {
        if counter < MAX_WAIT_UNTIL_ACTIVE {
            println!("Pressing normal spacebar");
            enigo.key_click(enigo::Key::Space);
        } else {
            println!("Clicking on stuff");
            enigo.mouse_move_to(500, 200);
            enigo.mouse_click(enigo::MouseButton::Right);
        }
    }
}