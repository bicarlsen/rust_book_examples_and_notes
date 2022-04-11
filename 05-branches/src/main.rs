fn main() {
    let number = 3;

    if number < 5 {
        println!( "condtion was true" );
    }
    else {
        println!( "condition was false" );
    }


    let number = 6;
    if number% 4 == 0 {
        println!( "number divisible by 4" );
    }
    else if number% 4 == 1 {
        println!( "number divisible by 3" );
    }
    else if number% 4 == 2 {
        println!( "number divisible by 2" );
    }
    else {
        println!( "number is not divisible by 2, 3, of 4" );
    }


    let condition = true;
    let number = if condition{ 5 } else { 6 };
    println!( "The value of number is: {}", number );
}
