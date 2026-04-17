use anyhow::Result;
use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub struct ComputerTools {
    enigo: Enigo,
}

impl ComputerTools {
    pub fn new() -> Self {
        ComputerTools {
            enigo: Enigo::new(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn get_screen_size() -> (i32, i32) {
        unsafe {
            let width = GetSystemMetrics(SM_CXSCREEN) as i32;
            let height = GetSystemMetrics(SM_CYSCREEN) as i32;
            (width, height)
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_screen_size() -> (i32, i32) {
        (1920, 1080)
    }

    pub fn reset(&mut self) {
        self.enigo.key_down(Key::Meta);
        self.enigo.key_down(Key::Layout('d'));
        self.enigo.key_up(Key::Layout('d'));
        self.enigo.key_up(Key::Meta);
        thread::sleep(Duration::from_millis(500));
    }

    pub fn press_key(&mut self, keys: Vec<String>) -> Result<()> {
        let cleaned_keys: Vec<String> = keys
            .into_iter()
            .map(|key| {
                let mut k = key.clone();
                if k.starts_with("keys=[") {
                    k = k[6..].to_string();
                }
                if k.ends_with(']') {
                    k = k[..k.len() - 1].to_string();
                }
                if (k.starts_with("['") || k.starts_with("[\"")) && k.len() > 2 {
                    k = k[2..].to_string();
                }
                if (k.ends_with("']") || k.ends_with("\"]")) && k.len() > 2 {
                    k = k[..k.len() - 2].to_string();
                }
                k.trim().to_string()
            })
            .collect();

        let key_map: std::collections::HashMap<&str, Key> = [
            ("arrowleft", Key::LeftArrow),
            ("arrowright", Key::RightArrow),
            ("arrowup", Key::UpArrow),
            ("arrowdown", Key::DownArrow),
            ("left", Key::LeftArrow),
            ("right", Key::RightArrow),
            ("up", Key::UpArrow),
            ("down", Key::DownArrow),
            ("ctrl", Key::Control),
            ("alt", Key::Alt),
            ("shift", Key::Shift),
            ("meta", Key::Meta),
            ("win", Key::Meta),
            ("enter", Key::Return),
            ("return", Key::Return),
            ("space", Key::Space),
            ("tab", Key::Tab),
            ("escape", Key::Escape),
            ("esc", Key::Escape),
            ("backspace", Key::Backspace),
            ("delete", Key::Delete),
            ("home", Key::Home),
            ("end", Key::End),
            ("pageup", Key::PageUp),
            ("pagedown", Key::PageDown),
        ]
        .into_iter()
        .collect();

        if cleaned_keys.len() > 1 {
            for key in &cleaned_keys {
                let k = key.to_lowercase();
                if let Some(mapped_key) = key_map.get(k.as_str()) {
                    self.enigo.key_down(*mapped_key);
                } else if key.len() == 1 {
                    self.enigo.key_down(Key::Layout(key.chars().next().unwrap()));
                }
            }

            for key in cleaned_keys.iter().rev() {
                let k = key.to_lowercase();
                if let Some(mapped_key) = key_map.get(k.as_str()) {
                    self.enigo.key_up(*mapped_key);
                } else if key.len() == 1 {
                    self.enigo.key_up(Key::Layout(key.chars().next().unwrap()));
                }
            }
        } else if let Some(key) = cleaned_keys.first() {
            let k = key.to_lowercase();
            if let Some(mapped_key) = key_map.get(k.as_str()) {
                self.enigo.key_click(*mapped_key);
            } else if key.len() == 1 {
                self.enigo.key_click(Key::Layout(key.chars().next().unwrap()));
            }
        }

        thread::sleep(Duration::from_millis(100));
        Ok(())
    }

    pub fn type_text(&mut self, text: &str) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        clipboard.set_text(text)?;
        
        self.enigo.key_down(Key::Control);
        self.enigo.key_down(Key::Layout('v'));
        self.enigo.key_up(Key::Layout('v'));
        self.enigo.key_up(Key::Control);
        
        thread::sleep(Duration::from_millis(200));
        Ok(())
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.mouse_move_to(x, y);
        thread::sleep(Duration::from_millis(100));
        self.enigo.mouse_move_to(x, y);
        Ok(())
    }

    pub fn left_click(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.mouse_move_to(x, y);
        thread::sleep(Duration::from_millis(100));
        self.enigo.mouse_click(MouseButton::Left);
        Ok(())
    }

    pub fn left_click_drag(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.mouse_down(MouseButton::Left);
        thread::sleep(Duration::from_millis(100));
        self.enigo.mouse_move_to(x, y);
        thread::sleep(Duration::from_millis(400));
        self.enigo.mouse_up(MouseButton::Left);
        self.enigo.mouse_move_to(x, y);
        Ok(())
    }

    pub fn right_click(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.mouse_move_to(x, y);
        thread::sleep(Duration::from_millis(100));
        self.enigo.mouse_click(MouseButton::Right);
        Ok(())
    }

    pub fn middle_click(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.mouse_move_to(x, y);
        thread::sleep(Duration::from_millis(100));
        self.enigo.mouse_click(MouseButton::Middle);
        Ok(())
    }

    pub fn double_click(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.mouse_move_to(x, y);
        thread::sleep(Duration::from_millis(100));
        self.enigo.mouse_click(MouseButton::Left);
        thread::sleep(Duration::from_millis(50));
        self.enigo.mouse_click(MouseButton::Left);
        Ok(())
    }

    pub fn triple_click(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.mouse_move_to(x, y);
        thread::sleep(Duration::from_millis(100));
        for _ in 0..3 {
            self.enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    pub fn scroll(&mut self, pixels: i32) -> Result<()> {
        self.enigo.mouse_scroll_y(pixels);
        Ok(())
    }

    pub fn wait(&mut self, seconds: f64) {
        thread::sleep(Duration::from_secs_f64(seconds));
    }
}
