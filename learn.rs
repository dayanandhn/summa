// Online Rust compiler to run Rust program online
// Print "Hello World!" message
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
use std::fmt;
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
       // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-10f32);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 3 is 0x{:x}", 0x80u32 >> 3);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
    
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, 'a', true, 0.23400f64);
    println!("Long tuple 1: {} 5: {} 7: {}", long_tuple.1, long_tuple.5, long_tuple.7);
    println!("Long tuple: {:?}", long_tuple);
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", (tuple_of_tuples));
    
    let pair = (1, true);
    println!("pair {:?} Reverse pair {:?}",pair,  reverse(pair));
    
    //destructuring
    
    let details = ("Daya", 14101u16, "zoho");
    let (name, id, company) = details;
    println!("name: {name} id: {id}, company: {company}", name=name, id=id, company=company);
    
    let mat = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("Matrix is\n{}", mat);
    println!("Transpose is\n{}", transpose(mat));
}

fn transpose(input_matrix: Matrix) -> Matrix{
    let Matrix(a11, a12, a21, a22) = input_matrix;
    Matrix(a11, a21, a12, a22)
}


fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (v1, v2) = pair;
    return (v2, v1)
}
