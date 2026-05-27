use enigo::{Enigo, Key, KeyboardControllable};

pub struct VolumeController {
    enigo: Enigo,
}

impl VolumeController {
    pub fn new() -> Self {
        let enigo = Enigo::new();
        VolumeController { enigo }
    }

    pub fn volume_up(&mut self) {
        self.enigo.key_click(Key::VolumeUp);
    }

    pub fn volume_down(&mut self) {
        self.enigo.key_click(Key::VolumeDown);
    }
}
