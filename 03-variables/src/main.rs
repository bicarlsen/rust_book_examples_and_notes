fn main() {
    // mutable
    let mut x = 5;
    println!( "The value of x is: {}", x );

    x = 6;
    println!( "The value of x is: {}", x );

    // shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x* 2;
        println!( "The value of x in the inner scope is: {}", x );
    }

    println!( "The values of x is: {}", x );


    // tuples
    let tup: ( u16, f32, i8 ) = ( 500, 6.4, 1 );
    let ( x, y, z ) = tup;
    println!( "The values of y is: {}", y );

    let five_hundred = tup.0;

    // arrays
    let a[i32; 5] = [ 1, 2, 3, 4, 5 ];
    three = a[ 2 ];

    let b = [3; 5];  // [3, 3, 3, 3, 3]

}
