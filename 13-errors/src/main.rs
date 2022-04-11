use std::error::Error;
use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open( "hello.txt" );
    let f = match f {
        Ok( file ) => file,
        Err( error ) => panic!( "Problem opening the fiel: {:?}", error ),
    };

    let f = File::open( "again.txt" );
    let f = match f {
        Ok( file ) => file,
        Err( error ) => match error.kind() {
            ErrorKind::NotFound => match File::create( "again.txt" ) {
                Ok( fc ) => fc,
                Err( e ) => panic!( "Problem creating the file: {:?}", e ),
            },

            other_error => {
                panic!( "Problem opening the file: {:?}", other_error )
            }
        }
    };

    
    let f = File::open( "hola.txt" ).unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create( "hola.txt" ).unwrap_or_else( |error| {
                panic!( "Problem creating the file: {:?}", error );
            } )
        }
        else {
            panic!( "Problem opening the file: {:?}", error );
        }
    } );


    let f = File::open( "bonjour.txt" ).unwrap();
    let f = File::open( "bomdia.txt" ).expect( "Failed to open bomdia.txt" );

    // return from main
    let f = File::open( "hello.txt" )?;
    Ok( () )
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open( "hello.txt" );
    let mut f = match f {
        Ok( file ) => file,
        Err( e ) => return Err( e ),
    };

    let mut s = String::new();
    match f.read_to_string( &mut s ) {
        Ok( _ ) => Ok( s ),
        Err( e ) => Err( e ),
    }
}


fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open( "hello.txt" )?;
    let mut s = String::new();
    f.read_to_string( &mut s )?;
    Ok( s )
}


fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open( "hello.txt" )?.read_to_string( &mut s )?;
    Ok( s )
}


fn last_char_of_first_line( text: &str ) -> Option<char> {
    text.lines().next()?.chars().last()
}