use thirtyfour::prelude::*;
use undetected_chromedriver::Chrome;
use tokio::time::{sleep, Duration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // scraping logic...

    // with the specified options
    let driver: WebDriver = Chrome::new().await;
    
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
    elem_loc.send_keys(Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace + Key::Backspace).await?;
    sleep(Duration::from_millis(300)).await;
    elem_loc.send_keys("USA").await?;
    sleep(Duration::from_millis(600)).await;
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;

    sleep(Duration::from_millis(10000)).await;

    // close the browser and release its resources
    let _ = driver.quit().await?;

    Ok(())
}
