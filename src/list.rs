#![feature(box_syntax)]
use List::{Cons,Nil};
use std::fmt::{Display,Result,Formatter};
#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    fn new() -> Box<List<T>> {
        Box::new(List::Nil)
    }
    
    fn prepend(self: Box<List<T>>, x: T) -> Box<Self> {
        Box::new(List::Cons(x, self))
    }
}

fn main() {
    let list = List::new();
    
    let list = list.prepend(1).prepend(2).prepend(3);
    
    println!("{:?}", list);
}
// enum List{
//     Cons(u32,Box<List>),
//     Nil,
// }

// impl List{
//     fn new()->List{
//         Nil
//     }

//     fn add(self,e:u32)->Self{
//         Cons(e,Box::new(self))
//     }

//     fn len(&self)->u32{
//         match *self{
//             Cons(_,ref tail)=>1+tail.len(),
//             Nil=>0,
//         }
//     }
//     fn stringfy(&self)->String{
//         match *self{
//             Cons(head,ref tail)=>format!("{},{}",head,tail.stringfy()),
//             Nil=>String::from_str("Nil"),
//         }
//     }
// }


// impl Display for List{
//     fn fmt(&self, fmt:&mut Formatter) -> Result{
//         write!(fmt,"{:?}",self)
//     }
// }

// fn main() {
//    let mut list=List::new();
//    list.add(2).add(3);
//    list.add(5);
//    println!("{:?}",list.len() );
//    println!("{:?}", list.stringfy());
// }