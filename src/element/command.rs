//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

pub mod simple;
pub mod paren;
pub mod brace;
pub mod double_paren;
pub mod if_command;
pub mod case_command;
pub mod while_command;
pub mod function_definition;

use nix::unistd::Pid;
use std::os::unix::prelude::RawFd;

use crate::{Feeder, ShellCore}; 

use self::double_paren::CommandDoubleParen;
use self::if_command::CommandIf;
use self::while_command::CommandWhile;
use self::paren::CommandParen;
use self::brace::CommandBrace;
use self::case_command::CommandCase;
use self::function_definition::FunctionDefinition;
use self::simple::SimpleCommand;

use std::process::exit;
use nix::unistd::{close, fork, ForkResult};

#[derive(PartialEq)]
pub enum CommandType {
    Case,
    While,
    If,
    Paren,
    //DoubleParen,
    Brace,
    //Simple,
    Null,
}

pub trait Command {
    fn exec(&mut self, conf: &mut ShellCore) {
        if self.no_connection() {
             self.exec_elems(conf);
             return;
        };

        unsafe {
            match fork() {
                Ok(ForkResult::Child) => {
                    if let Err(s) = self.set_child_io(conf){
                        eprintln!("{}", s);
                        exit(1);
                    }
                    self.exec_elems(conf);
                    close(1).expect("Can't close a pipe end");
                    exit(conf.vars["?"].parse::<i32>().unwrap());
                },
                Ok(ForkResult::Parent { child } ) => {
                    self.set_pid(child);
                    return;
                },
                Err(err) => panic!("Failed to fork. {}", err),
            }
        }
    }

    fn set_pipe(&mut self, pin: RawFd, pout: RawFd, pprev: RawFd);
    fn get_pid(&self) -> Option<Pid>;
    fn get_pipe_end(&mut self) -> RawFd;
    fn get_pipe_out(&mut self) -> RawFd;
    fn get_text(&self) -> String;
    fn set_child_io(&mut self, _conf: &mut ShellCore) -> Result<(), String> {Ok(())}
    fn exec_elems(&mut self, _conf: &mut ShellCore) {}
    fn no_connection(&self) -> bool { true }
    fn set_pid(&mut self, _pid: Pid) {}
}

pub fn parse(text: &mut Feeder, conf: &mut ShellCore) -> Option<Box<dyn Command>> {
    if let Some(a) =      CommandIf::parse(text,conf)                  {Some(Box::new(a))}
    else if let Some(a) = CommandWhile::parse(text, conf)              {Some(Box::new(a))}
    else if let Some(a) = CommandCase::parse(text, conf)               {Some(Box::new(a))}
    else if let Some(a) = CommandParen::parse(text, conf, false)       {Some(Box::new(a))}
    else if let Some(a) = CommandDoubleParen::parse(text, conf, false) {Some(Box::new(a))}
    else if let Some(a) = CommandBrace::parse(text, conf)              {Some(Box::new(a))}
    else if let Some(a) = FunctionDefinition::parse(text, conf)        {Some(Box::new(a))}
    else if let Some(a) = SimpleCommand::parse(text, conf)             {Some(Box::new(a))}
    else {None}
}
