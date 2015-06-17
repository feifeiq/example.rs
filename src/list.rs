#![feature(box_syntax)]
use List::{Cons,Nil};
use std::fmt::{Display,Result,Formatter};
use std::mem;

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T:Display> List<T> {
    fn new() -> Box<List<T>> {
        Box::new(List::Nil)
    }
    
    fn prepend(self: Box<List<T>>, x: T) -> Box<List<T>> {
        Box::new(List::Cons(x, self))
    }
    
    fn concat(mut self: Box<List<T>>, l: Box<List<T>>) -> Box<List<T>> {
        match self.take() {
            List::Nil => l,
            List::Cons(x, rest) => Box::new(List::Cons(x, rest.concat(l)))
        }
    }
    
    fn take(&mut self) -> List<T> {
        let mut x = List::Nil;
        mem::swap(&mut x, &mut *self);
        x
    }

    fn len(&self)->u32{
        match *self{
            Cons(_,ref tail)=>1+tail.len(),
            Nil=>0,
        }
    }

 fn stringify(self:&List<T>) -> String {
        match *self {
            Cons(ref head, ref tail) =>  format!("{}, {}", head, tail.stringify()),
            Nil =>format!("Nil"),
        }
    }
   
}



fn main() {
    let list = List::new();
    
    let list = list.prepend(1).prepend(2).prepend(3);
    
    println!("{:?}", list);
    
    let list2 = List::new();
    let list2 = list2.prepend(6).prepend(7).prepend(8);
    
    println!("{:?}", list2);

    println!("{:?}", list2.stringify());
    
    let list=list.concat(list2);
    println!("{:?}",list );

    println!("linked list has length: {}", list.len());
    println!("{:?}", list.stringify());
     
}
