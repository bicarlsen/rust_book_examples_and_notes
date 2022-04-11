use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Nil, Cons};
use crate::ListRc::{NilRc, ConsRc};
use crate::ListRcRefCell::{NilRcRc, ConsRcRc};


fn main() {

    // box
    let b = Box::new( 5 );
    println!("b = {}", b );

    let list = Cons(
        1, Box::new( Cons(
            2, Box::new( Cons(
                3, Box::new ( Nil )
            ) ) 
        ) )
    );


    // rc
    let a = Rc::new( ConsRc( 
        5, Rc::new( ConsRc(
            10, Rc::new( NilRc )
        ) )
    ) );
    println!( "count after creating a = {}", Rc::strong_count( &a ) );

    let b = ConsRc( 3, Rc::clone( &a ) );
    println!( "count after creating b = {}", Rc::strong_count( &a ) );

    {
        let c = ConsRc( 4, Rc::clone( &a ) );
        println!( "count after creating c = {}", Rc::strong_count( &a ) );
    }

    println!( "count after dropping c = {}", Rc::strong_count( &a ) );


    // rc refcell
    let value = Rc::new( RefCell::new( 5 ) );
    let a = Rc::new( ConsRcRc(
        Rc::clone( &value ), Rc::new( NilRcRc ) 
    ) );

    let b = ConsRcRc(
        Rc::new( RefCell::new( 3 ) ),
        Rc::clone( &a )
    );

    let c = ConsRcRc(
        Rc::new( RefCell::new( 4 ) ),
        Rc::clone( &a )
    );

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a );
    println!("b after = {:?}", b );
    println!("c after = {:?}", c );


    // deref
    let x = 5;
    let y = &x;
    assert_eq!( 5, *y );

    let y = Box::new( x );
    assert_eq!( 5, *y );

    let y = MyBox::new( x );
    assert_eq!( 5, *y );

    let m = MyBox::new( String::from( "Rust" ) );
    hello( &m );


    // drop
    let c = CustomSmartPointer {
        data: String::from( "my stuff" ),
    };

    let d = CustomSmartPointer {
        data: String::from( "other stuff" ),
    };

    println!( "CustomSmartPointers created." );
    drop( c );
    println!( "CustomSmartPointer dropped before end of scope." );
}


fn hello( name: &str ) {
    println!( "Hello, {}!", name );
}


enum List {
    Cons( i32, Box<List> ),
    Nil,
}


enum ListRc {
    ConsRc( i32, Rc<ListRc> ),
    NilRc,
}


#[derive(Debug)]
enum ListRcRefCell {
    ConsRcRc( Rc<RefCell<i32>>, Rc<ListRcRefCell> ),
    NilRcRc,
}


struct MyBox<T>( T );

impl<T> MyBox<T> {
    fn new( x: T ) -> MyBox<T> {
        MyBox( x )
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop( &mut self ) {
        println!( "Dropping CustomSmartPointer with data `{}`!", self.data );
    }
}

