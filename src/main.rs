#[macro_use] extern crate clap;
extern crate failure;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use chrono::{NaiveDateTime};

struct Config {}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Commit {
    subject: String,
    author_name: String,
    author_date: NaiveDateTime,
    committer_date: NaiveDateTime,
}

fn run(_config: &Config) -> Result<(), failure::Error> {
    let repo = git2::Repository::open_from_env()?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let mut commits: Vec<Commit> = vec![];
    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        commits.push(Commit{
            subject: commit.summary().unwrap().to_owned(),
            author_name: commit.author().name().unwrap().to_owned(),
            author_date: NaiveDateTime::from_timestamp(commit.author().when().seconds(), 0),
            committer_date: NaiveDateTime::from_timestamp(commit.committer().when().seconds(), 0),
        });
        let x = serde_json::to_string(commits.last().unwrap()).unwrap();
        println!("{}", x);
    }


    Ok(())
}

fn main() {
    let _args = app_from_crate!()
        .about("git log with structured output")
        .get_matches();
    if let Err(e) = run(&Config{}) {
        println!("HELLO");
        println!("{:?}", e);
        ::std::process::exit(1);
    }
}
