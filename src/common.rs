//to prevent conflicts with examples, i'll import things here and let examples use it from here if
//they need it
pub extern crate chrono;
pub extern crate tokio_core;
pub extern crate futures;

use std;
use std::io::{Write, Read};
use egg_mode;

use self::tokio_core::reactor::Core;

//This is not an example that can be built with cargo! This is some helper code for the other
//examples so they can load access keys from the same place.

pub struct Config {
    pub token: egg_mode::Token,
    pub user_id: u64,
    pub screen_name: String,
}

impl Config {
    pub fn load(core: &mut Core) -> Self {
        //IMPORTANT: make an app for yourself at apps.twitter.com and get your
        //key/secret into these files; these examples won't work without them
        let consumer_key = "W6ZmoV5Ts6klntYLSPePuEe4p".trim();
        let consumer_secret = "6OJ471837GPbjZa8TFot4dtvqju8lQrIjNSK9Gn5MBXkTXb8oB".trim();
        let handle = core.handle();

        let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);

        let mut config = String::new();
        let user_id: u64;
        let username: String;
        let token: egg_mode::Token;

        //look at all this unwrapping! who told you it was my birthday?
        if let Ok(mut f) = std::fs::File::open("twitter_settings") {
            f.read_to_string(&mut config).unwrap();

            let mut iter = config.split('\n');

            username = iter.next().unwrap().to_string();
            user_id = u64::from_str_radix(&iter.next().unwrap(), 10).unwrap();
            let access_token = egg_mode::KeyPair::new(iter.next().unwrap().to_string(),
                                                      iter.next().unwrap().to_string());
            token = egg_mode::Token::Access {
                consumer: con_token,
                access: access_token,
            };

            if let Err(err) = core.run(egg_mode::verify_tokens(&token, &handle)) {
                println!("We've hit an error using your old tokens: {:?}", err);
                println!("We'll have to reauthenticate before continuing.");
                std::fs::remove_file("twitter_settings").unwrap();
            } else {
                println!("Welcome back, {}!", username);
            }
        } else {
            let request_token = core.run(egg_mode::request_token(&con_token, "oob", &handle)).unwrap();

            println!("Go to the following URL, sign in, and give me the PIN that comes back:");
            println!("{}", egg_mode::authorize_url(&request_token));

            let mut pin = String::new();
            std::io::stdin().read_line(&mut pin).unwrap();
            println!("");

            let tok_result = core.run(egg_mode::access_token(con_token, &request_token, pin, &handle)).unwrap();

            token = tok_result.0;
            user_id = tok_result.1;
            username = tok_result.2;

            match token {
                egg_mode::Token::Access { access: ref access_token, .. } => {
                    config.push_str(&username);
                    config.push('\n');
                    config.push_str(&format!("{}", user_id));
                    config.push('\n');
                    config.push_str(&access_token.key);
                    config.push('\n');
                    config.push_str(&access_token.secret);
                },
                _ => unreachable!(),
            }

            let mut f = std::fs::File::create("twitter_settings").unwrap();
            f.write_all(config.as_bytes()).unwrap();

            println!("Welcome, {}, let's get this show on the road!", username);
        }

        //TODO: Is there a better way to query whether a file exists?
        if std::fs::metadata("twitter_settings").is_ok() {
            Config {
                token: token,
                user_id: user_id,
                screen_name: username,
            }
        } else {
            Self::load(core)
        }
    }
}

pub fn print_tweet(tweet: &egg_mode::tweet::Tweet) {
    if !tweet.truncated && !tweet.text.contains("RT") {
        println!("{:?}", tweet.text);
    }
}
