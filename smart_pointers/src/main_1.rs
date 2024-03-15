use std::cell::RefCell;
use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};
use crate::List3::{Cons3, Nil3};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Using Box<T> Like a Reference
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implicit Deref Coercions with Functions and Methods

    // Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value
    // Because we implemented the Deref trait on MyBox<T>, Rust can turn &MyBox<String> into &String by calling deref
    // Rust calls deref again to turn the &String into &str, which matches the hello function’s definition
    let m = MyBox(String::from("Rust"));
    // If Rust didn’t implement deref coercion, we would have to write the code like this: hello(&(*m)[..]);
    hello(&m);

    // ====== Drop trait

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // c.drop();// error: not allowed to explicitly call drop. Instead use std::mem::drop (already in the prelude)
    drop(c); // Calling std::mem::drop to explicitly drop a value before it goes out of scope
    println!("CustomSmartPointer dropped before the end of main.");

    // ===== Rc<T> reference counting
    // Using Rc<T> allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist.

    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // a is moved into b and b owns a
    // let c = Cons(4, Box::new(a)); // we are trying to use a again which is not allowed as a has been moved

    // When we create b, instead of taking ownership of a, we’ll clone the Rc<List> that a is holding, thereby increasing the number of references from one to two and letting a and b share ownership of the data in that Rc<List>. We’ll also clone a when creating c, increasing the number of references from two to three. Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase, and the data won’t be cleaned up unless there are zero references to it.
    // Rc::clone doesn’t make a deep copy of all the data. The call to Rc::clone only increments the reference count, which doesn’t take much time
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    let b = Cons2(3, Rc::clone(&a)); 
    let c = Cons2(4, Rc::clone(&a));

    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules: multiple mutable borrows to the same place can cause data races and inconsistencies

    // ===== RefCell<T> and the Interior Mutability Pattern
    // Mutating the value inside an immutable value is the interior mutability pattern. Note as per borrowing rules, when you have an immutable value, you can’t borrow it mutably. For example, let x = 5; let y = &mut x; 
    // a value is able to mutate itself in its methods but appear immutable to other code. Code outside the value’s methods would not be able to mutate the value
    // Borrow checking is still prsesent but at runtime instead of compile time. if violated at run time, you get a panic

    // See lib.rs for RefCell example

    // Rc<RefCell<i32>> to create a List that we can mutate
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));

    let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    
}

fn hello(name: &str) {
    println!("Hello, {}!", name);

}

#[derive(Debug)]
enum List3 { 
    Cons3(Rc<RefCell<i32>>, Rc<List3>), // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    Nil3,
}

enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

enum List {
    // Cons(i32, List), // recursive type List has infinite size error
    Cons(i32, Box<List>),
    Nil,
}
// Treating a Type Like a Reference by Implementing the Deref Trait
struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}
