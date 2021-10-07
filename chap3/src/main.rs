fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is : {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("result is {} and counter is {}", result, counter);

    /* another kind of loop */
    for elt in (1..4).rev() {
        println!("element : {}", elt);
    }

}


