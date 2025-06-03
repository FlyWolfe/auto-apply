use std::error::Error;
use thirtyfour::prelude::*;
use undetected_chromedriver::chrome;
use tokio::time::{sleep, Duration};
use tokio;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // scraping logic...

    // define the browser options
    let mut caps = DesiredCapabilities::firefox();
    // set the User-Agent header
    caps.add_arg("--disable-notifications");
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
    caps.add_arg(&format!("--user-agent={}", user_agent))?;

    // with the specified options
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    // visit the target page
    driver.goto("https://www.indeed.com").await?;
    let url = driver.current_url().await?;
    assert_eq!(url.as_ref(), "https://www.indeed.com/");

    //driver.wait(200);
    
    let elem_form = driver.find(By::Id("jobsearch")).await?;
    let elem_job = elem_form.find(By::Id("text-input-what")).await?;
    let elem_loc = elem_form.find(By::Id("text-input-where")).await?;
    
    sleep(Duration::from_millis(1000)).await;

    elem_job.send_keys("Senior Software Engineer").await?;
    sleep(Duration::from_millis(500)).await;
    elem_loc.clear().await?;
    sleep(Duration::from_millis(300)).await;
    elem_loc.send_keys("USA").await?;
    sleep(Duration::from_millis(600)).await;
    let elem_close = elem_form.find(By::Id("close")).await?;
    elem_close.click().await?;
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    //elem_button.click().await?;

    let html = driver.source().await?;
    println!("{html}");

    // close the browser and release its resources
    let _ = driver.leak();

    Ok(())
}
