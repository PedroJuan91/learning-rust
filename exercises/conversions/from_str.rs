// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}


// Steps:
// 1. If the length of the provided string is 0 an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`.
// 5. If while extracting the name and the age something goes wrong an error should be returned
// If everything goes well, then return a Result of a Person object

impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // rdfa:src="https://github.com/ikeohachidi/rustling-solutions/blob/mine/exercises/conversions/from_str.rs"
        let err: Self::Err = String::from("Cannot do this");
        if s.len() == 0 { return Err(err) }
        let split_str = s.split(",").collect::<Vec<&str>>();
        
        if split_str.len() != 2 {
            return Err(err)
        }
        if split_str[0].is_empty() {
            return Err(err)
        }
        if split_str[1].is_empty() {
            return Err(err)
        }
        // let name = String::from_str(split_str[0]).unwrap_or(return Err(err));
        // let age = usize::from_str(split_str[1]).unwrap_or(return Err(err));
        let name = if String::from_str(split_str[0]).is_ok() {
            String::from_str(split_str[0]).unwrap()
        } else {
            return Err(err)
        };
        let age = if usize::from_str(split_str[1]).is_ok() {
            usize::from_str(split_str[1]).unwrap()
        } else {
            return Err(err)
        };

        Ok(Person { name, age })

        // check for empty string
        // if split_str.len() == 
        // construct the person
        // let mut person = Person {
        //     name: split_str[0].to_string(),
        //     age: 0,
        // };
        // check the person has a name
        // if person.name.len() == 0 { return Err(err) }
        // let age = split_str[1].parse::<usize>();

        // match age {  
        //     Ok(i) => {
        //         person.age = i;
        //     },
        //     Err(i) => { return Err(err) }
        // }
        // Ok(person)
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!("John,".parse::<Person>().is_err());
    }

    #[test]
    fn invalid_age() {
        assert!("John,twenty".parse::<Person>().is_err());
    }

    #[test]
    fn missing_comma_and_age() {
        assert!("John".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name() {
        assert!(",1".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_age() {
        assert!(",".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(",one".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma() {
        assert!("John,32,".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert!("John,32,man".parse::<Person>().is_err());
    }
}
