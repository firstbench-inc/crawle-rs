use reqwest::{Client, Proxy};

pub struct Fetcher {
    client: Client,
}



impl Fetcher {
    // Constructor to create a new Fetcher instance
    pub fn new() -> Self {
        let proxy = Proxy::all("tor:9050")
            .expect("tor proxy should be there");
        let client = Client::builder()
            .proxy(proxy)
            .build()
            .expect("should be able to build reqwest client");

        Fetcher { client }
    }

    // The fetch method
    pub async fn fetch<S: Into<String>>(
        &self,
        url: S,
    ) -> Result<String, reqwest::Error> {
        let res = self.client.get(url.into()).send().await?;
        println!("Status: {}", res.status());

        let text = res.text().await?;
        let is_tor = text.contains(
            "Congratulations. This browser is configured to use Tor.",
        ); 
        println!("Is Tor: {is_tor}");
        Ok(text)
    }
}