extern crate regex;

use regex::Regex;
use std::env;
// use std::io;
use std::io::{self, Write};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref REGEX_PATH_WITH_DOT: Regex = Regex::new(r#"^\."#).unwrap();
    pub static ref REGEX_PATH_WITHOUT_DOT: Regex = Regex::new(r#"/mnt/(\w)/"#).unwrap();
}

fn main() {
    let path = env::current_dir().unwrap();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from pipe");
        input = input.trim().to_string();

        if input == "" {
            break;
        }

        if REGEX_PATH_WITH_DOT.is_match(&input) {
            input = str::replace(&input, "./", "");
            input = format!("{}\\{}", path.display(), input);
            input = format!(
                "{}",
                REGEX_PATH_WITHOUT_DOT.replace_all(
                    &input,
                    |captures: &regex::Captures| captures[1].to_uppercase() + ":\\"
                )
            );
        } else {
            input = format!(
                "{}",
                REGEX_PATH_WITHOUT_DOT.replace_all(
                    &input,
                    |captures: &regex::Captures| captures[1].to_uppercase() + ":\\"
                )
            );
        }

        input = str::replace(&input, "/", "\\");
        writeln!(io::stdout(), "\"{}\"", input);
    }
}
/* Perl Program:
[05:34:05 abhasker@wsl -> code_liveops$ cat /usr/local/bin/winpath
perl -ne 
'
use strict; no warnings; use Cwd;
my $cwd = fastcwd();
chomp($_);
my $path = $_;

if ($path =~ m/^\./){
    $path =~ s/^\.//;
    $path = $cwd . $path;
}
$path =~ s/\/mnt\/(d|c)\//\U$1\:\\/g;
$path =~ tr/\//\\/;
$path = q{"}. $path . q{"};
print $path . "\n";
'
*/
