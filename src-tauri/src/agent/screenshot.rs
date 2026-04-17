use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use image::GenericImageView;
use screenshots::Screen;
use std::path::Path;

pub struct ScreenshotTool;

impl ScreenshotTool {
    pub fn new() -> Self {
        ScreenshotTool
    }

    pub fn capture(&self, output_path: &str) -> Result<()> {
        let screens = Screen::all()?;
        
        if screens.is_empty() {
            return Err(anyhow::anyhow!("No screen found"));
        }

        let screen = &screens[0];
        let image = screen.capture()?;
        
        if Path::new(output_path).exists() {
            std::fs::remove_file(output_path)?;
        }
        
        image.save(output_path)?;
        
        Ok(())
    }

    pub fn encode_to_base64(&self, image_path: &str) -> Result<String> {
        let bytes = std::fs::read(image_path)?;
        Ok(STANDARD.encode(&bytes))
    }

    pub fn get_image_info(&self, image_path: &str) -> Result<(u32, u32)> {
        let img = image::open(image_path)?;
        let (width, height) = img.dimensions();
        Ok((width, height))
    }
}

pub fn smart_resize(
    height: u32,
    width: u32,
    factor: u32,
    min_pixels: u32,
    max_pixels: u32,
) -> Result<(u32, u32)> {
    fn round_by_factor(number: f64, factor: f64) -> u32 {
        ((number / factor).round() * factor) as u32
    }

    fn ceil_by_factor(number: f64, factor: f64) -> u32 {
        ((number / factor).ceil() * factor) as u32
    }

    fn floor_by_factor(number: f64, factor: f64) -> u32 {
        ((number / factor).floor() * factor) as u32
    }

    let height_f = height as f64;
    let width_f = width as f64;
    let factor_f = factor as f64;

    if height < 2 || width < 2 {
        return Err(anyhow::anyhow!(
            "height:{} or width:{} must be larger than factor:{}",
            height,
            width,
            factor
        ));
    }

    let max_aspect_ratio = 200.0;
    if (height_f.max(width_f) / height_f.min(width_f)) > max_aspect_ratio {
        return Err(anyhow::anyhow!(
            "absolute aspect ratio must be smaller than 200, got {} / {}",
            height,
            width
        ));
    }

    let h_bar = round_by_factor(height_f, factor_f);
    let w_bar = round_by_factor(width_f, factor_f);

    let pixels = h_bar * w_bar;

    if pixels > max_pixels {
        let beta = ((height_f * width_f) / max_pixels as f64).sqrt();
        let h_bar = floor_by_factor(height_f / beta, factor_f);
        let w_bar = floor_by_factor(width_f / beta, factor_f);
        Ok((h_bar, w_bar))
    } else if pixels < min_pixels {
        let beta = (min_pixels as f64 / (height_f * width_f)).sqrt();
        let h_bar = ceil_by_factor(height_f * beta, factor_f);
        let w_bar = ceil_by_factor(width_f * beta, factor_f);
        Ok((h_bar, w_bar))
    } else {
        Ok((h_bar, w_bar))
    }
}
