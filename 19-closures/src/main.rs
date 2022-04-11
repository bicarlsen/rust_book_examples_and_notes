use std::thread;
use std::time::Duration;
use std::collections::HashMap;


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout( simulated_user_specified_value, simulated_random_number );
}


fn simulated_expensive_calculation( intensity: u32 ) -> u32 {
    println!( "calculation slowly..." );
    thread::sleep( Duration::from_secs( 2 ) );

    intensity
}

fn generate_workout( intensity: u32, random_number: u32 ) {
    let expensive_closure = |num: u32| -> u32 {
        println!( "calculating slowly..." );
        thread::sleep( Duration::from_secs( 2 ) );
        num
    };


    let mut expensive_result = Cacher::new(
        |num| {
            println!( "calculation slowly..." );
            thread::sleep( Duration::from_secs( 2 ) );

            num
        }
    );


    if intensity < 25 {
        println!( "Today, do {} pushups!", expensive_result.value( intensity ) );
        println!( "Today, do {} situps!", expensive_result.value( intensity ) );
    }
    else {
        if random_number == 3 {
            println!( "Take a break today! Remember to hydarated!" );
        }
        else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value( intensity )
            );
        }
    }
}


struct Cacher<T>
where T:
    Fn( u32 ) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn( u32 ) -> u32,
{
    fn new( calculation: T ) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value( &mut self, arg: u32 ) -> &u32 {
        self.value.entry( arg ).or_insert_with(
            || ( self.calculation )( arg )
        )
    }
}