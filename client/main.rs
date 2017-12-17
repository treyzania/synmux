#[macro_use]
extern crate clap;

extern crate notify;

use std::path::PathBuf;

mod watch;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "treyzania <treyzania@gmail.com>")
        (about: "Opportunistically file trees between multiple computers to a remote server.")
        (@arg INPUT: +required "Watched directory.")
        (@arg DATA: +required "Data directory, data for pending files are stored.")
        (@arg DEST: +required "Destination expression, passed directly to rsync.  Password-less login required!"))
        .get_matches();

    let dir = matches.value_of("INPUT").unwrap();
    let _data = matches.value_of("DATA").unwrap();
    let _dest = matches.value_of("DEST").unwrap();

    watch::dir_watch_prodecure(vec![PathBuf::from(dir)]);

}
