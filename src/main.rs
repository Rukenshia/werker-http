extern crate time;

use std::io::prelude::*;

fn exec(command: &str, args: &[&str]) -> std::process::Output {
    std::process::Command::new(command).args(args).output().unwrap_or_else(|e| {
        panic!(format!("failed to execute command {}: {}", command, e))
    })
}

struct LogFile {
    handle: std::fs::File
}

impl LogFile {
    pub fn new(path: &str) -> Result<LogFile, String> {
        let handle = std::fs::File::create(path);
        if handle.is_err() {
            // I dont know how to get the actual error message.
            return Err("error creating file".to_string())
        }
        Ok(LogFile{ handle: handle.unwrap() })
    }

    pub fn write(&mut self, message: &str) {
        self.handle.write_all(&*message.to_string().into_bytes()).or_else(|_| {
            println!("could not write to log file");
            Err(())
        }).ok();
    }
}


fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 7 {
        panic!("invalid args")
    }

    let anime = &*args[1];
    let url = &*args[3];
    let filename = &*args[4];
    let outdir = &*args[5];
    let intermediate_dir = &*args[6];

    std::fs::create_dir("./.werker-logs").or_else(|e| -> Result<(), ()> {
        if e.raw_os_error().unwrap() == 17 {
            return Ok(())
        }
        panic!(format!("could not create log directory: {}", e))
    }).ok();

    let tm = time::now();

    let file = LogFile::new(&*format!(".werker-logs/werker-http_{}.log", tm.strftime("%Y-%m-%d-%H-%M-%S").unwrap()));

    if file.is_err() {
        panic!("could not create log file");
    }
    let mut log = file.unwrap();
    // Create an empty file in the output directory to signalize that we are downloading the file.
    log.write("creating dummy file\n");
    std::fs::File::create(format!("{}/{}/{}", outdir, anime, filename))
        .or_else(|e| -> Result<std::fs::File, _> {
            log.write(&*format!("could not create dummy file: {}\n", e));
            Err(())
        })
        .ok();

    // Download the actual file
    log.write("downloading file\n");
    log.write(&*format!("{}/{}", intermediate_dir, filename));
    exec("wget", &["-O", &*format!("{}/{}", intermediate_dir, filename), &*url]);
    log.write("moving output file\n");

    // Move the downloaded file to the output directory
    let mv_result = std::fs::rename(format!("{}/{}", intermediate_dir, filename), format!("{}/{}/{}", outdir, anime, filename))
        .or_else(|e| {
            log.write(&*format!("could not move file: {}\n", e));
            Err(())
        });

    if mv_result.is_ok() {
        log.write("finished\n");
    } else {
        log.write("finished with error(s).\n");
    }
}
