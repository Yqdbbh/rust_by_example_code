use std::fmt;
// 类型转换
use std::convert::From;
use std::str::FromStr;


fn from_trait(){
    let my_str  ="hello";
    let my_string = String::from(my_str);
    println!("my_string: {}", my_string);

    // 自定义类型转换 from
    #[derive(Debug)]
    struct Number{
        value: i32
    }

    impl From<i32> for Number{
        fn from(item: i32) -> Self {
            Number{
                value: item
            }
        }
    }
    let num = Number::from(30);
    println!("num: {:?}", num);

    // 实现了from trait 默认自动实现Into

    let int = 5;
    let num : Number = int.into();
    println!("num: {:?}", num);
    
}

fn into_trait(){
    // 自定义类型转换 from
    #[derive(Debug)]
    struct Number{
        value: i32
    }

    impl From<i32> for Number{
        fn from(item: i32) -> Self {
            Number{
                value: item-5
            }
        }
    }
    let num = Number::from(30);
    println!("num: {:?}", num); // 30:i32 - 5 = 25

    // 实现了from trait 默认自动实现Into
    let int = 5;
    let num : Number = int.into(); // int:i32 - 5 --> 5:i32 - 5 = 0
    println!("num: {:?}", num);
}

fn try_from_trait(){
    // try_from 和 try_into 用于容易出错时的转换

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);
    impl TryFrom<i32> for EvenNumber{
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value%2 == 0{
                Ok(EvenNumber(value))
            }else{
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // try_into
    let result:Result<EvenNumber,()> =8.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result:Result<EvenNumber,()> =5.try_into();
    assert_eq!(result, Err(()));
}

fn to_string_and_from_str(){
    // 通过实现 fmt::Display 来实现 to_string
    #[derive(Debug)]
    struct Circle{
        radius: f64
    }
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle{radius: 6.0};
    println!("circle: {}", circle.to_string());

    // 反向转换, 下面这段是AI自动生成的
    impl FromStr for Circle{
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let radius = s.parse::<f64>().map_err(|_|())?;
            Ok(Circle{radius})
        }
    }

    let parsed_circle = "5.5".parse::<Circle>().unwrap();
    println!("parsed_circle: {:?}", parsed_circle);
}


fn main() {
    // from_trait();
    // into_trait();
    // try_from_trait();
    to_string_and_from_str();
}
