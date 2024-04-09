use std::fs;

use anyhow::{anyhow, Result};
use headless_chrome::{
    browser::default_executable, protocol::cdp::Page::CaptureScreenshotFormatOption, Browser,
    LaunchOptions,
};

pub fn shot(url: &str) -> Result<()> {
    let launch_options = LaunchOptions::default_builder()
        .path(Some(default_executable().map_err(|e| anyhow!(e))?))
        .window_size(Some((1920, 1080)))
        .build()?;
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    let result_object = tab.navigate_to(url)?.wait_until_navigated()?.evaluate("Math.max( document.body.scrollHeight, document.body.offsetHeight, document.documentElement.clientHeight, document.documentElement.scrollHeight, document.documentElement.offsetHeight )", false)?;
    let height: u64 = match result_object.value {
        Some(h) => match h.as_u64() {
            Some(h) => h,
            None => 1080,
        },
        None => 1080,
    };

    let launch_options = LaunchOptions::default_builder()
        .path(Some(default_executable().map_err(|e| anyhow!(e))?))
        .window_size(Some((1920, height as u32)))
        .build()?;
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    let image_bytes = tab
        .navigate_to(url)?
        .wait_until_navigated()?
        .capture_screenshot(CaptureScreenshotFormatOption::Png, Some(75), None, true)?;
    fs::write("./screen.png", image_bytes)?;
    Ok(())
}
