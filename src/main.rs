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

    //loop 
    let mut counter:i32 = 0;
    loop{
        println!("looping... {}",counter);
        counter += 1;
        if counter>=5{
            break;
        }
    }

    let mut i: i32 = 0;
    while i<10 {
       println!("Looping => {}",i);
        i+=1;
    }

    let mut j:i32 = 0;
    for j in 0..10{
        println!("looping for loop => {}",j)
    }

    //swich or matcher

    let day = 4;

    match day {
        1=> println!("Monday"),
        2=> println!("Tuesday"),
        3=> println!("Wednesday"),
        _=> println!("It is something else"),
    }


}
