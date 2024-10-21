// 原生类型

// 1. i8
// 2. i16
// 3. i32
// 4. i64
// 5. i128
// 6. isize
// 7. u8
// 8. u16
// 9. u32
// 10. u64
// 11. u128
// 12. usize
// 13. f32
// 14. f64
// 15. bool
// 16. char

use core::fmt;
use std::cmp::Reverse;
///
/// 运算符
fn operator(){
    // 算数逻辑
    // println!("{} + {} = {}", 1, 2f32, 1 + 2f32);
    println!("{} + {} = {}", 1, 2i32, 1 + 2i32);

    // 布尔运算
    println!("{}",true && false);
    println!("{}",true || false);
    println!("{}",!true);

    // 位运算
    println!("0011 & 0101 = {}", 0b0011u32 & 0b0101);
    println!("0011 | 0101 = {}", 0b0011u32 | 0b0101);
    println!("0011 ^ 0101 = {}", 0b0011u32 ^ 0b0101);
    println!("0011 << 2 = {}", 0b0011u32 << 2); // * n
    println!("0011 >> 2 = {}", 0b0011u32 >> 2); // /n
}

fn tuple(){
    // 元组
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    fn fn1(tup:(i32,bool))->(bool,i32){
        (tup.1,tup.0)
    }

    // let too_long_tuple=(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17);
    // println!("{:?}",too_long_tuple)

    #[derive(Debug)]
    struct Matrix(f32, f32, f32,f32);

    impl fmt::Display for Matrix{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    let matrix = transpose(matrix);
    println!("{}", matrix);

}

fn array_and_slice(){
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 初始化为同一个值
    let ys: [i32; 500] = [0; 500];

    println!("xs[2] is {}", xs[2]);
    println!("ys[2] is {}", ys[2]);

    // 输出长度
    println!("array size: {}", xs.len());
    // 切片
    let slice: &[i32] = &ys[1..4];
    println!("slice is {:?}", slice);

    // 数组在栈中的大小
    println!("array occupies {} bytes", std::mem::size_of_val(&xs));

}

fn main() {
    operator();
    tuple();
    array_and_slice();
}
