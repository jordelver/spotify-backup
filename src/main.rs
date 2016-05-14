extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use rustc_serialize::json;

static PLAYLIST_URL: &'static str = "https://api.spotify.com/v1/me/playlists";

#[derive(RustcDecodable, Debug)]
struct Playlists {
    items: Vec<Playlist>,
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
    let mut res = match client.get(PLAYLIST_URL)
      .headers(headers)
      .send() {
        Ok(res) => res,
        Err(_) => panic!("Whoops."),
    };

    // Read the response
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let playlists: Playlists = json::decode(&mut body).unwrap();
    println!("Playlists: {:?}", playlists);
}

