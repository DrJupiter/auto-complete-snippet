#[cfg(test)]
mod tests {

    // The super prelude gives acces to the rest of the code/the code in the outer scope, since the
    // test module is an inner scope separate from the outer scope, where we define the rest of our
    // code.
    use super::*;

    #[test]
    fn testing_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "This test will panic")]
    fn another_test() {
        panic!("This test will panic and fail");
    }

    #[test]
    fn triangle_60_degree() {
        let triangle = Triangle {
            base: 2,
            side_one: 2,
            side_two: 2,
        };
        assert!(
            triangle.is_equal_sided(),
            "Didn't determine the triangle {:#?} as an equilateral triangle",
            triangle
        );
    }

    #[test]
    fn triangle_not_equal_sided() {
        let triangle: Triangle = Triangle {
            base: 2,
            side_one: 3,
            side_two: 3,
        };

        assert!(!triangle.is_equal_sided());
    }

    // should_panic expected = "Substring" can be used to test whether the panic message contains the
    // expected substring.
    #[test]
    #[should_panic(expected = "The value wasn't between 1 and 100")]
    fn guess_less_than_1() {
        Guess::new(0);
    }

    // Custom error messeges can also be displayed by using the Result Enum.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn read_lines_test() {
        if let Ok(lines) = read_lines("/home/klaus/.config/nvim/UltiSnips/tex.snippets") {
            for line in lines {
                if let Ok(_current_line) = line {
                } else {
                    panic!("Unable to read line");
                }
            }
        }
    }

    #[test]
    fn pattern_vec_creation() {
        let mut file_buffer = read_lines("/home/klaus/.config/nvim/UltiSnips/tex.snippets");
        dbg!(vec_from_pattern(&mut file_buffer));
    }

    #[test]
    fn fysik_re() {
        let test_end_re: Regex = Regex::new(r"\w+$").unwrap();
        let test_end_re_re: Regex = Regex::new(r"[r]").unwrap();
        let snippet_no_regex = "snippet fmindskningafintensitet \"I prop to k\" iA";
        let snippet_regex = "snippet \"f[\\;|æ]+kvivalentdosis\" \"Ækvivalent dosis\" riA";
        // assert!(!dbg!(test_end_re_re.is_match(test_end_re.captures(snippet_no_regex).unwrap().get(0).map_or("", |m| m.as_str()))));

        // Im pretty sure that the second match in redundant, since we are guarenteed
        // to have a capture if we get some match back.
        // So we should just capture.unwrap();
        if let Some(capture) = test_end_re.captures(snippet_no_regex) {
            match capture.get(0) {
                Some(capture) => match test_end_re_re.is_match(capture.as_str()) {
                    false => (),
                    true => panic!("This should not match as an RE"),
                },
                None => println!("No match found at end"),
            };
        }

        if let Some(capture) = test_end_re.captures(snippet_regex) {
            match capture.get(0) {
                Some(capture) => match test_end_re_re.is_match(capture.as_str()) {
                    true => (),
                    false => panic!("Match, but not RE"),
                },
                None => panic!(),
            };
        }
    }

    #[test]
    fn fysik_re_construction () -> Result<Regex,String>{

        let snippet_regex = "snippet \"f[\\;|æ]+kvivalentdosis\" \"Ækvivalent dosis\" riA";
         
        todo!()

    }

}

#[derive(Debug)]
struct Triangle {
    base: u32,
    side_one: u32,
    side_two: u32,
}

impl Triangle {
    fn is_equal_sided(&self) -> bool {
        self.base == self.side_one && self.side_one == self.side_two
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "The value wasn't between 1 and 100, instead {} was received",
                value
            );
        }

        Guess { value }
    }
}

// TODO

// Implement reading the snippet file and create a list or vector with the snippet words

// Path 1:

// Export that list to a python module which then uses the list to generate auto completion
// snippet.analyze("path/to/file","regex","optional_parameter:return_formart")
// return_format could be a json object, it defaults to a raw Vec which I assume is converted
// to a dict or list in python.
// The auto completion is then handled in python(already implemented).

// Path 2:

// Implement the auto completion as a part of the rust code as well, so all you call in python would be
// snippet.analyze("path/to/file","regex","optional_parameter:return_formart")
// then based on the match we pass it to rust code that auto completes and returns a string on enter or something.

// In both cases I have to implement a file searcher which searches line by line after a certain
// criteria.

// fn pattern_finder(file: Buffer, pattern: regex) -> Vec::<str> {
//     todo!()
// }

// fn sorter(complete_options: Vec::<str>, current_text: mut String) -> String {
// todo!()
// }

// Possibly look into vim pop up menus, but I'm pretty sure that could just be implemented in rust
// or python pretty easily.

// Cases:
// Regex
// snippet "f[\;|æ]+kvivalentdosis" "Ækvivalent dosis" riA
// Normal
// snippet fmindskningafintensitet "I prop to k" iA

use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Maybe this should be changed later, since I don't think we will need the buffer more than once.
fn vec_from_pattern<'a>(lines: &mut io::Result<io::Lines<io::BufReader<File>>>) -> Vec<String> {
    let test_re: Regex = Regex::new("^snippet").unwrap();

    let mut vec_re_matches = Vec::new();

    match lines {
        Ok(lines_iter) => {
            for line in lines_iter {
                if let Ok(line) = line {
                    if test_re.is_match(&line) {
                        println!("found snippet: {}", &line);
                        vec_re_matches.push(line);
                    }
                }
            }
        }
        Err(err) => eprintln!("{}", err),
    }

    //    let tmp: &'a str = "tmp";

    //   vec![tmp];

    return vec_re_matches;
}
