use std::mem;

struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn area(self: &Object) -> u32 {
        self.width * self.height
    }
}

fn check() -> u32 {
    0
}

fn main() {
    let mut t = (1, 2, 3, 4);
    let a = [0, 1, 2, 3, 5];
    t.0 = 69;

    let b: [i32; 5] = [12, 23, 34, 45, 56];
    println!("{}", b.len());
    println!("{}", mem::size_of_val(&b));
    println!("Hello, world!");
    println!("{:?}", a);

    let bs = &b[2..4];
    println!("{:?}", bs);

    let string = "string";
    let actual_string = String::from("string");
    let actual_string_ = string.to_string();
    println!("{} {} {}", string, actual_string, actual_string_);

    let sliced = &string[0..3];
    println!("{}", sliced);

    println!("{}", actual_string + &string);
    check();

    let o = Object {
        width: 7,
        height: 3,
    };

    let answer = o.area();
    println!("{}x{}", o.height, o.width);
    println!("{}", answer);
}
