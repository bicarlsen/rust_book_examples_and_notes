fn main() {
    let home = IpAddr::V4( 127, 0, 0, 1 );
    let loopback = IpAddr::V6( String::from( "::1" ) );

    let m = Message::Write( String::from( "hello" ) );
    m.call();

    // option enum
    let some_number = Some( 5 );
    let some_string = Some( "a string" );
    let absent_number: Option<i32> = None;

    let five = Some( 5 );
    let six = plus_one( five );
    let non = plus_one( None );


    let config_max = Some( 3u8 );
    if let Some( max ) = config_max {
        println!( "The maximum is configured to be {}", max );
    }
}


enum IpAddr {
    V4( u8, u8, u8, u8 ),
    V6( String ),
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },  // named fields
    Write( String ),
    ChangeColor( i32, i32, i32 ),
}

impl Message {
    fn call( &self ) {
        println!( "Calling..." );
    }
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    CanadianDollar,
}

fn value_in_cents( coin: Coin ) -> u8 {
    match coin {
        Coin::Penny => {
            println!( "Lucky penny!" );
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter( state ) => {
            println!( "State quarter from {:?}!", state );
            25
        },
        _ => {
            println!( "I don't recognize that coin, it's probably worthless." );
            0
        }
    }
}


fn plus_one( x: Option<i32> ) -> Option<i32> {
    match x {
        None => None,
        Some( i ) => Some( i + 1 ),
    }
}