use std::fmt;

/*
   Execution starts here
*/
fn main() {
    // run the tests
    test_pair_up();
    test_factorial_numbers();
    test_make_element_generator();
    test_run_length_encoding();
    test_forms_chain();
}

fn make_element_generator<T>(list: Vec<T>) -> impl FnMut() -> Option<T> {

    || None
}

fn pair_up<T: Copy>(list: &Vec<T>) -> Vec<Vec<T>> {
    vec![]
}


fn factorial_numbers(n: i32) -> Vec<i32> {
    vec![]
}

fn run_length_encoding<T: Ord + Copy>(list: &Vec<T>) -> Vec<(i32,T)> {
    vec![]
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

