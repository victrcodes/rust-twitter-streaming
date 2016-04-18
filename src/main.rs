extern crate url;
#[macro_use] extern crate hyper;
extern crate rustc_serialize;
extern crate oauthcli;

mod json_streamer;

use hyper::Client;
use hyper::header::Headers;
use std::io::BufReader;
use json_streamer::JsonObjectStreamer;

header! { (Authorization, "Authorization") => [String] }
header! { (Accept, "Accept") => [String] }
header! { (ContentType, "Content-Type") => [String] }

fn main() {

    //Change these values to your real Twitter API credentials
	let consumer_key = "CONSUMER_KEY";
	let consumer_secret = "CONSUMER_SECRET";
	let token = "OAUTH_TOKEN";
	let token_secret = "OAUTH_TOKEN_SECRET";

    //Track words
    let params: Vec<(String, String)> = vec![("track".to_string(), "london".to_string())];
	let url = "https://stream.twitter.com/1.1/statuses/filter.json";

	let header = oauthcli::authorization_header(
	    "POST",
	    url::Url::parse(url).unwrap(),
	    None, // Realm
	    consumer_key,
	    consumer_secret,
	    Some(token),
	    Some(token_secret),
	    oauthcli::SignatureMethod::HmacSha1,
	    &oauthcli::timestamp(),
	    &oauthcli::nonce(),
	    None, // oauth_callback
	    None, // oauth_verifier
	    params.clone().into_iter()
	);

    let client = Client::new();

	let mut headers = Headers::new();
	headers.set(Authorization(header.to_owned()));
	headers.set(Accept("*/*".to_owned()));
	headers.set(ContentType("application/x-www-form-urlencoded".to_owned()));

	let param_string: String = params.iter().map(|p| p.0.clone() + &"=".to_string() + &p.1).collect::<Vec<String>>().join("&");

	let res: hyper::client::response::Response = client.post(url).headers(headers).body(&param_string).send().unwrap();

	for obj in BufReader::new(res).json_objects() {
        println!("{:?}", obj.as_object().unwrap().get("text").unwrap().as_string().unwrap());
    }

}
