#[macro_use]
extern crate clap;

extern crate notify;

use std::path::PathBuf;

mod watch;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "treyzania <treyzania@gmail.com>")
        (about: "Opportunistically syncs file trees between multiple computers to a remote server.")
        (@arg remote_user: -c +takes_value "Override the username to connect to the remote host as.")
        (@arg remote_path: -p +takes_value "Override the remote path to store data in.  Default: ~/.synmux")
        (@arg label: +required "Label for this sync instance.")
        (@arg watch: +required "Watched directory.")
        (@arg rhost: +required "Remote host address."))
        .get_matches();

    let dir = matches.value_of("watch").unwrap();

    watch::dir_watch_prodecure(vec![PathBuf::from(dir)]);

}
