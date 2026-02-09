use std::fmt;

/*
   Execution starts here
*/
fn main() {
    // run the tests
    println!("\nTesting pair_up:");
    test_pair_up();
    println!("\nTesting factorial_numbers:");
    test_factorial_numbers();
    println!("\nTesting make_element_generator:");
    test_make_element_generator();
    println!("\nTesting run_length_encoding:");
    test_run_length_encoding();
    println!("\nTesting forms_chain:");
    test_forms_chain();
}

fn make_element_generator<T>(list: Vec<T>) -> impl FnMut() -> Option<T> {

    || None
}

fn pair_up<T: Copy>(list: &Vec<T>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![];
    let len = list.len();
    for i in 0..len/2 {
        let pair = vec![list[2*i], list[2*i + 1]];
        result.push(pair);
    }
    if len % 2 == 1 {
        result.push(vec![list[len - 1]]);
    }
    result
}


fn factorial_numbers(n: i32) -> Vec<i32> {
    let mut factorials = vec![];
    let mut current_factorial = 1;
    for i in 1..=n {
        current_factorial *= i;
        factorials.push(current_factorial);
    }
    factorials
}

fn run_length_encoding<T: Ord + Copy>(list: &Vec<T>) -> Vec<(i32,T)> {
    let mut result: Vec<(i32, T)> = vec![];
    if list.is_empty() {
        return result;
    }
    let mut count = 1;
    let mut current_element = list[0];
    for i in 1..list.len() {
        if list[i] == current_element {
            count += 1;
        } else {
            result.push((count, current_element));
            current_element = list[i];
            count = 1;
        }
    }
    result.push((count, current_element));
    result
}

fn forms_chain<T: Ord>(list: &[Vec<T>]) -> bool {
  false
}

/*
  Function to test run_length_encoding
*/
fn test_run_length_encoding() {
    // Set up the test cases
    let test_cases = [
      vec![2, 3, 3],
      vec![2, 1, 2, 2, 2, 3],
      vec![],
      vec![77]
    ];
    // Execute the test cases
    for test_case in test_cases {
       println!("Run length encoding of {:?} is {:?}", test_case, run_length_encoding(&test_case));
    }
}

/*
  Function to test forms_chain
*/
fn test_forms_chain() {
   // Set up the test cases
   let tests = [ 
     vec![],
     vec![vec![1, 2], vec![2, 1]],
     vec![vec![1, 2], vec![5, 3]],
     vec![vec![1, 5, 0], vec![0, 2, 3], vec![3,2], vec![2, 7]]
   ];
    // Execute the test cases
   for test_case in tests {
     println!("List is {:?} forms chain {} ", test_case, forms_chain(&test_case));
   }
}

/*
  Function to test make_element_generator
*/
fn test_make_element_generator() {
   // Set up the test cases
   let tests = [ 
     vec![],
     vec![1, 2], 
     vec![1, 2, 3]
   ];
   // Execute the test cases
   for test_case in tests {
     let mut f = make_element_generator(test_case.clone());
     println!("Testing for vector {:?}", test_case);
     println!("  First element: {}", f().unwrap_or(0));
     println!("  Second element: {}", f().unwrap_or(0)); 
     println!("  Third number: {}", f().unwrap_or(0)); 
   }
}

/*
  Function to test pair_up
*/
fn test_pair_up() {
   // Set up the test cases
   let tests = [ 
     vec![],
     vec![1],
     vec![1, 2],
     vec![1, 2, 3, 4, 5]
   ];
   // Execute the test cases
   for test_case in tests {
     println!("Pair up of {:?} is {:?}", test_case, pair_up(&test_case));
   }
}

/*
  Function to test factorial_numbers
*/
fn test_factorial_numbers() {
    println!("Factorials up to 1 {:?}", factorial_numbers(1));
    println!("Factorials up to 4 {:?}", factorial_numbers(4));
    println!("Factorials up to 10 {:?}", factorial_numbers(10));
}

