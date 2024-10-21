use std::{env::VarError, fmt};

/// 输出格式化
fn format_output(){
    println!("Hello, world!");
    println!("Hello,123");
    
    println!("Hello, {}", "world");
    // 使用变量来替代字符串参数
    println!("{0},{1}", "Hello", "world");
    // 使用命名参数
    println!("{name} , {age}", name = "hello", age = 123);
    println!("{name} , {name}", name = "hello");

    // 特殊格式字符串
    println!("{} of {:b} people know binary, the others half don't", 1,2);
    println!("{:b},{:b}",2i32,-2i32);
    
    // 补齐宽度
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>-width$}", number = "1234567", width = 6);
    // 宽度补0
    println!("{number:>0width$}", number = 1, width = 6);

    // 打印结构体
    #[allow(deed_code)]
    #[derive(Debug)]
    struct Sructure(i32);

    println!("This struct '{:?}' will print", Sructure(3));
}

fn debug_out(){
    println!("{:?} months in a year", 12);

    // 不一定非要完全符合参数顺序
    // {:?} 会把字符串的“"” 打印出来
    println!("{1:?} {0:?} is the {actor:?} name",
        "Slater",
        "Christian",
        actor = "actor's"
    );
}

#[derive(Debug)]
struct  Person<'a>{
    name: &'a str,
    age: u8
}

fn print_struct(){
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};
    println!("{:#?}", peter);
}

fn impl_display(){
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // 只在impl内部生效
    impl fmt::Display for Person<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "name: {}", self.name)
        }
    }

    let s = Structure(7);
    println!("s: {}", s);

    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};
    println!("peter: {}", peter);

    #[derive(Debug)]
    struct MinMax(i64,i64);

    impl fmt::Display for MinMax{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D{
        x: f64,
        y: f64
    }
    impl fmt::Display for Point2D{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    let point = Point2D{x: 3.3, y: 7.2};
    println!("Display Point: {}", point);
    println!("Debug Point: {:?}", point);

    #[derive(Debug)]
    struct Complex{
        real:f32,
        imag:f32
    }

    impl fmt::Display for Complex{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.imag < 0.0 {
                write!(f, "{} {}i", self.real, self.imag)
            }else{
                write!(f, "{} + {}i", self.real, self.imag)                
            }
        }
    }

    let c = Complex{real: 3.3, imag: -4.5};
    println!("Display: {}", c);
    println!("Debug: {:?}", c);

}

fn print_list(){

    #[derive(Debug)]
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1,2,3,4,]);
    println!("{}", v);
    println!("{:?}",v);
}

fn print_format(){
    #[derive(Debug)]
    struct Color{
        red: u8,
        green: u8,
        blue: u8
    }
    impl fmt::Display for Color{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}", 
                self.red, self.green, self.blue, self.red, self.green, self.blue)
        }
    }

    let color = Color{red:0,green:0,blue:0};
    println!("{:?}",color);
    println!("{}",color);
}

/// 主函数
fn main() {   
    // format_output();
    // debug_out();
    // print_struct();
    // impl_display();
    // print_list();
    print_format();
}