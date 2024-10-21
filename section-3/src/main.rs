use std::f32::consts::E;

/// 自定义类型

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
#[allow(dead_code)]
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




fn main() {
    test_init_struct();
}
