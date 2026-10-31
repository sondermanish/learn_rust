use std::{fmt::Error, fs};

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn first(&self) -> &T {
        return &self.first;
    }

    fn second(&self) -> &U {
        return &self.second;
    }
}

fn largest<T>(arr: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut p = &arr[0];
    for i in arr {
        if i > p {
            p = i;
        }
    }

    return p;
}


fn sum(numbers: &[i32]) -> i32 {
    let mut s = 0;
    for num in numbers {
        s += num;
    }

    return s;
}

fn add_five(numbers: &mut Vec<i32>) {
    for element in numbers {
        *element += 5;
    }
}

fn first_word(s: &str) -> &str {
    let mut count = 0;
    for ch in s.chars() {
        if ch == ' ' {
            break;
        }

        count += 1;
    }

    return &s[0..count];
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        return Ok(a/b);
    }

    return Err(String::from("Zero division error"));
}

pub fn read_from_file(path: &str) -> String {
    let contents= fs::read_to_string(path);
    match contents {
        Ok(value) => value,
        Err(error) => {
            println!("some error --> {error}");
            return String::from("some error");
        },
    }
}

pub fn read_from_file_propagation(path: &str) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(path)?)
}

pub fn read_from_file_second(path: &str) -> usize {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(value) =>  {
            println!("{value}");
            return value.len();
        }
        Err(err) => {
            println!("{err:?}");
            return 0;
        }
    }
}

pub fn read_file_length(path: &str) -> Result<usize, std::io::Error> {
    let bytes = fs::read_to_string(path);
    match bytes {
        Ok(value) => Ok(value.len()),
        Err(err) => Err(err)
    }

}

pub fn read_file_propagate(path: &str) -> Result<usize, std::io::Error> {
    let bytes = fs::read_to_string(path)?;
    Ok(bytes.len())
}

pub fn owernship_und_reference() {
    let mut s1 = String::from("hello");
    // Change this line:
    let s2 = &mut s1;

    s2.push_str(" world");
    println!("{s2}");
}
