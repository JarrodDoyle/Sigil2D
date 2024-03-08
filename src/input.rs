use winit::{event::WindowEvent, keyboard::KeyCode};

pub struct Input {
    keys_held: Vec<KeyCode>,
    keys_just_pressed: Vec<KeyCode>,
    keys_just_released: Vec<KeyCode>,
    // TODO: Modifiers
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_held: vec![],
            keys_just_pressed: vec![],
            keys_just_released: vec![],
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();

        if let WindowEvent::KeyboardInput { event, .. } = event {
            let keycode = match event.physical_key {
                winit::keyboard::PhysicalKey::Code(code) => code,
                _ => KeyCode::Abort,
            };
            match event.state {
                winit::event::ElementState::Pressed => {
                    self.keys_held.push(keycode);
                    self.keys_just_pressed.push(keycode);
                }
                winit::event::ElementState::Released => {
                    self.keys_held.retain(|v| *v != keycode);
                    self.keys_just_released.push(keycode);
                }
            }
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_held.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keys_just_released.contains(&key)
    }
}
