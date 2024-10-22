/// 自定义类型
#[allow(dead_code)]

#[derive(Debug)]
struct Person{
    name: String,
    age: u8
}

// 单元结构体
struct  Unit;

// 元组结构体 具名元组
struct Pair(i32,i32);

#[derive(Debug)]
struct Point{
    x: f32,
    y: f32
}

// 结构体作为另一个结构体的字段

struct Rectangle{
    top_left: Point,
    bottom_right: Point
}

fn test_init_struct(){
    // 简单写法初始化字段
    let name_1 = String::from("Perter");
    let age = 27;
    // 这里如果是同名可以直接赋值，如果不同名，需要手动指定字段,字段顺序无关
    let p = Person{age,name:name_1};
    // 引用类型会传递所有权，所以这里name_1不可以再次使用
    let p2 = Person{name:String::from("Tom"), age};

    println!("p: {:?}", p);
    println!("p2: {:?}", p2);

    // 实例化 Point
    let point:Point = Point{x:3.3,y:7.2};
    println!("point: {:?}", point);

    // 使用结构体更新语法创建新的 point, 此时y不赋值，会从前一个解构出来
    let new_point = Point{x:10.3,..point};
    println!("new_point: {:?}", new_point);    

    let _rectangle = Rectangle{
        top_left: Point{x:1.0,y:2.0},
        bottom_right: new_point
    };

    // 实例化单元结构体
    let _unit = Unit;

    // 实例化元组结构体
    let pair = Pair(1,2);
    // 访问元组结构体数据
    println!("pair: x:{}, y:{}", pair.0, pair.1); 
    // 元组结构体结构
    let Pair(item1,item2) = pair;
    println!("pair: item1:{}, item2:{}", item1, item2);

    // 小练习 增加一个计算 Rectangle （长方形）面积的函数 rect_area（尝试使用嵌套的解构方式）
    fn reat_area(rect:&Rectangle) -> f32{
        // 解构，属性名:参数名 --> top_left:tl
        let Rectangle{top_left:tl,bottom_right:br} = rect;
        (br.x - tl.x) * (br.y - tl.y)
    }
    let area = reat_area(&_rectangle);
    println!("area: {}", area);

    // 增加一个函数 square，接受的参数是一个 Point 和一个 f32，并返回一个 Rectangle（长方形），其左上角位于该点上，长和宽都对应于 f32。
    fn square(point:Point,side:f32) -> Rectangle{
        let top_left = Point{x:point.x,y:point.y};
        let bottom_right = Point{x:point.x+side,y:point.y+side};
        Rectangle{
            top_left,
            bottom_right
        }
    }

}

fn test_enum(){
    enum WebEvent{
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click{x:i32,y:i32}
    }

    fn inspect(event:WebEvent){
        match event{
            WebEvent::PageLoad => println!("PageLoad"),
            WebEvent::PageUnload => println!("PageUnload"),
            WebEvent::KeyPress(c) => println!("KeyPress:{}",c),
            WebEvent::Paste(s) => println!("Paste:{}",s),
            WebEvent::Click{x,y} => println!("Click:({},{})",x,y)
        }
    }

    let pressed = WebEvent::KeyPress('a');
    inspect(pressed);

    // 类型别名， 当枚举类型过长时使用
    type NewTypeWebEvent = WebEvent;
    let click = NewTypeWebEvent::Click{x:10,y:20};
    inspect(click);

}

fn test_enum_use(){
    enum Status {
        Rich,
        Poor
    }

    enum Work{
        Civilian,
        Soldier
    }

    fn test_a(){
        let status = Rich;
        // 引入use 后可以直接使用枚举的值，测试下来，use可以写后面
        use Status::{Poor,Rich};
        let status2 = Poor;
        let status3 = Rich;
    }

    
}

/// 枚举类型转i32
fn test_enum_i32(){
    enum Enum1 {
        Rich,
        Poor
    }
    // 默认从0开始，后续加1
    println!("Enum1 Rich: {}",Enum1::Rich as i32);
    println!("Enum1 Poor: {}",Enum1::Poor as i32);

    enum Enum2{
        Rich = 2,
        Poor
    }
    // 可以自定义指定
    println!("Enum2 Rich: {}",Enum2::Rich as i32);
    println!("Enum2 Poor: {}",Enum2::Poor as i32);

    enum Enum3{
        Rich = -2, // 也可以从负数开始
        Poor
    }
    println!("Enum3 Rich: {}",Enum3::Rich as i32);
    println!("Enum3 Poor: {}",Enum3::Poor as i32);

    // enum Enum4{
    //     Rich = -2, // 此时将会出错，-3 + 1 的值已经被使用
    //     Poor = -3,
    //     Err
    // }
    // println!("Enum4 Rich: {}",Enum4::Rich as i32);
    // println!("Enum4 Poor: {}",Enum4::Poor as i32);
    // println!("Enum4 Poor: {}",Enum4::Err as i32);
    
}

fn linked_list(){
    enum List {
        // Box 语法后续再学
        Cons(i32, Box<List>),
        Nil
    }

    use List::*;

    // 为枚举实现方法
    impl List{
        fn new() -> List{
            Nil
        }

        // 添加元素
        fn prepend(self,elem:i32)->List{
            Cons(elem,Box::new(self))
        }

        // 获取长度
        fn len(&self) -> i32{
            // 必须对self 进行匹配，方法的行为取决于self的具体类型
            // self 为 &List 的类型，*self 为 List 类型，匹配一个具体的 T
            match *self{
                // 不能得到tail的所有权，因为self是借用的，因此使用一个对tail的引用
                Cons(_,ref tail) => 1 + tail.len(),
                // 递归结束
                Nil => 0
            }
        }

        // 返回列表的字符串表示
        fn stringify(&self) -> String{
            match *self{
                Cons(head,ref tail) => {
                    // format! 返回堆上的字符串，print! 将结果打印到标准输出
                    format!("{}, {}",head,tail.stringify())
                },
                Nil => {
                    format!("Nil")
                }
            }
        }

    }
    // test
    let mut list = List::new();
    list = list.prepend(1).prepend(2).prepend(3);
    println!("linked list: {}",list.stringify());
    println!("linked list len: {}",list.len());

}

fn constant_variables(){
    static mut LANGUAGE:&str="Rust"; // 静态变量，mut可变
    const PI:f32=3.14; // 常量 mut不可变

    // todo 'static 声明周期

}

fn main() {
    // test_init_struct();
    // test_enum();
    // test_enum_use();
    // test_enum_i32();
    linked_list();
}
