fn copy_and_print(a: i32) -> () {
    println!("{}", a);
}

fn own_and_print(a: String) -> () {
    println!("{}", a);
}

fn borrow_and_print(a: &String) -> () {
    println!("{}", a);
}

fn main() {
    let num :i32 = 36;
    copy_and_print(num);
    println!("{}", num);

    let sentence = String::from("Merhaba Rust");
    borrow_and_print(&sentence);
    println!("{}", sentence);
}


// RUST CORE CONCEPT: OWNERSHIP VS. BORROWING
//
// The Book Analogy:
//
// 1. OWNERSHIP (MOVE):
//    // let my_book = String::from("Mathematics Book");
//    // give_to_friend(my_book);
//    // Analogy: You GIFT the book to a friend. You no longer own it.
//    // If you try to read it later, the compiler throws an error: "Book has been moved."
//
// 2. BORROWING (&): (The efficient way to share)
//    // read_only_function(&my_book);
//    // Analogy: You lend the book to a friend, telling them *where it is* (the address).
//    // They can read it, but they CANNOT take it or change it.
//    // Ownership remains with you (the main function). This is FAST because no copying occurs.
//
// 3. MUTABLE BORROWING (&mut): (The only way to modify)
//    // edit_function(&mut my_book);
//    // Analogy: You let them read the book AND make notes in it, BUT they must put it back.
//    // Crucial Rule: ONLY ONE person can have a mutable borrow at a time. (No data races!)
