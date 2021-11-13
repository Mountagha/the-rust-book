
fn main() {
    use std::mem;

    let color = String::from("green");

    let print = || println!("color : {}", color);

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;
    // print(); // won't work. color has been moved.

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("Count: {}", count);
    };

    inc();

    let _reborrow = &mut count;

    //inc(); // won't work &count borrowed earlier
    
    let movable = Box::new(3);

    let consume = || {
        println!("movable : {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume() won't work already consumed.

    let my_vec = vec![1, 2, 3];
    let contains = move |elt| my_vec.contains(elt);
    
    println!("{}", contains(&2));

    // println!("{}", my_vec.len()); // won't work cause ownership changed
    // removing move would make it work again.

}