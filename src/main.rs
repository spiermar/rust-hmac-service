extern crate iron;
extern crate crypto;
extern crate rustc_serialize;
extern crate rand;

use iron::prelude::*;
use iron::status;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use rustc_serialize::hex::ToHex;

fn hmac() -> String {
	let hmac_key = "esU5jdGCbM7E/ME5WBECJ+BdX3kt7bcQ3HkeEK+W6ZQ=";
	let message = "Ceterum censeo Carthaginem esse delendam";
	let mut hmac = Hmac::new(Sha256::new(), hmac_key.to_string().as_bytes());
	hmac.input(message.as_bytes());
	return hmac.result().code().to_base64(STANDARD);
}

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, hmac())))
    }
    println!("Iron running on port 3000.");
    Iron::new(hello_world).http("0.0.0.0:3000").unwrap();
}
