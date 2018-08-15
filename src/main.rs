extern crate wlroots;

use wlroots::key_events::KeyEvent;
use wlroots::wlr_key_state::WLR_KEY_PRESSED;
use wlroots::xkbcommon::xkb::keysyms::*;
use wlroots::{CompositorHandle, InputManagerHandler, KeyboardHandle, KeyboardHandler};

struct InputManager;

impl InputManagerHandler for InputManager {
    fn keyboard_added(
        &mut self,
        _: CompositorHandle,
        _: KeyboardHandle,
    ) -> Option<Box<KeyboardHandler>> {
        Some(Box::new(Keyboard))
    }
}

struct Keyboard;

impl KeyboardHandler for Keyboard {
    fn on_key(&mut self, _: CompositorHandle, _: KeyboardHandle, key_event: &KeyEvent) {
        for key in key_event.pressed_keys() {
            if key_event.key_state() == WLR_KEY_PRESSED {
                println!("Key {} pressed", key);

                if key == KEY_Escape {
                    wlroots::terminate();
                } else if key == KEY_Alt_L {
                    println!("Alt_L is a modifier");
                } else if key == KEY_Control_L {
                    println!("Control_L is a modifier");
                } else {
                    println!("No binding for key {}", key);
                }
            } else {
                println!("Key {} released", key);
                return;
            }
        }
    }
}

fn main() {
    wlroots::utils::init_logging(wlroots::utils::WLR_DEBUG, None);

    let compositor_builder =
        wlroots::CompositorBuilder::new().input_manager(Box::new(InputManager));

    let compositor = compositor_builder.build_auto(());

    compositor.run();
}
