extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use rustc_serialize::json;

static PLAYLIST_URL: &'static str = "https://api.spotify.com/v1/me/playlists?limit=50&offset=0";

#[derive(RustcDecodable, Debug)]
struct Playlists {
    items: Vec<Playlist>,
    next: String,
}

#[derive(RustcDecodable, Debug)]
struct Playlist {
    name: String,
}

fn main() {
    let oauth_token = match env::var("OAUTH_TOKEN") {
      Ok(val) => val,
      Err(_) => panic!("OAUTH_TOKEN environment variable must be set"),
    };

    let body = get_playlists(oauth_token, PLAYLIST_URL);

    let playlists: Playlists = json::decode(&body).unwrap();

    for entry in playlists.items.iter() {
        println!("{}", entry.name);
    }
}

fn get_playlists(oauth_token: String, url: &str) -> String {
    // Headers
    let mut headers = Headers::new();
    headers.set(
        Authorization(
            Bearer {
                token: oauth_token.to_owned()
            }
        )
    );

    headers.set(ContentType::json());

    // Create a request
    let client = Client::new();
    let mut res = match client.get(url)
      .headers(headers)
      .send() {
        Ok(res) => res,
        Err(_) => panic!("Whoops."),
    };

    // Read the response
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

