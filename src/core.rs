//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;
use std::process::exit;
use std::env;
use std::path::Path;

pub struct Flags {
    pub v: bool,
    pub x: bool,
    pub i: bool,
    pub d: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags{
            v: false, 
            x: false,
            i: false,
            d: false,
        }
    }
}

pub struct History {
    pub commandline: String,
    pub charwidths: Vec<u8>, 
}

pub struct ShellCore {
    pub internal_commands: HashMap<String, fn(args: &mut Vec<String>) -> i32>,
    pub vars: HashMap<String, String>,
    pub history: Vec<History>,
    pub flags: Flags,
}

impl ShellCore {
    pub fn new() -> ShellCore {
        let mut conf = ShellCore{
            internal_commands: HashMap::new(),
            vars: HashMap::new(),
            history: Vec::new(),
            flags: Flags::new(),
        };

        conf.internal_commands.insert("exit".to_string(), Self::exit);
        conf.internal_commands.insert("pwd".to_string(), Self::pwd);
        conf.internal_commands.insert("cd".to_string(), Self::cd);

        conf
    }

    pub fn get_var(&self, key: &String) -> String {
        if let Some(s) = self.vars.get(&key as &str){
            return s.to_string();
        };

        if let Ok(s) = env::var(&key) {
            return s.to_string();
        };

        "".to_string()
    }

    pub fn get_internal_command(&self, name: &String) -> Option<fn(args: &mut Vec<String>) -> i32> {
        if self.internal_commands.contains_key(name) {
            Some(self.internal_commands[name])
        }else{
            None
        }
    }
    /////////////////////////////////
    /* INTERNAL COMMANDS HEREAFTER */
    /////////////////////////////////
    pub fn exit(_args: &mut Vec<String>) -> i32 {
        exit(0);
    }

    pub fn pwd(_args: &mut Vec<String>) -> i32 {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            _        => panic!("Cannot get current dir"),
        }
        0
    }

    pub fn cd(args: &mut Vec<String>) -> i32 {
        if args.len() == 0 {
            eprintln!("Bug of this shell");
        }else if args.len() == 1 {
            let var = env::var("HOME").expect("HOME is not defined");
            args.push(var);
        };

        let path = Path::new(&args[1]);
        if env::set_current_dir(&path).is_ok() {
            0
        }else{
            eprintln!("Not exist directory");
            1
        }
    }
}
