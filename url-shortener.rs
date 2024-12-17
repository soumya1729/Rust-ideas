use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct UrlShortener {
    url_map: Arc<Mutex<HashMap<String, String>>>,
    base_url: String,
}

impl UrlShortener {
    fn new(base_url: &str) -> Self {
        UrlShortener {
            url_map: Arc::new(Mutex::new(HashMap::new())),
            base_url: base_url.to_string(),
        }
    }

    fn shorten(&self, original_url: &str) -> String {
        let mut rng = rand::thread_rng();
        let short_id: String = (0..8).map(|_| rng.sample(Alphanumeric) as char).collect();

        let short_url = format!("{}/{}", self.base_url, short_id);

        let mut url_map = self.url_map.lock().unwrap();
        url_map.insert(short_id.clone(), original_url.to_string());

        short_url
    }

    fn resolve(&self, short_url: &str) -> Option<String> {
        let short_id = short_url.split('/').last().unwrap_or("");
        let url_map = self.url_map.lock().unwrap();
        url_map.get(short_id).cloned()
    }
}

fn main() {
    let base_url = "http://short.ly";
    let shortener = UrlShortener::new(base_url);

    let original_url = "https://www.example.com/very/long/url";
    let short_url = shortener.shorten(original_url);

    println!("Original URL: {}", original_url);
    println!("Shortened URL: {}", short_url);

    if let Some(resolved_url) = shortener.resolve(&short_url) {
        println!("Resolved URL: {}", resolved_url);
    } else {
        println!("Shortened URL could not be resolved.");
    }
}
