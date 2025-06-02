use fantoccini::{ClientBuilder, Locator};

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // first, go to Indeed
    c.goto("https://www.indeed.com").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://www.indeed.com/");

    c.wait()
        .for_element(Locator::Id(
            "text-input-what",
        ))
        .await?;
    c.wait()
        .for_element(Locator::Id(
            "text-input-where",
        ))
        .await?;

    let search_form = c.form(Locator::Id("jobsearch")).await?;
    search_form.set_by_name("q", "Software Engineer").await?;
    search_form.set_by_name("l", "USA").await?;
    search_form.submit().await?;

    // TODO: Figure out if a wait for load is needed here

    let mut job_listings = c.find_all(Locator::Css(".resultContent")).await?;
    
    while job_listings.len() <= 0 {
        job_listings = c.find_all(Locator::Css(".resultContent")).await?;
    }

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