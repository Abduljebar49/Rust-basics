fn main() {
    let unassigned: u32 = 10;

    let signed: i32 = -10;

    let float:f64 = 0.32;

    println!("Hello, world! => {} {} {}",unassigned,signed,float);

    let array:[i32;5] = [1,2,3,4,5];

    println!("array : {:?}",array);

    let mut vector:Vec<i32> = vec![1,2,3,4,5];
    vector.push(6);

    println!("vector {:?}",vector);


}
