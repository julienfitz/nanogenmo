#![feature(underscore_lifetimes)]

extern crate egg_mode;

mod common;

use common::tokio_core::reactor;

use egg_mode::search::{self, ResultType};

fn main() {
    // start with an initial search term
    let mut search_term = "love";

    // start with a word count of 0
    let mut word_count = 0;

    // start with an empty vector that we will fill up with tweets
    let mut final_tweets = vec![];

    // this stuff is from hyper/egg_mode for HTTP requests to the Twitter API
    let mut core = reactor::Core::new().unwrap();
    let config = common::Config::load(&mut core);
    let handle = core.handle();

    // until we get to the desired word count, keep looping
    while word_count < 100 {
        // print the current search_term so we can be sure it's being reassigned on each loop
        println!("{:?}", search_term);

        // search for tweets that include the search term, are not replies, retweets, media, or links
        // and are in English
        let search_results = &core.run(search::search(search_term.to_string() + " -filter:replies -filter:retweets -filter:media -filter:links lang:en")
                                     .result_type(ResultType::Recent)
                                     .count(10)
                                     .call(&config.token, &handle)).unwrap();

        // find the first tweet that's not a reply or truncated and get its text
        // (the search filter doesn't appear to always remove all replies)
        let tweet = &search_results.statuses.iter().find(|&toot| !&toot.text.contains("@") && !&toot.text.contains("t.co")).unwrap().text;

        // push a clone of the tweet onto final_tweets
        final_tweets.push(tweet.clone());

        // split the tweet by whitespace and clone so you can isolate the word count and last word
        let last_word = tweet.split_whitespace().last();
        let count = tweet.split_whitespace().count();

        // clone the count so we can add it to the total word count
        word_count += count;

        // reassign the value of the last word in the tweet to `search_term`
        search_term = last_word.unwrap();

        println!("{:?}", search_term);
        // print the current word count so we can be sure it's adding correctly
        println!("{:?}", word_count);
    }

    // print out all the tweets

    for i in &final_tweets {
        println!("{:?}", i);
    }
}
