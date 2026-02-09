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
    // change the list into an iterator and return a closure that calls next on the iterator
    let mut iterator = list.into_iter();
    move || iterator.next()
}

fn pair_up<T: Copy>(list: &Vec<T>) -> Vec<Vec<T>> {
    // create a new vector to hold the pairs and get list length
    let mut result: Vec<Vec<T>> = vec![];
    let len = list.len();

    // iterate through the list in steps of 2 and create pairs
    for i in 0..len/2 {
        let pair = vec![list[2*i], list[2*i + 1]];
        result.push(pair);
    }
    // handle odd length list
    if len % 2 == 1 {
        result.push(vec![list[len - 1]]);
    }
    result
}


fn factorial_numbers(n: i32) -> Vec<i32> {
    // create a vector to hold the factorials and a variable to keep track of the current factorial value
    let mut factorials = vec![];
    let mut current_factorial = 1;

    // calculate factorials from 1 to n and store them in the vector
    for i in 1..=n {
        current_factorial *= i;
        factorials.push(current_factorial);
    }
    factorials
}

fn run_length_encoding<T: Ord + Copy>(list: &Vec<T>) -> Vec<(i32,T)> {
    // create a vector to hold the result and handle the case of an empty list
    let mut result: Vec<(i32, T)> = vec![];
    if list.is_empty() {
        return result;
    }

    // iterate through the list and count consecutive occurrences of each element
    let mut count = 1;
    let mut current_element = list[0];
    // skip first element because it is already counted
    for &item in list.iter().skip(1) {
        if item == current_element {
            count += 1;
        } else {
            result.push((count, current_element));
            current_element = item;
            count = 1;
        }
    }
    // push the last counted element to the result and return
    result.push((count, current_element));
    result
}

fn forms_chain<T: Ord>(list: &[Vec<T>]) -> bool {
    // handle the case of an empty list or a list with only one sublist
    if list.len() < 2 {
        return true;
    }
    // iterate through the list and check if the last element of the current sublist matches the first element of the next sublist
    for i in 0..list.len() - 1 {
        let last_of_current = list[i].last();
        let first_of_next = list[i + 1].first();
        if last_of_current != first_of_next {
            return false;
        }
    }
    // if all pairs of sublists form a chain, return true
    true
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

