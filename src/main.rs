#[macro_use]
extern crate clap;
extern crate chrono;
extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use chrono::NaiveDateTime;
use serde::ser::{SerializeSeq, Serializer};

// TODO: Let's add some config!
// Ideas:
// - dir of git repo, with or without .git specified
// - commits since some date, or relative duration
// - whether to output json-lines or json array
// - whether to output to a file
struct Config {}

#[derive(Serialize, Deserialize, Debug)]
struct StructuredCommit {
    author_name: String,
    author_date: NaiveDateTime,
    committer_name: String,
    committer_date: NaiveDateTime,
    subject: String,
    body: String,
}

fn run(_config: &Config) -> Result<(), failure::Error> {
    let repo = git2::Repository::open_from_env()?;

    let out = std::io::stdout();
    let mut ser = serde_json::Serializer::new(out);
    let mut seq = ser.serialize_seq(None)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let (author, committer) = (commit.author(), commit.committer());

        // TODO: Let's learn how to set the subject, body tuple in one go. Feels kinda clunky right
        // now. And why do we need <Vec<_>> on collect? And what's with all the to_owned() stuff?
        let pair = commit
            .message()
            .unwrap()
            .splitn(2, '\n')
            .collect::<Vec<_>>();
        let subject = pair[0];
        let body = if pair.len() > 1 { pair[1] } else { "" };
        let sc = StructuredCommit {
            author_name: author.name().unwrap().to_owned(),
            author_date: NaiveDateTime::from_timestamp(author.when().seconds(), 0),
            committer_name: committer.name().unwrap().to_owned(),
            committer_date: NaiveDateTime::from_timestamp(committer.when().seconds(), 0),
            subject: subject.to_owned(),
            body: body.to_owned(),
        };
        seq.serialize_element(&sc)?;
        // TODO: Why is this so slow? git log --stat is pretty snippy. This takes like 500ms per
        // commit.
        //
        // let a = if commit.parents().len() == 1 {
        //     let parent = commit.parent(0)?;
        //     Some(parent.tree()?)
        // } else {
        //     None
        // };
        // let b = commit.tree()?;
        // let diff = repo.diff_tree_to_tree(a.as_ref(), Some(&b), None)?;
        // println!("{}", diff.stats()?.insertions());
    }
    seq.end()?;

    Ok(())
}

fn main() {
    let _args = app_from_crate!()
        .about("git log with structured output")
        .get_matches();
    if let Err(e) = run(&Config {}) {
        // TODO: Is this the way one logs errors? Are there better idioms?
        error!("{}", e);
        ::std::process::exit(1);
    }
}
