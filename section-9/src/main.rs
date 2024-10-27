#![allow(dead_code,unused)]
use std::mem;

///
/// 函数

//

use std::default;

struct Point{
    x: i32,
    y: i32
}

/// 方法，是依赖对象的函数，通过self来访问对象的数据和其他内容，在impl 中定义
impl Point{

    // 静态方法，不需要被实例化，因此没有self，通过类型的名称调用 ::
    fn origin()->Point{
        Point{x: 0, y: 0}
    }
    // 也是静态方法
    fn new(x:i32,y:i32)->Point{
        Point{x,y}
    }
}

struct Rectangle{
    p1: Point,
    p2: Point
}

impl Rectangle {
    // 实例方法，需要传入self  调用方式为 rect.area()
    fn area(&self) -> i32{
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }

    // 带参数的示例方法,第一个参数必须是&self
    fn contains(&self,point:&Point) -> bool{
        point.x > self.p1.x && point.x < self.p2.x && point.y > self.p1.y && point.y < self.p2.y
    }

    // 调用者可变
    fn translate(&mut self, x: i32, y: i32) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destory(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // first 和 second 会被销毁，self 也会被销毁
    }
}

fn test(){
    let rect = Rectangle{
        p1:Point::origin(), // 静态方法调用
        p2:Point::new(3,4)
    };

    let p2 = Point::new(2,3);
    println!("rect contains p2? {}", rect.contains(&p2)); // 实例方法调用
    println!("rect area: {}", rect.area());

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destory();
    // destory 在定义时，拿到了self 的所有权，代码块执行完后就释放了，
    // println!("{:?}",pair); 
}

/// 函数闭包 lambda
fn closure_test(){
    // 声明时使用 || 替代 () 将输入参数括起来
    // 函数体定界符（{}）对于单个表达式是可选的，其他情况必须加上。
    // 有能力捕获外部环境的变量。
    let closure = |i:i32|i+1;
    let i = 1;
    println!("{}",closure(i));

    let closure = ||println!("hello closure");
    closure();
    let closure = |a:bool,b: bool |a||b;

    // 闭包的变量捕获
    // &T、&mut T 和 T
    // 优先捕获变量，仅在需要时使用其他方式

    let color = String::from("color");
    let print = || println!("{}",color);
    print();
    let _rebrrow = &color;
    print();

    let mut count = 0;
    let mut inc = || { // 因为count 时mut的，这里闭包也得是
        count += 1;
        println!("count is {}",count);
    };
    inc();
    // let _rebrrow = &count; // 可变引用之间，不可以有只读的引用
    inc();
    let _rebrrow = &count;

    // 不可复制类型的变量
    let moveable = Box::new(3);
    let consume = || {
        println!("moveable: {}",moveable);
        mem::drop(moveable);
    };
    consume();
    // consume(); // removed value

    let haystack = vec![1,2,3];
    // 默认捕获只读引用
    // let concats = |v|haystack.contains(v);
    // 强行将捕获的变量移动到表达式内部
    let concats = move |v|haystack.contains(v);
    println!("{}",concats(&2));
    println!("{}",concats(&4));
    // println!("elements count is {}",haystack.len());

}

/// 闭包作为参数
/// 闭包作为参数时，必须声明类型
/// Fn --> &T
/// FnMut --> &mut T
/// FnOnce --> T （move）
fn closure_as_param(){
    // 例如用一个类型说明为 FnOnce 的闭包作为参数。这说明闭包可能采取 &T，&mut T 或 T 中的一种捕获方式，
    // 但编译器最终是根据所捕获变量在闭包里的使用情况决定捕获方式。
    // 这是因为如果能以移动的方式捕获变量，则闭包也有能力使用其他方式借用变量。
    // 注意反过来就不再成立：如果参数的类型说明是 Fn，那么不允许该闭包通过 &mut T 或 T 捕获变量。
    fn apply<F>(f:F) where // <F> 泛型
        F:FnOnce()
    {
        f();
    }

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = ||{
        println!("I said {}.",greeting);
        farewell.push_str("!!!");
        println!("I also said {}.",farewell);
    };
    apply(diary);
    // println!("{}",greeting);
}

/// 函数作为输入，需要定义接收闭包作为参数的函数，
/// 实现了 Fn、FnMut、 FnOnce的函数就可以做为上述函数的参数 
/// 闭包就是匿名函数
fn fn_as_param(){
    fn call_me<T>(f: T) where 
        T:Fn(){
        f();
    }

    fn hello(){
        println!("hello +++");
    }
    call_me(hello); // hello 是函数

    let hello = ||{
        println!("hello ---");
    };
    call_me(hello); // hello 是闭包
}

fn fn_as_output(){
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
    
        move || println!("This is a: {}", text)
    }
    
    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
    
        move || println!("This is a: {}", text)
    }
    
    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        
        move || println!("This is a: {}", text)
    }
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

/// 十分抽象的发散函数
fn deverging_fn() {

    /// 函数返回值为 ！ 为发散函数，返回值可能是任意类型
    fn foo()->!{ 
        panic!("This call never returns");
        // println!("This will never be returns");
    }
    
}

fn main() {
    // test();
    // closure_test();
    // closure_as_param();
    // fn_as_param();
    // fn_as_output();
    deverging_fn();
}
