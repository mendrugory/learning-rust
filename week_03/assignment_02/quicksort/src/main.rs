use std::clone::Clone; 
use std::cmp::PartialOrd;
use std::fmt::Display;


fn main() {
    let vec = vec![2, 3, 7, 4, 5, 1, 3, 9, 8, 6, 32, 43, 333, 12, 87, 90, 99, 201, 320];
    print_array(quicksort(vec))
}

fn quicksort<T: Clone + PartialOrd>(vector: Vec<T>) -> Vec<T> {
    let mut vec = vector.clone();
    let length = vec.len();
    if length > 1 {
        let pivot = vec.remove(length / 2);
        let mut first_list = vec![];
        let mut second_list = vec![];
        for n in vec {
            if n > pivot {
                second_list.push(n);
            }
            else{
                first_list.push(n);
            }
        }
        let mut sorted_first_list = quicksort(first_list);
        let mut sorted_second_list = quicksort(second_list);
        sorted_first_list.append(&mut vec![pivot]);
        sorted_first_list.append(&mut sorted_second_list);
        sorted_first_list
    }
    else{
        vec
    }
}

fn print_array<T: Display>(vec: Vec<T>) {
    for a in vec{
        print!("{} ", a);
    }
    println!()
}
