mod front_of_house;

pub use crate::front_of_house::hosting;


mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}

	fn cook_order() {}

	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer( toast: &str ) -> Breakfast {
			Breakfast {
				toast: String::from( toast ),
				seasonal_fruit: String::from( "peaches" ),
			}
		}
	}

	pub enum Appetizer {
		Soup,
		Salad,
	}
}


pub fn eat_at_restaurant() {
	crate::front_of_house::hosting::add_to_waitlist();
	hosting::add_to_waitlist();

	let mut meal = back_of_house::Breakfast::summer( "Rye" );
	meal.toast = String::from( "Wheat" );
	println!( "I'd like {} toast plase", meal.toast );

	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}