fn main() {
    // functions
    let number_list = vec![ 34, 50, 25,100, 65 ];
    let result = largest( &number_list );
    println!( "The largest number is {}", result );

    let char_list = vec![ 'y', 'm', 'a', 'q' ];
    let result = largest( &char_list );
    println!( "The largest char is {}", result );


    // structs
    let intenger = Point { x: 5, y: 10, z: 1.25 };
    let float = Point { x: 1.0, y: 4.0, z: 3.5 };

    // methods
    let p = Point { x: 5, y: 10, z: 3.5 };
    println!( "p.x = {}", p.x() );

    let p2 = Point { x: 1.0, y: 3.5, z: 10 };
    let p3 = p.mixup( p2 );
    println!( "p3.x = {}, p3.y = {}, p3.z = {}", p3.x, p3.y, p3.z );
}


fn largest<T: PartialOrd + Copy>( list: &[T] ) -> T {
    let mut largest = list[ 0 ];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_ref<T: PartialOrd >( list: &[T] ) -> &T {
    let mut largest = &list[ 0 ];
    for item in list {
        if *item > *largest {
            largest = &item;
        }
    }

    largest
}


struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

impl<T, U> Point<T, U> {
    fn x( &self ) -> &T {
        &self.x
    }

    fn z( &self ) -> &U {
        &self.z
    }

    fn mixup<T2, U2>( self, other: Point<T2, U2> ) -> Point<T, U2> {
        Point {
            x: self.x,
            y: self.y,
            z: other.z
        }
    }
}

impl<T> Point<f32, T> {
    fn distance_from_origin( &self ) -> f32 {
        ( self.x.powi( 2 ) + self.y.powi( 2 ) ).sqrt()
    }
}