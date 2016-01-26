extern crate term;

use matcher::Match;
use std::io;
use std::io::Write;
use std::process;
use term::{Terminal, StdoutTerminal, StderrTerminal};

// ---------------------------------------------------------------------------------------------------------------------
// Console
// ---------------------------------------------------------------------------------------------------------------------

pub enum ConsoleTextKind {
    Filename ,
    Text     ,
    MatchText,
    Other    ,
    Error    ,
}

pub struct Console {
    pub is_color: bool,
    term_stdout: Box<StdoutTerminal>,
    term_stderr: Box<StderrTerminal>,
}

impl Console {
    pub fn new() -> Self {
        Console {
            term_stdout: term::stdout().unwrap_or_else( || { process::exit( 1 ); } ),
            term_stderr: term::stderr().unwrap_or_else( || { process::exit( 1 ); } ),
            is_color   : true,
        }
    }

    pub fn carriage_return( &mut self ) {
        let _ = self.term_stdout.carriage_return();
    }

    pub fn cursor_up( &mut self ) {
        let _ = self.term_stdout.cursor_up();
    }

    pub fn delete_line( &mut self ) {
        let _ = self.term_stdout.delete_line();
    }

    pub fn write_with_clear( &mut self, kind: ConsoleTextKind, val: &str ) {
        self.carriage_return();
        self.delete_line();
        self.write( kind, val );
    }

    pub fn write( &mut self, kind: ConsoleTextKind, val: &str ) {
        if self.is_color {
            let color = match kind {
                ConsoleTextKind::Filename  => term::color::BRIGHT_GREEN,
                ConsoleTextKind::Text      => term::color::WHITE,
                ConsoleTextKind::MatchText => term::color::BRIGHT_YELLOW,
                ConsoleTextKind::Other     => term::color::BRIGHT_CYAN,
                ConsoleTextKind::Error     => term::color::BRIGHT_RED,
            };
            self.term_stdout.fg( color ).unwrap_or_else( |_| { process::exit( 1 ); } );
            self.term_stderr.fg( color ).unwrap_or_else( |_| { process::exit( 1 ); } );
        }

        match kind {
            ConsoleTextKind::Error => write!( self.term_stderr, "{}", val ).unwrap_or_else( |_| { process::exit( 1 ); } ),
            _                      => write!( self.term_stdout, "{}", val ).unwrap_or_else( |_| { process::exit( 1 ); } ),
        }

        if self.is_color {
            self.term_stdout.reset().unwrap_or_else( |_| { process::exit( 1 ); } );
            self.term_stderr.reset().unwrap_or_else( |_| { process::exit( 1 ); } );
        }
        let _ = io::stdout().flush();
        let _ = io::stderr().flush();
    }

    pub fn write_match_line( &mut self, src: &[u8], m: &Match ) {
        let mut beg = m.beg;
        let mut end = m.end;
        while beg > 0 {
            if src[beg] == 0x0d || src[beg] == 0x0a { beg += 1; break; }
            beg -= 1;
        }
        while src.len() > end {
            if src[end] == 0x0d || src[end] == 0x0a { end -= 1; break; }
            end += 1;
        }
        if src.len() <= end { end = src.len() } else { end += 1 };

        if beg < m.beg {
            self.write( ConsoleTextKind::Text, &String::from_utf8_lossy( &src[beg..m.beg] ) );
        }
        self.write( ConsoleTextKind::MatchText, &String::from_utf8_lossy( &src[m.beg..m.end] ) );
        if m.end < end {
            self.write( ConsoleTextKind::Text, &String::from_utf8_lossy( &src[m.end..end] ) );
        }
        self.write( ConsoleTextKind::Other, "\n" );
    }
}
