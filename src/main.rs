#![feature(underscore_lifetimes)]
#![feature(nll)]

extern crate egg_mode;

mod common;

use common::tokio_core::reactor;

use egg_mode::search::{self, ResultType};
use std::io::prelude::*;
use std::fs::File;

fn any_alphabetic(word: &str) -> bool {
    word.chars().all(|c| c.is_alphabetic()) // && !word.contains("#")
}

fn main() {
    // start with an initial search term
    let mut search_term = "love";

    // start with a word count of 0
    let mut word_count = 0;

    // start with an empty vector that we will fill up with tweets
    let mut final_tweets = "".to_string();

    // create an empty file
    let mut file = File::create("nanogenmo.txt");

    // this stuff is from hyper/egg_mode for HTTP requests to the Twitter API
    let mut core = reactor::Core::new().unwrap();
    let config = common::Config::load(&mut core);
    let handle = core.handle();

    // until we get to the desired word count, keep looping
    while word_count < 1000 {
        // print the current search_term so we can be sure it's being reassigned on each loop
        println!("INITIAL SEARCH TERM: {:?}", search_term);

        // search for tweets that include the search term, are not replies, retweets, media, or links
        // and are in English
        let search_results = &core.run(search::search(search_term.to_string() + " -filter:replies -filter:retweets -filter:media -filter:links lang:en")
                                     .result_type(ResultType::Recent)
                                     .count(50)
                                     .call(&config.token, &handle)).unwrap();

        // find the first tweet that's not a reply or truncated and get its text
        // (the search filter doesn't appear to always remove all replies)
        let tweet = &search_results.statuses.iter().find(|&toot| !&toot.text.contains("@") && !&toot.text.contains("t.co") && !final_tweets.contains(&toot.text)).unwrap().text;

        // push a space onto final_tweets so they don't just run together
        final_tweets.push_str("\n");
        // push a clone of the tweet onto final_tweets
        final_tweets.push_str(&tweet.clone());

        // split the tweet by whitespace and clone so you can isolate the word count and last word
        let mut last_word = tweet.split_whitespace().nth(0);
        for i in 0..tweet.split_whitespace().count() {
            if !any_alphabetic(last_word.unwrap()) {
                last_word = tweet.split_whitespace().nth(i+1);
                break;
            }
        }

        let count = tweet.split_whitespace().count();

        // clone the count so we can add it to the total word count
        word_count += count;

        // reassign the value of the last word in the tweet to `search_term`
        search_term = last_word.unwrap();
    }

    let mut really_final_tweets = "".to_string();

    // since API rate limiting is a thing, for now we are just repeating the first thousand words
    // 50 times 
    for i in 1..51 {
        really_final_tweets.push_str(&final_tweets.clone());
    }

    file.unwrap().write_all(really_final_tweets.as_bytes());
}
