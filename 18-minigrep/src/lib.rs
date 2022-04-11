use std::fs;
use std::env;
use std::error::Error;


pub fn run( config: Config ) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string( config.filename )?;
    let results = if config.case_sensitive {
        search( &config.query, &contents )
    } else {
        search_case_insensitive( &config.query, &contents )
    };

    for line in results {
        println!( "{}", line );
    }

    Ok( () )
}


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new( mut args: env::Args ) -> Result<Config, &'static str> {
        args.next();  // skip arg 0 which is program name
        let query = match args.next() {
            Some( arg ) => arg,
            None => return Err( "Didn't get a query string" ),
        };

        let filename = match args.next() {
            Some( arg ) => arg,
            None => return Err( "Didn't get a file name" )
        };



        // default flag values
        let mut case_sensitive = env::var( "CASE_INSENSITIVE" ).is_err();

        // set flag values
        for arg in args {  // already consumed first two arguments that are required
            if arg == "-i" {
                case_sensitive = false;
            }
            else if arg == "-I" {
                case_sensitive = true;
            }
        }

        Ok( Config {
            query,
            filename,
            case_sensitive,
        } )
    }
}


pub fn search<'a>( query: &str, contents: &'a str ) -> Vec<&'a str> {
    contents
        .lines()
        .filter( | line | line.contains( query ) )
        .collect()
}


pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter( | line | line.to_lowercase().contains( &query ) )
        .collect()
}

// *************
// *** tests ***
// *************

#[cfg( test )]
mod tests {
    use super::*;


    #[test]
    fn case_sensitive() {
        let query = "duct";  // in 'productive'
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!( vec![ "safe, fast, productive." ], search( query, contents ) );
    }


    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![ "Rust:", "Trust me." ],
            search_case_insensitive( query, contents )
        );
    }
}