fn process_vector_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for item in vec {
        result.push(f(item));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector_loop(numbers.clone(), |x| {  
        x * 2
    });

    let replaced = process_vector_loop(numbers, |x| {         
        if x > 2 {  // ✅ > 2 not > 3
            0
        } else {
            x
        }
    });

    println!("Loop");
    println!("Doubled: {:?}", doubled);   
    println!("Replaced: {:?}", replaced); 
}