#![feature(underscore_lifetimes)]

extern crate egg_mode;

mod common;

use common::tokio_core::reactor;

use egg_mode::search::{self, ResultType};

fn main() {
    // start with an initial search term
    let mut search_term = "love";
    let mut word_count = 0;
    // let mut final_tweets = vec![];

    let mut core = reactor::Core::new().unwrap();
    let config = common::Config::load(&mut core);
    let handle = core.handle();

    while word_count < 100 {
        // do the search

        println!("{:?}", search_term);
        let mut search_results = core.run(search::search(search_term.to_string() + " -filter:replies -filter:retweets -filter:media -filter:links lang:en")
                                     .result_type(ResultType::Recent)
                                     .count(10)
                                     .call(&config.token, &handle)).unwrap();
                                     
        // find the first tweet that's not a reply or truncated
        let mut tweet = &search_results.statuses.iter().find(|&toot| !&toot.text.contains("@") && !&toot.text.contains("t.co")).unwrap().text.clone();
        println!("{:?}", tweet);

        // push a copy of the tweet onto final_tweets

        // split the tweet by whitespace so you can get count and last
        let mut search_term_whitespace = &tweet.split_whitespace();

        // add the count to the total word count
        word_count += &search_term_whitespace.clone().count();

        // reassign the value of the last word in the tweet to `search_term`
        search_term = &search_term_whitespace.clone().last().unwrap();
        search_term = &search_term.clone();
        println!("{:?}", word_count);
    }

    // print out all the tweets
    // for i in &final_tweets {
    //     println!("{:?}", i);
    // }
}
