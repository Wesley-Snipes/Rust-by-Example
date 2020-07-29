fn main() {
    let mut count = 0u32;

    println!("Let's count unitl infinity!");


    // Initite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of the iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit the loop
            break;
        }
    }
}
