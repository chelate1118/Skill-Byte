use std::sync::{Arc, Mutex, MutexGuard};
use thirtyfour::prelude::*;

pub(super) struct Selenium {
    driver: Arc<Mutex<Option<WebDriver>>>
}

impl Selenium {
    pub(super) fn new() -> Selenium {
        let selenium = Selenium { driver: Arc::new(Mutex::new(None)) };

        let driver = Arc::clone(&selenium.driver);
        tokio::spawn(async move {
            Selenium::connect_driver(driver).await.unwrap(); // TODO: Error Handling
        });

        selenium
    }

    async fn connect_driver(driver: Arc<Mutex<Option<WebDriver>>>) -> WebDriverResult<()> {
        let caps = DesiredCapabilities::chrome();

        *driver.lock().unwrap()
            = Some(WebDriver::new("http://localhost:9515", caps).await?);

        Ok(())
    }

    pub(super) fn get_driver(&self) -> MutexGuard<'_, Option<WebDriver>> {
        self.driver.lock().unwrap()
    }
}

