use fantoccini::{error, Client, ClientBuilder, Locator};
use serde_json::map;

pub async fn make_capabilities(s: &str) -> map::Map<String, serde_json::Value> {
    match s {
        "firefox" => {
            let mut caps = serde_json::map::Map::new();
            //let opts = serde_json::json!({ "args": ["--headless"] });
            //caps.insert("moz:firefoxOptions".to_string(), opts);
            caps
        }
        "chrome" => {
            let mut caps = serde_json::map::Map::new();
            let opts = serde_json::json!({
                "args": [
                    "--headless",
                    "--disable-gpu",
                    "--disable-dev-shm-usage",
                ],
            });
            caps.insert("goog:chromeOptions".to_string(), opts);
            caps
        }
        browser => unimplemented!("unsupported browser backend {}", browser),
    }
}


#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let cap = make_capabilities("firefox").await;
    let c = ClientBuilder::native().capabilities(cap).connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // first, go to Indeed
    //c.goto("https://www.indeed.com").await?;
    c.goto("https://abrahamjuliot.github.io/creepjs/").await?;
    let url = c.current_url().await?;
    //assert_eq!(url.as_ref(), "https://www.indeed.com/");

    c.wait()
        .for_element(Locator::Id("text-input-what",))
        .await?;
    c.wait()
        .for_element(Locator::Id("text-input-where",))
        .await?;

    let search_form = c.form(Locator::Id("jobsearch")).await?;
    search_form.set_by_name("q", "Software Engineer").await?;
    search_form.set_by_name("l", "USA").await?;
    search_form.submit().await?;

    // TODO: Figure out if a wait for load is needed here

    c.wait()
        .for_element(Locator::Css(".resultContent"))
        .await?;

    let job_listings = c.find_all(Locator::Css(".resultContent")).await?;

    for job in job_listings {
        // Extract Job Title
        let title_element = job.find(Locator::Css(r#"h2.jobTitle span[title]"#)).await?;
        let title = title_element.text().await?;

        // Extract Company Name
        let company_element = job.find(Locator::Css(r#"div.company_location [data-testid="company-name"]"#)).await?;
        let company = company_element.text().await?;


        // Extract Location
        let location_element = job.find(Locator::Css(r#"div.company_location [data-testid="text-location"]"#)).await?;
        let location = location_element.text().await?;

        println!("Title: {}, Company: {}, Location: {}", title, company, location);
    }

    c.close().await
}