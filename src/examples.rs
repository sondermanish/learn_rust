// Error handling in multiple ways.

fn error_handling() {
    // result
    let result: Result<u32, String> = divide(5, 25);
    let result2: Result<u32, String> = divide(0, 25);
    let result3: Result<u32, String> = divide(0, 25);
    
    // different ways of unpacking
    // unwrap without error message
    let quotient = result.unwrap();
    let quotient2 = result2.expect("Zero divsion error");

    match result3 {
        Ok(value) => println!("{value}"),
        Err(error) => println!("{error}"),
    }

    println!("printint all results {quotient}, {quotient2}");
    let result3 = no_error_despite_failure().unwrap();
    println!("result is {result3}");
}

/// Question mark operator
fn no_error_despite_failure() -> Result<u32, String> {
    // Errors could also be paased to callers directly
    // If this divide results errors it would directly go to the caller
    Ok(divide(0, 100)?)
}

fn divide(divisor: u32, div: u32) -> Result<u32, String> {
    if divisor == 0 {
        return Err(String::from("Zero division error"));
    }

    return Ok(div/divisor);
}

fn closures() -> i32 {
    let x = 5;
    let add_x = |x| x*10;
    let y = add_x(x);
    y
}

fn revision() {
    let mut some:String = String::from("hello");
    let x = &mut some;
    println!("{x}");

    some.push_str(" world");
    println!("{some}");

    let a = 16;
    let b = &a;
    println!("{b}, {a}");

}

/// closures and iterators
fn iter() {
    // Three type of clousers
    // Fn --> just reads the variable 
    let x: i32 = 25;
    // sample closures
    let closure_fn = |var| var + x;
    println!("closure : {}", closure_fn(22)); 
    // FnMut -->  reads the variable and then modifies it as well. captured environment is modified
    // FnOnce --> could be used only once as it takes ownership of the variable
    // sample closures

    let mut y = 20;
    let mut closure_fn_mut = || y += 10;
    closure_fn_mut();
    println!("Post closure_fn_mut: {y}");

    let ey: String = String::from("some string");
    let clousre_fn_once = || drop(ey);
    clousre_fn_once(); 

    let numbers = vec![1, 2, 3, 4, 5, 6];

    // iter() is a borrow of reference
    let doubled_nums: Vec<i32> = numbers.iter().map(|elements| elements*2).collect();
    let even_nums: Vec<i32> = numbers.iter().filter(|num| *num%2 == 0).map(|num| *num).collect();
    let doubled_evens: Vec<i32> = numbers.iter().filter(|num| *num%2 == 0).map(|num| num*2).collect();

    println!("doubled_nums {:?}", doubled_nums);
    println!("even_nums {:?}", even_nums);
    println!("doubled_evens {:?}", doubled_evens);

    let mut mut_nums = vec![1,3,4,5,6,45];
    for num in mut_nums.iter_mut() {
        *num += 10;
    }
    let abc: Vec<i32> = numbers.iter().filter(|num| **num % 2 == 0).map(|num| *num).collect();
    let mut some_nums = vec![1, 2, 3];
    let meow: Vec<i32> = some_nums.iter_mut().map(|elements| *elements*2).map(|num| num).collect();
}


struct Counter {
    x: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(& mut self) -> Option<u32> {
        self.x += 1;
        return Some(self.x);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from(
                "Not enough arguments. Expected a query and a file path."
            ));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

