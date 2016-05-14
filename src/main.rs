extern crate hyper;

use std::env;
use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

static PLAYLIST_URL: &'static str = "https://api.spotify.com/v1/me/playlists";

fn main() {
    let oauth_token = match env::var("OAUTH_TOKEN") {
      Ok(val) => val,
      Err(_) => panic!("OAUTH_TOKEN environment variable must be set"),
    };

    // Headers
    let mut headers = Headers::new();
    headers.set(
        Authorization(
            Bearer {
                token: oauth_token.to_owned()
            }
        )
    );

    headers.set(
      ContentType(
        Mime(
          TopLevel::Application,
          SubLevel::Json,
          vec![(Attr::Charset, Value::Utf8)]
        )
      )
    );

    println!("{}", headers);

    // Create a request
    let client = Client::new();
    let mut res = match client.get(PLAYLIST_URL)
      .headers(headers)
      .send() {
        Ok(res) => res,
        Err(_) => panic!("Whoops."),
    };

    // Read the response
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}

