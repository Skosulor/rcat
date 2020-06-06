use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;

fn main() {
    let args = Options::from_args();

    let mut out = match Output::from(args) {
        Ok(o) => o,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    out.format_output();
    out.print();
}

#[derive(StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,

    /// Same as -vET
    #[structopt(short, long = "show-all")]
    A: bool,

    /// Number each line except blank lines
    #[structopt(short = "b", long = "number-nonblank")]
    numbered_nonblank: bool,

    /// same as -vE
    #[structopt(short = "e")]
    nonPrint_and_showEnds: bool,

    /// add $ to the end of each line
    #[structopt(short = "E", long = "show-ends")]
    show_ends: bool,

    ///  Number each line
    #[structopt(short = "n", long = "number")]
    numbered: bool,

    /// remove duplicate blank lines
    #[structopt(short = "s", long = "squeeze-blank")]
    squeeze_blank: bool,

    /// same as -vT
    #[structopt(short = "t")]
    nonPrint_and_showTabs: bool,

    /// remove duplicate blank lines
    #[structopt(short = "T", long = "show-tabs")]
    show_tabs: bool,

    /// (ignored)
    #[structopt(short = "u")]
    ignored: bool,

    /// display non printing characters as ^
    #[structopt(short = "v", long = "show-nonprinting")]
    non_printing: bool,
}

struct Output {
    out: Vec<String>,
    opt: Options,
}

// fn display_help() {}
// fn version() {}

impl Output {
    fn format_output(&mut self) {
        let f_vec: Vec<&dyn Fn(&mut Output)> = vec![
            &Output::number_lines,
            &Output::show_ends,
            &Output::remove_duplicate_blank,
            &Output::show_tabs,
            &Output::show_nonprinting,
        ];
        for f in f_vec.iter() {
            f(self);
        }

        // create vector with functions based on options
        // loop through vector "self.out" and call each function
    }
    // -n & -b
    fn number_lines(&mut self) {
        let padding = format!("{:width$}", " ", width = 4);
        let mut n = 1;
        let mut prefix = String::from("");
        if self.opt.numbered || self.opt.numbered_nonblank {
            for line in self.out.iter_mut() {
                if self.opt.numbered_nonblank && line == "" {
                    prefix = String::from("");
                } else {
                    prefix = format!("{}", n);
                    n += 1;
                }
                let temp = format!("{0:>6}  ", prefix);
                *line = String::from(temp + line);
                //*line = format!("{0: <2}  {1: <5} ", prefix, line);
            }
        }
    }
    fn show_ends(&mut self) {
        // -s
        if self.opt.show_ends || self.opt.A || self.opt.nonPrint_and_showEnds {
            ()
        }
    }
    fn remove_duplicate_blank(&mut self) {
        // -T
        //
        if self.opt.squeeze_blank {
            ()
        }
    }
    fn show_tabs(&mut self) {
        if self.opt.show_tabs || self.opt.nonPrint_and_showTabs || self.opt.A {
            ()
        }
    }
    //
    // display charactars as ^ which are not supported by the terminal
    fn show_nonprinting(&mut self) {
        if self.opt.nonPrint_and_showTabs || self.opt.nonPrint_and_showEnds || self.opt.non_printing
        {
            ()
        }
    }

    // helper function for nonprinting and show tabs
    fn replace_char(&mut self, line: &mut String, old: char, new: char) {}

    fn from(o: Options) -> Result<Output, String> {
        let mut out = Output {
            out: Vec::new(),
            opt: o,
        };

        let input = match &out.opt.path {
            Some(p) => match std::fs::read_to_string(p) {
                Ok(p) => p,
                Err(_) => {
                    return Err(String::from(format!(
                        "Error: No such file: {}",
                        p.display()
                    )))
                }
            },
            None => {
                let mut std_input = String::new();
                match std::io::stdin().read_to_string(&mut std_input) {
                    Ok(_) => std_input,
                    Err(_) => {
                        return Err(String::from(format!("Error: Failed to read from stdin")))
                    }
                }
            }
        };

        for line in input.lines() {
            out.out.push(line.to_string());
        }

        return Ok(out);
    }
    fn print(self) {
        for line in self.out.iter() {
            println!("{}", line);
        }
    }
}