fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    {
        let v = vec![1, 2, 3, 4];
        println!("{:?}", v);
    } // v out of scope

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v6[100];
    let _does_not_exist = v.get(100);

    /*
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is {}", first);
    */

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
