use std::fs;

use anyhow::{anyhow, Result};
use headless_chrome::{
    browser::default_executable, protocol::cdp::Page::CaptureScreenshotFormatOption, Browser,
    LaunchOptions,
};

pub struct Captureshot {
    url: String,
    width: u32,
    height: u32,
    quality: u32,
    full_page: bool,
    screenshot_bytes: Option<Vec<u8>>,
}

impl Captureshot {
    pub fn new(url: String, width: u32, height: u32, quality: u32, full_page: bool) -> Captureshot {
        Self {
            url,
            width,
            height,
            quality,
            full_page,
            screenshot_bytes: None,
        }
    }

    pub fn shot(mut self) -> Result<Captureshot> {
        let mut height = self.height;
        let launch_options = LaunchOptions::default_builder()
            .path(Some(default_executable().map_err(|e| anyhow!(e))?))
            .window_size(Some((1280, 1080)))
            .build()?;
        let browser = Browser::new(launch_options)?;
        let tab = browser.new_tab()?;
        let result_object = tab.navigate_to(&self.url)?.wait_until_navigated()?.evaluate("Math.max( document.body.scrollHeight, document.body.offsetHeight, document.documentElement.clientHeight, document.documentElement.scrollHeight, document.documentElement.offsetHeight )", false)?;
        if self.full_page {
            height = match result_object.value {
                Some(h) => match h.as_u64() {
                    Some(h) => h as u32,
                    None => 1080,
                },
                None => 1080,
            };
        }

        let launch_options = LaunchOptions::default_builder()
            .path(Some(default_executable().map_err(|e| anyhow!(e))?))
            .window_size(Some((self.width, height)))
            .build()?;
        let browser = Browser::new(launch_options)?;
        let tab = browser.new_tab()?;
        let image_bytes = tab
            .navigate_to(&self.url)?
            .wait_until_navigated()?
            .capture_screenshot(
                CaptureScreenshotFormatOption::Png,
                Some(self.quality),
                None,
                true,
            )?;
        self.screenshot_bytes = Some(image_bytes);
        Ok(self)
    }

    pub fn write_to_file(self, filename: &str) -> Result<()> {
        match self.screenshot_bytes {
            Some(data) => {
                fs::write(filename, data)?;
            }
            None => return Err(anyhow!("write to file must after shot page")),
        };
        Ok(())
    }
}
