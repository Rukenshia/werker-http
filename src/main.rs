extern crate time;

use std::io::prelude::*;

fn exec(command: &str, args: &[&str]) -> std::process::Output {
    std::process::Command::new(command).args(args).output().unwrap_or_else(|_| {
        panic!("failed")
    })
}

fn mkdir(path: &str) {
    std::fs::create_dir(path).ok();
}

fn create_logfile(path: &str) -> std::fs::File {
    std::fs::File::create(path).or_else(|e| -> Result<_, std::io::Error> {
        panic!(format!("could not create file {}: {}", path, e))
    }).unwrap()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 7 {
        panic!("invalid args")
    }

    mkdir("./.werker-logs");
    let tm = time::now();
    let mut f = create_logfile(&*format!(".werker-logs/werker-http_{}.log", tm.strftime("%Y-%m-%d-%H-%M-%S").unwrap()));

    mkdir(&*args[6]);
    mkdir(&*format!("{}/{}", args[5], args[1]));

    f.write_all(b"creating dummy file\n").ok();
    std::fs::File::create(format!("{}/{}/{}", args[5], args[1], args[4]))
        .ok().expect("could not create dummy file");

    f.write_all(b"downloading file\n").ok();
    println!("{}/{}", args[6], args[4]);
    println!("{}", args[3]);
    exec("wget", &["-O", &*format!("{}/{}", args[6], args[4]), &*args[3]]);
    f.write_all(b"moving output file\n").ok();

    std::fs::rename(format!("{}/{}", args[6], args[4]), format!("{}/{}/{}", args[5], args[1], args[4]))
        .ok().expect("could not move file\n");
    f.write_all(b"finished").ok();
}
