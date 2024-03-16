pub trait Draw {
    fn draw(&self);
}

// Using generics here would have limited us since we can only create a Struct of a specific type T that implements Draw eg Screen<Button>. In this case, all components will be of type Button which is not what we want here.
// That's why we use the trait object which allows for different types as long as they implement Draw.
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // Box<dyn Draw> is a trait object. it’s a stand-in for any type inside a Box that implements the Draw trait. Can be used in place of a generic or concrete type
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        
    }
}

// When we use trait objects, Rust must use dynamic dispatch unlike using trait bounds on generics which uses static dispatch. The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations. 