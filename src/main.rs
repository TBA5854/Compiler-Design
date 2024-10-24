use std::{fs::File, io::Read, path::Path};

fn add(a: &str, line: u32) -> i32 {
    let b = a.split(" ").collect::<Vec<&str>>();
    // print!("{:?}",a.split(" ").collect<Vec<&str>>());
    // print!("{:?}",b);
    let err: String = format!("Erron on Line {}", line.to_string());
    return b[1].parse::<i32>().expect(&err);
}

// fn sub(a:&str,line:u32) -> i32{
//     let b =a.split(" ").collect::<Vec<&str>>();
//     // print!("{:?}",a.split(" ").collect<Vec<&str>>());
//     // print!("{:?}",b);
//     let err:String=format!("Erron on Line {}",line.to_string());
//     return b[1].parse::<i32>().expect(&err);
// }

// fn mul(a:&str,line:u32) -> i32{
//     let b =a.split(" ").collect::<Vec<&str>>();
//     // print!("{:?}",a.split(" ").collect<Vec<&str>>());
//     // print!("{:?}",b);
//     let err:String=format!("Erron on Line {}",line.to_string());
//     return b[1].parse::<i32>().expect(&err);
// }

// fn div(a:&str,line:u32) -> i32{
//     let b =a.split(" ").collect::<Vec<&str>>();
//     // print!("{:?}",a.split(" ").collect<Vec<&str>>());
//     // print!("{:?}",b);
//     let err:String=format!("Erron on Line {}",line.to_string());
//     return b[1].parse::<i32>().expect(&err);
// }

fn parse(a: &str, mut reg: i32) -> i32 {
    // println!("a: {:?}",a);
    let b = a.split("\n").collect::<Vec<&str>>();
    // println!("b: {:?}",b);
    let mut line = 1;
    for ele in b {
        let err = format!("SYNTAX on Line {}", line.to_string());
        // println!("ele {:?}",ele);
        if ele.split(" ").collect::<Vec<&str>>()[0] == "ADD" {
            reg += add(ele, line);
        } else if ele.split(" ").collect::<Vec<&str>>()[0] == "SUB" {
            reg -= add(ele, line);
        } else if ele.split(" ").collect::<Vec<&str>>()[0] == "MUL" {
            reg *= add(ele, line);
        } else if ele.split(" ").collect::<Vec<&str>>()[0] == "DIV" {
            reg /= add(ele, line);
        } else {
            panic!("{}", &err);
        }
        line += 1;
    }
    return reg;
}

fn main() {
    // println!("Hello, world!");
    let path = Path::new("test.tba");
    let mut file = File::open(path).expect("Couldn't Open File");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error in reading");
    contents=contents.trim().to_string();
    let mut reg = 0;
    // print!("{:?}\n",contents);
    // reg+=add(&contents,1);
    reg = parse(&contents, reg);
    println!("{}", reg);
}
// reg=0;

/*
ADD

*/

// SUB
// MUL
// DIV
