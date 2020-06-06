use std::fs::File;
use std::io::{self, prelude::*};
use structopt::StructOpt;

fn main() -> Result<(), std::io::Error> {
    let args = Options::from_args();

    match Output::from(args) {
        Ok(_) => (),
        Err(err) => {
            println!("Error: {}", err);
        }
    };
    Ok(())
}

#[derive(StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,

    /// Same as -vET
    #[structopt(short = "A", long = "show-all")]
    show_all: bool,

    /// Number each line except blank lines
    #[structopt(short = "b", long = "number-nonblank")]
    numbered_nonblank: bool,

    /// same as -vE
    #[structopt(short = "e")]
    non_print_and_show_ends: bool,

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
    non_print_and_show_tabs: bool,

    /// remove duplicate blank lines
    #[structopt(short = "T", long = "show-tabs")]
    show_tabs: bool,

    /// (ignored)
    #[structopt(short = "u")]
    _ignored: bool,

    /// display non printing characters as ^
    #[structopt(short = "v", long = "show-nonprinting")]
    non_printing: bool,
}

struct Output {
    out: Vec<String>,
    opt: Options,
}

enum Input {
    FromFile(io::BufReader<File>),
    FromStdin,
}

enum ReadResult {
    Line(String),
    EOF,
}

impl Output {
    fn format_output(&mut self) {
        // Yes each function could have been called on self instead
        // The reason a vector of function pointers is used are purley educational for myself
        let f_vec: Vec<&dyn Fn(&mut Output)> = vec![
            &Output::remove_duplicate_blank,
            &Output::number_lines,
            &Output::show_ends,
            &Output::show_tabs,
            &Output::show_nonprinting,
        ];
        for f in f_vec.iter() {
            f(self);
        }
    }
    // -n & -b
    fn number_lines(&mut self) {
        if self.opt.numbered || self.opt.numbered_nonblank {
            let mut n = 1;
            let mut _prefix = String::from("");

            for line in self.out.iter_mut() {
                if self.opt.numbered_nonblank && line == "" {
                    _prefix = String::from("");
                } else {
                    _prefix = format!("{}", n);
                    n += 1;
                }
                let temp = format!("{0:>6}  ", _prefix);
                *line = String::from(temp + line);
            }
        }
    }
    fn show_ends(&mut self) {
        // -s
        if self.opt.show_ends || self.opt.show_all || self.opt.non_print_and_show_ends {
            for line in self.out.iter_mut() {
                *line = String::from(line.clone() + "$");
            }
        }
    }
    fn remove_duplicate_blank(&mut self) {
        // -s
        if self.opt.squeeze_blank {
            let mut prev_line = String::from("negative line");
            let mut to_remove: Vec<usize> = Vec::new();

            // Get a vector with the indexes of multiple blanks
            for (n, line) in self.out.iter_mut().enumerate() {
                if prev_line == "" && line.replace(" ", "") == "" {
                    to_remove.push(n);
                    prev_line = String::from("");
                } else {
                    prev_line = line.clone();
                }
            }

            for (n, rem) in to_remove.iter().enumerate() {
                self.out.remove(rem - n);
            }
        }
    }
    fn show_tabs(&mut self) {
        if self.opt.show_tabs || self.opt.non_print_and_show_tabs || self.opt.show_all {
            for line in self.out.iter_mut() {
                *line = line.replace("\t", "^I").clone();
            }
        }
    }

    //
    // display charactars as ^ which are not supported by the terminal
    fn show_nonprinting(&mut self) {
        if self.opt.non_print_and_show_tabs
            || self.opt.non_print_and_show_ends
            || self.opt.non_printing
            || self.opt.show_all
        {
            for line in self.out.iter_mut() {
                //line.retain(|c| c.is_ascii());
                let mut temp = String::new();
                for c in &mut line.chars() {
                    if c.is_ascii() {
                        temp.push(c);
                    } else {
                        temp.push_str("^?");
                        // TODO call helper function to represent the character correctly
                    }
                }
                *line = temp;
            }
        }
    }

    fn from(o: Options) -> Result<(), std::io::Error> {
        let mut out = Output {
            out: Vec::new(),
            opt: o,
        };

        let mut f = match Input::from(&out.opt.path) {
            Ok(f) => f,
            Err(err) => return Err(err),
        };

        loop {
            match f.readline() {
                Ok(res) => match res {
                    ReadResult::Line(l) => out.out.push(l),
                    ReadResult::EOF => break,
                },
                Err(err) => return Err(err),
            }
        }

        out.format_output();
        out.print();
        return Ok(());
    }
    fn print(&self) {
        for line in self.out.iter() {
            println!("{}", line);
        }
    }
}

impl Input {
    fn from(p: &Option<std::path::PathBuf>) -> Result<Input, std::io::Error> {
        //let file = File::open("/dev/urandom");
        match p {
            Some(p) => {
                return Ok(Input::FromFile(io::BufReader::new(match File::open(p) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                })))
            }

            None => return Ok(Input::FromStdin),
        }
    }

    fn readline(&mut self) -> Result<ReadResult, std::io::Error> {
        let mut input = String::new();
        let res = match self {
            Input::FromStdin => {
                let res = std::io::stdin().read_line(&mut input);
                input = String::from(input.trim());
                res
            }

            Input::FromFile(f) => {
                let res = f.read_line(&mut input);
                input = String::from(input.trim());
                res
            }
        };

        match res {
            Ok(r) => {
                if r == 0 {
                    return Ok(ReadResult::EOF);
                }
            }
            Err(err) => return Err(err),
        }
        return Ok(ReadResult::Line(input));
    }
}
