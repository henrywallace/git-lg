git-lg
------

`git-lg` is like `git log` but with structured output.

[![Linux build status](https://travis-ci.org/henrywallace/git-lg.svg)](https://travis-ci.org/henrywallace/git-lg.svg)

If the binary is in your path you can run simply run `git lg` as git will
automatically pick it up as a subcommand. Use the `GIT_DIR=/path/to/.git` env
to control which repo to use.

While one could use git log [pretty
formatting](https://git-scm.com/docs/pretty-formats) like `git log
--pretty=format='%an,aI,%s'`, parsing the output can be cumbersome, possibly
having to deal with illegal sequences. This tool aims to be a thin layer to
allow easy parsing with other tools like `jq`: https://stedolan.github.io/jq/.

Currently the only output format is an array of commit objects, with only the
following fields:
```json
[
  {
    "author_name": "kennytm",
    "author_date": "2019-02-05T15:29:17",
    "committer_name": "GitHub",
    "committer_date": "2019-02-05T15:29:17",
    "subject": "Rollup merge of #58169 - boringcactus:patch-1, r=alexcrichton",
    "body": "\nUpdate contributor name in .mailmap\n\nfollowing up on email correspondence with @steveklabnik\n"
  }
]
```

### Example:
```
$ git clone https://github.com/rust-lang/rust /tmp/rust
$ export GIT_DIR=/tmp/rust/.git
$ git lg | jq -r '.[].author_name' | sort | uniq -c | sort -nrk1 | head
  14399 bors
   5503 Brian Anderson
   4791 Alex Crichton
   3834 Niko Matsakis
   2828 Patrick Walton
   2228 Graydon Hoare
   1980 Manish Goregaokar
   1757 Guillaume Gomez
   1689 kennytm
   1671 Steve Klabnik
```

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).
