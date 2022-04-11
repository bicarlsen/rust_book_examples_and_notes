

fn main() {
    let msg = Message::ChangeColor( Color::Rgb( 0, 160, 255 ) );

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7
        } => println!( "Found an id in range: {}", id_variable ),

        Message::Hello { id } => println!( "Id was out of range: {}", id ),

        Message::Quit => {
            println!( "The Quit variant has no data to destructure." )
        },

        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        },

        Message::Write( text ) => println!( "Text message: {}", text ),
        
        Message::ChangeColor( Color::Rgb( r, g, b ) ) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),

        Message::ChangeColor( Color::Hsv( h, s, v ) ) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }
}


enum Message {
    Hello { id: i32 },
    Quit,
    Move { x: i32, y: i32 },
    Write( String ),
    ChangeColor( Color ),
}

enum Color {
    Rgb( i32, i32, i32 ),
    Hsv( i32, i32, i32 ),
}