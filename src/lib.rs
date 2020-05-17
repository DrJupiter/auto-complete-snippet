// Cases:
// Regex
// snippet "f[\;|æ]+kvivalentdosis" "Ækvivalent dosis" riA
// Normal
// snippet fmindskningafintensitet "I prop to k" iA

//TODO add error handling for regex without optional completion

use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let file_metadata = file.metadata()?;
    if file_metadata.is_file() {
        Ok(io::BufReader::new(file).lines())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Not a file"))
    }
}

lazy_static! {
    static ref SNIPPET_FINDER_RE: Regex = Regex::new(r"^snippet\s").unwrap();
    static ref TEST_RE: Regex = Regex::new(r"r\w*?$").unwrap();
    static ref RE_CAPTURE: Regex = Regex::new("\"(.+?)\"").unwrap();
    static ref NORMAL_CAPTURE: Regex = Regex::new(r"^\w+ (\S+)").unwrap();
    static ref RE_DELIMETER_CAPTURE: Regex = Regex::new(r"\|(\S+)\|").unwrap();
}

// Maybe this should be changed later, since I don't think we will need the buffer more than once.
/// Creates a vector from the patterns specified in the lazy_static! clause.
fn vec_from_pattern<'a>(lines: &mut io::Lines<io::BufReader<File>>) -> Vec<String> {
    let mut vec_re_matches = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            if SNIPPET_FINDER_RE.is_match(&line) {
                if TEST_RE.is_match(&line) {
                    if let Some(cap) = RE_DELIMETER_CAPTURE.captures(&line) {
                        vec_re_matches.push(cap.get(cap.len() - 1).unwrap().as_str().into());
                    }
                //                    vec_re_matches.push(RE_DELIMETER_CAPTURE.captures(&line).unwrap().get(1).unwrap().as_str().into());
                } else {
//                    println!("Line: {}\n Norm cap:{:?}",&line, NORMAL_CAPTURE.captures(&line));
                    vec_re_matches.push(
                        NORMAL_CAPTURE
                            .captures(&line)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .into(),
                    );
                }
            }
        }
    }
    return vec_re_matches;
}

#[pyfunction]
///Returns a list with all matched items in the file.
fn init_py<'a>(path: &'a str) -> PyResult<Vec<String>> {
    let file_buffer = read_lines(path);

    if let Ok(mut succesful_file_read) = file_buffer {
        Ok(vec_from_pattern(&mut succesful_file_read))
    } else {
        Ok(vec![String::from("Failed to read file")])
    }
}

//#[pyfunction]
/// Should panic
//fn another_test() {
//    println!("We can print");
//    panic!("This test will panic and fail");
//}

#[pymodule]
/// A Python module implemented in Rust.
fn rust_sp_snippet_finder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(init_py))?;
    //   m.add_wrapped(wrap_pyfunction!(another_test))?;
    Ok(())
}
