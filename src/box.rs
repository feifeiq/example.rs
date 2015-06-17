#![feature(box_syntax, box_patterns)]
#![feature(associated_consts)]
use std::rc::Rc;
use std::any::Any;

trait Foo {
    const ID: i32;
}

impl Foo for i32 {
    const ID: i32 = 1;
}

struct BigStruct {
    one: i32,
    two: i32,
    // etc
    one_hundred: i32,
}

fn foo(x: Box<BigStruct>) -> BigStruct {
    *x
}

fn main() {
    let x:Box<Any>=box 1;
    println!("{:?}", x.downcast::<i32>().ok().unwrap());

    let x = Box::new(BigStruct {
        one: 1,
        two: 2,
        one_hundred: 100,
    });
    let x = box BigStruct {
        one: 1,
        two: 2,
        one_hundred: 100,
    };

    let y: Box<BigStruct> = box foo(x);

    let b = Some(box 5);
    match b {
        Some(box n) if n < 0 => {
            println!("Box contains negative number {}", n);
        },
        Some(box n) if n >= 0 => {
            println!("Box contains non-negative number {}", n);
        },
        None => {
            println!("No box");
        },
        _ => unreachable!()
    }

    // let rc_ptr:Rc<i32> = box 1;
    // let box_ptr:Box<i32> = box 1;
    // let box_on_heap: Box<i32> = box(HEAP) 1;
}