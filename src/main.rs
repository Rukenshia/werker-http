extern crate time;

use std::io::prelude::*;
use std::fs;
use std::io;
use std::process;

fn exec(command: &str, args: &[&str]) -> process::Output {
    match process::Command::new(command).args(args).output() {
        Err(e) => panic!(format!("failed to execute command {}: {}", command, e)),
        Ok(v) => v,
    }
}

struct LogFile {
    handle: fs::File
}

impl LogFile {
    pub fn new(path: &str) -> Result<LogFile, std::io::Error> {
        let handle = try!(fs::File::create(path));
        Ok(LogFile{ handle: handle })
    }

    pub fn write(&mut self, message: &str) {
        let msg = message.to_string() + "\n";
        if let Err(..) = self.handle.write_all(msg.as_bytes()) {
            println!("could not write to log file");
        }
    }
}


fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 7 {
        panic!("invalid args")
    }

    let anime = &args[1];
    let episode = &args[2];
    let url = &args[3];
    let filename = &args[4];
    let outdir = &args[5];
    let intermediate_dir = &args[6];

    if let Err(e) = fs::create_dir("./.werker-logs") {
        match e.kind() {
            io::ErrorKind::AlreadyExists => {},
            _ => panic!("could not create werker-logs directory: {}", e),
        }
    }

    let tm = time::now();
    let tm_formatted = tm.strftime("%Y-%m-%d-%H-%M-%S").unwrap();
    let log_name = format!(".werker-logs/werker-http-{}-{}_{}", anime, episode, tm_formatted);
    let intermediate_file = format!("{}/{}", intermediate_dir, filename);
    let output_file = format!("{}/{}/{}", outdir, anime, filename);

    let mut log = LogFile::new(&log_name).expect("could not create log file");

    // Create an empty file in the output directory to signalize that we are downloading the file.
    log.write("creating dummy file");
    if let Err(e) = fs::File::create(&output_file) {
        println!("could not create dummy file: {}", e);
    }

    // Download the actual file
    log.write("downloading file");
    exec("wget", &["-O", &intermediate_file, &url]);
    log.write("moving output file");

    // Move the downloaded file to the output directory
    match fs::rename(intermediate_file, output_file) {
        Err(e) => {
            log.write(&format!("could not move output file: {}", e));
            log.write("finished with error(s).");
        },
        Ok(..) => log.write("finished."),
    }
}
