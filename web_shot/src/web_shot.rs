use anyhow::{anyhow, Result};
use headless_chrome::{
    browser::default_executable,
    protocol::cdp::{Page::CaptureScreenshotFormatOption, Target::CreateTarget},
    Browser, LaunchOptions,
};
use tokio::fs;

#[derive(Clone)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
}

impl Into<ImageFormat> for String {
    fn into(self) -> ImageFormat {
        let format = self.to_lowercase();
        if format.eq("png") {
            return ImageFormat::PNG;
        }
        if format.eq("jpeg") {
            return ImageFormat::JPEG;
        }
        if format.eq("webp") {
            return ImageFormat::WEBP;
        }
        ImageFormat::PNG
    }
}

impl Into<CaptureScreenshotFormatOption> for ImageFormat {
    fn into(self) -> CaptureScreenshotFormatOption {
        match self {
            ImageFormat::PNG => CaptureScreenshotFormatOption::Png,
            ImageFormat::JPEG => CaptureScreenshotFormatOption::Jpeg,
            ImageFormat::WEBP => CaptureScreenshotFormatOption::Webp,
        }
    }
}

pub struct Captureshot {
    url: String,
    width: u32,
    height: u32,
    quality: u32,
    full_page: bool,
    image_format: ImageFormat,
    screenshot_bytes: Option<Vec<u8>>,
}

impl Captureshot {
    pub fn new(
        url: String,
        width: u32,
        height: u32,
        quality: u32,
        full_page: bool,
        image_format: ImageFormat,
    ) -> Captureshot {
        Self {
            url,
            width,
            height,
            quality,
            full_page,
            image_format,
            screenshot_bytes: None,
        }
    }

    pub async fn shot(mut self) -> Result<Captureshot> {
        let mut height = self.height;
        let launch_options = LaunchOptions::default_builder()
            .path(Some(default_executable().map_err(|e| anyhow!(e))?))
            .window_size(Some((1260, 1080)))
            .build()?;
        let browser = Browser::new(launch_options)?;
        if self.full_page {
            let tab = browser.new_tab()?;
            let result_object = tab.navigate_to(&self.url)?.wait_until_navigated()?.evaluate("Math.max( document.body.scrollHeight, document.body.offsetHeight, document.documentElement.clientHeight, document.documentElement.scrollHeight, document.documentElement.offsetHeight )", false)?;
            height = match result_object.value {
                Some(h) => match h.as_u64() {
                    Some(h) => h as u32,
                    None => 1280,
                },
                None => 1280,
            };
        }

        let tab = browser.new_tab_with_options(CreateTarget {
            url: "about:blank".to_string(),
            width: Some(self.width),
            height: Some(height),
            browser_context_id: None,
            enable_begin_frame_control: None,
            new_window: Some(true),
            background: None,
        })?;

        let image_bytes = tab
            .navigate_to(&self.url)?
            .wait_until_navigated()?
            .capture_screenshot(
                self.image_format.clone().into(),
                Some(self.quality),
                None,
                true,
            )?;
        self.screenshot_bytes = Some(image_bytes);
        Ok(self)
    }

    pub async fn get_bytes(self) -> Result<Vec<u8>> {
        match self.screenshot_bytes {
            Some(data) => return Ok(data),
            None => return Err(anyhow!("get bytes must after shot page")),
        };
    }

    pub async fn write_to_file(self, filename: &str) -> Result<()> {
        match self.screenshot_bytes {
            Some(data) => {
                fs::write(filename, data).await?;
            }
            None => return Err(anyhow!("write to file must after shot page")),
        };
        Ok(())
    }
}
