use std::borrow::Borrow;
use std::fs::read_to_string;
use std::os::unix::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use clap::{App, Arg};
use path_absolutize::Absolutize;

use punt::{Error, FileType};

fn main() {
    let matches = App::new("Punt")
        .version("1.0")
        .about("A dotfiles manager?")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets the config to load")
                .default_value("punt.yml"),
        ).arg(
        Arg::with_name("tags")
            .multiple(true)
            .index(1)
            .required(false)
            .help("Specify extra tags to use")
    )
        .get_matches();

    let f = read_to_string(matches.value_of("config").unwrap()).unwrap();
    let conf: punt::Config = serde_yaml::from_str(&f).expect("Error parsing config");
    println!("{:?}", conf);
    let mut tags = conf.general.default_tags;
    match matches.values_of_lossy("tags") {
        Some(t) => {
            let mut t = t.clone();
            tags.append(&mut t)
        }
        None => {}
    }
    println!("{:?}", tags);
    for (src, action) in conf.files {
        if action.tags.is_empty() || action.tags.iter().any(|i| tags.clone().iter().any(|j| j == i)) {
            match do_action(&src.clone(), action.ft) {
                Ok(_) => {
                    println!("Completed action for {}", src);
                }
                Err(e) => {
                    println!("Error running action for {}: {:?}", src, e);
                }
            }
        }
    }
}

fn do_action(src: &str, action: FileType) -> punt::Result {
    let src = shellexpand::full(&src)?;
    let src: &str = src.borrow();
    let src = Path::new(src).absolutize()?;
    let src = src.to_str().ok_or(Error::Convert)?;

    match action {
        FileType::Link { dest } => {
            let target = shellexpand::full(&dest)?;
            let target: &str = target.borrow();
            let target = Path::new(target).absolutize()?;
            let target = target.to_str().ok_or(Error::Convert)?;

            fs::symlink(src, target).map_err(Error::Link)
        }
        FileType::Exec => {
            let mut cmd = Command::new("sh");
            cmd.arg("-c")
                .arg(src)
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .stderr(Stdio::inherit());
            let mut r = cmd.spawn().map_err(Error::CommandCreate)?;
            let ret = r.wait()?;
            if ret.success() {
                Ok(())
            } else {
                Err(Error::Run(ret.code().ok_or_else(|| Error::Run(-1))?))
            }
        }
    }
}
