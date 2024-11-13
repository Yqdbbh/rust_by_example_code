#![allow(dead_code,unused)]

struct Empty;
struct Null;

// T 的泛型trait
trait DoubleDrop<T> {
    // 定义一个调用者的方法，接受一个额外的参T,单步执行任何操作
    fn double_drop(self,this: T);
}

// 对泛型的调用者类型U和任何泛型类型T实现DoubleDrop<T>

impl<T,U> DoubleDrop<T> for U {
    // 传入两个参数的所有权，并进行释放
    fn double_drop(self,this: T) {}
}

pub fn test(){
    let empty  = Empty;
    let null = Null;
    empty.double_drop(null);

    // empty;
    // null;
}

