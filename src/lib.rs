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

// Path 2:

// Impliment the auto completion as a part of the rust code as well, so all you call in python would be 
// snippet.analyze("path/to/file","regex","optional_parameter:return_formart")
// then based on the match we pass it to rust code that auto completes and returns a string on enter or something.