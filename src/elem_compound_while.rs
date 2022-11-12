//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use crate::abst_elems::PipelineElem;
use std::os::unix::prelude::RawFd;
use crate::elem_script::Script;
use crate::elem_redirect::Redirect;
use crate::element_list::Compound;
use nix::unistd::Pid;
use crate::utils_io::*;

/* ( script ) */
pub struct CompoundWhile {
    pub conddo: Option<(Script, Script)>,
    text: String,
    pid: Option<Pid>,
    fds: FileDescs,
    my_type: Compound, 
}

impl PipelineElem for CompoundWhile {
    fn get_pid(&self) -> Option<Pid> { self.pid }
    fn set_pid(&mut self, pid: Pid) { self.pid = Some(pid); }
    fn no_connection(&self) -> bool { self.fds.no_connection() }

    fn set_pipe(&mut self, pin: RawFd, pout: RawFd, pprev: RawFd) {
        self.fds.pipein = pin;
        self.fds.pipeout = pout;
        self.fds.prevpipein = pprev;
    }

    fn set_child_io(&self){
        self.fds.set_child_io();
    }

    fn get_pipe_end(&mut self) -> RawFd { self.fds.pipein }
    fn get_pipe_out(&mut self) -> RawFd { self.fds.pipeout }
    fn get_text(&self) -> String { self.text.clone() }

    fn exec_elems(&mut self, conf: &mut ShellCore) {
        loop {
            if let Some((cond, doing)) = &mut self.conddo {
                cond.exec(conf);
                if conf.vars["?"] != "0" {
                    conf.set_var("?", "0");
                    break;
                }
                doing.exec(conf);
            }
        }
    }
}

impl CompoundWhile {
    pub fn new() -> CompoundWhile{
        CompoundWhile {
            conddo: None,
            text: "".to_string(),
            fds: FileDescs::new(),
            pid: None,
            my_type: Compound::While,
        }
    }


    fn parse_cond_do_pair(text: &mut Feeder, conf: &mut ShellCore, ans: &mut CompoundWhile) -> bool {
        ans.text += &text.request_next_line(conf);

        let cond = if let Some(s) = Script::parse(text, conf, &ans.my_type) {
            ans.text += &s.text;
            s
        }else{
            return false;
        };

        ans.text += &text.request_next_line(conf);

        if text.starts_with( "do"){
            ans.text += &text.consume(2);
        }

        ans.text += &text.request_next_line(conf);

        let doing = if let Some(s) = Script::parse(text, conf, &ans.my_type) {
            ans.text += &s.text;
            s
        }else{
            return false;
        };

        ans.text += &text.request_next_line(conf);

        ans.conddo = Some( (cond, doing) );
        true
    }

    pub fn parse(text: &mut Feeder, conf: &mut ShellCore) -> Option<CompoundWhile> {
        if text.len() < 5 || ! text.starts_with( "while") {
            return None;
        }

        let backup = text.clone();

        let mut ans = CompoundWhile::new();
        ans.text += &text.consume(5);

        if ! CompoundWhile::parse_cond_do_pair(text, conf, &mut ans) {
            text.rewind(backup);
            return None;
        }

        if text.starts_with( "done"){
            ans.text += &text.consume(4);
        }else{
            text.rewind(backup);
            return None;
        }

        loop {
            ans.text += &text.consume_blank();

            if let Some(r) = Redirect::parse(text, conf){
                    ans.text += &r.text;
                    ans.fds.redirects.push(Box::new(r));
            }else{
                break;
            }
        }
        /*
        if let Some(e) = Eoc::parse(text){
            ans.text += &e.text;
            ans.eoc = Some(e);
        }*/

        Some(ans)
    }
}
