fn main() {
    let v = vec![10, 20, 30];
    // vector v owns the object in heap
    let v2 = v;
    display(v2.clone());
    println!("In main ({:?})", v2.clone());
}

fn display(v: Vec<i32>) {
    println!("inside display : {:?}", v);
}