//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::str::Chars;

#[derive(Clone)]
pub struct Feeder {
    remaining: String,
    from_lineno: u32,
    to_lineno: u32,
    pos_in_line: u32,
}

impl Feeder {
    pub fn new() -> Feeder {
        Feeder {
            remaining: "".to_string(),
            from_lineno: 0,
            to_lineno: 0,
            pos_in_line: 0,
        }
    }

    pub fn lineno(&self) -> (u32, u32) {
        (self.from_lineno, self.to_lineno)
    }

    pub fn pos(&self) -> u32 {
        self.pos_in_line
    }

    pub fn len(&self) -> usize {
        self.remaining.len()
    }

    /*
    pub fn chars(&self) -> Chars {
        self.remaining.chars()
    }
    */

    pub fn chars_after(&self, s: usize) -> Chars {
        self.remaining[s..].chars()
    }

    pub fn nth(&self, p: usize) -> char {
        if let Some(c) = self.remaining.chars().nth(p){
            c
        }else{
            panic!("Parser error")
        }
    }

    pub fn rewind(&mut self, backup: Feeder) {
        self.remaining = backup.remaining.clone();
        self.from_lineno = backup.from_lineno;
        self.to_lineno = backup.to_lineno;
        self.pos_in_line = backup.pos_in_line;
    }

    pub fn consume(&mut self, cutpos: usize) -> String {
        let cut = self.remaining[0..cutpos].to_string();
        self.pos_in_line += cutpos as u32;
        self.remaining = self.remaining[cutpos..].to_string();

        cut
    }

    pub fn add_line(&mut self, line: String) {
        self.to_lineno += 1;

        if self.remaining.len() == 0 {
            self.from_lineno = self.to_lineno;
            self.pos_in_line = 0;
            self.remaining = line;
        }else{
            self.remaining += &line;
        };
    }

    pub fn match_at(&self, pos: usize, chars: &str) -> bool{
        let ch = self.nth(pos);

        if let Some(_) = chars.to_string().find(ch){
            true
        }else{
            false
        }
    }
}

