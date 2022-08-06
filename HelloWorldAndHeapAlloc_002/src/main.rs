use std::mem;
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() {
    println!("Hello, world!");

    let mut x : u32 = 5;
    let y : isize = 3;
    x = 11;
    println!("{}", x);
    println!("{}", mem::size_of::<* const isize>());


    let heap_vector: Vec<i8> = Vec::new();
    let heap_vector1: Vec<u8> = vec![5,2];
    let heap_vector3 = heap_vector1.clone();

    println!("{}", heap_vector1[1]);
    println!("{}", heap_vector3[1]);
    println!("{:?}", heap_vector);

    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    heap_procedure(&heap_f64);

    println!("{}", heap_f64);

}


fn stack_procedure(mut param: f64){
    param +=9.;
    println!("Stack here: {}", param);
}

fn heap_procedure(param: &Box<f64>){
    println!("Heap here: {}", param);
}

