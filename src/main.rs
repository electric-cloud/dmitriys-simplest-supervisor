use std::fs;
use std::env;
// use std::process;
use std::process::Command;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

struct RunCommand {
    shell: String,
    command: String
}

impl RunCommand {
    pub fn new(shell:String, command:String) -> RunCommand {
        return RunCommand {
            shell,
            command
        };
    }
    pub fn run(&self) {
        let shell = &self.shell;
        let file = &self.command;
        Command::new(shell).arg(file).spawn().expect("Failed");
    }
    // TODO: Add monitoring for running process and relaunch if failed.
}

impl ToString for RunCommand {
    fn to_string(&self) -> String {
        return format!("Shell: {}, Command: {}", &self.shell, &self.command);
    }
}

fn main() {
    println!("Welcome to Dmitriy's Simplest Supervisor v0.1.0");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Mising init directory param. Usage: {} /path/to/dir", args.get(0).unwrap());
        std::process::exit(1);
    }
    let directory = args.get(1).unwrap();
    
    let files = get_tree(directory); //get_files_in_dir(directory);

    let mut runners: Vec<RunCommand> = vec![];
    for file in files {
        if let Some(x) = get_shell(file.as_str()) {
            runners.push(RunCommand::new(x, file));
        }
    }
    for r in runners {
        println!("Running: {}", r.to_string());
        r.run();
    }
    loop{};
}

fn get_shell(path: &str) -> Option<String> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {}", path),
    };
    let mut buf = BufReader::new(file);
    let mut shell = String::new();
    buf.read_line(&mut shell).expect("Unable to read from file");

    let len: usize = shell.len();
    if let Some("\n") = shell.get(len-1 .. len) {
        shell.pop();
    }
    if let Some("#!") = shell.get(..2) {
        return Some(shell.get(2..).unwrap().to_string());
    }
    // TODO: do it in more crossplatform way.
    return Some(String::from("powershell"));
}

fn get_tree(path: &str) -> Vec<String> {
    let init = get_files_in_dir(path);
    let mut retval: Vec<String> = vec![];
    for p in init {
        let filepath = Path::new(p.as_str());
        if filepath.is_dir() {
            let runpath = filepath.join("run");
            if runpath.exists() && !runpath.is_dir() {
                retval.push(String::from(runpath.to_str().unwrap()));
            }
            let runpath = filepath.join("run.bat");
            if runpath.exists() && !runpath.is_dir() {
                retval.push(String::from(runpath.to_str().unwrap()));
            }
            let runpath = filepath.join("run.ps1");
            if runpath.exists() && !runpath.is_dir() {
                retval.push(String::from(runpath.to_str().unwrap()));
            }
        }
    }
    return retval;
}

fn get_files_in_dir(path: &str) -> Vec<String> {
    if !Path::new(path).exists() {
        eprintln!("Path '{}' does not exist", path);
        std::process::exit(1);
    }
    if !Path::new(path).is_dir() {
        eprintln!("Path '{}' is not a directory", path);
        std::process::exit(1);
    }
    let paths = fs::read_dir(path).unwrap();
    let mut v = vec![];
    
    for path in paths {
        let p = path.unwrap().path().clone(); //display();
        v.push(String::from(p.to_str().unwrap()));
    }
    return v;
}
