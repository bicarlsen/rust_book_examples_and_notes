use std::collections::HashMap;


fn main() {
    // vectors
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![ 1, 2, 3 ];

    let mut v3 = Vec::new(); 
    v3.push( 4 );
    v3.push( 5 );

    let four: &i32 = &v3[ 0 ];
    let five = &v3[ 1 ];
    println!( "The first element is {}", four );
    match v3.get( 1 ) {
        Some( second ) => println!( "The second element is {}", second ),
        None => println!( "There is no second element." ),
    }

    for i in &v3 {
        println!( "{}", i );
    }

    let mut v4 = vec![ 100, 32, 57 ];
    for i in &mut v4 {
        *i += 50;
    }


    // strings
    let data = "initial comments";
    let s = data.to_string();

    let mut s = String::new();
    s.push_str( "bar" );
    s.push( 's' );

    let s1 = String::from( "Hello, " );
    let s2 = String::from( "world!" );
    let s3 = s1 + &s2;
    let s4 = format!( "{} You mean the {}", s3, s2 );


    // hashmaps
    let mut scores = HashMap::new();
    scores.insert( String::from( "Blue" ), 10 );
    scores.insert( String::from( "Yellow" ), 50 );

    let teams = vec![ String::from( "Blue" ), String::from( "Yellow" ) ];
    let initial_scores = vec![ 10, 50 ];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(
        initial_scores.into_iter()
    ).collect();

    let team_name = String::from( "Blue" );
    let score = scores.get( &team_name );

    for ( key, value ) in &scores {
        println!( "{}: {}", key, value );
    } 

    scores.entry( String::from( "Blue" ) ).or_insert( 25 );
    scores.entry( String::from( "Green" ) ).or_insert( 20 );
}
