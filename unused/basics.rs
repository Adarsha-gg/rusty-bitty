// // Create a program that:
// // 1. Declares an immutable variable with your name
// // fn namer() -> &'static str{
// //     let name: &str = "pass";
// //     return name;
// // }

// // 2. Declares a mutable variable with your age
// // 3. Increments your age by 1
// // 4. Prints both variables using println!
// // fn main(){
   
// //     let mut age: u32 = 21;
// //     println!("{}", age);
// //     age = 33;
// //     println!("{}", age);
// // }


// // Create a function that:
// // 1. Takes two numbers as parameters
// // 2. Uses if/else to check which number is larger
// // 3. Returns the larger number
// // 4. Call this function from main with different test cases
// // fn hello(a:i32, b:i32) -> i32{
// //     if a >= b{
// //         return a;
// //     }
// //     else {
// //         return b;
// //     }
// // }



// // Create a program that:
// // 1. Declares an array of 5 numbers
// // 2. Uses a for loop to iterate through the array
// // 3. Calculates the sum and average of the numbers
// // 4. Prints both results

//     // let a = vec![1,2,3,4,5];
//     // let mut sum= 0;
//     // let mut _average = 0;
//     // for i in a{
//     //     sum += i
//     // }
//     // _average = sum/5;
//     // println!("{} {}",sum, _average) 

// // Create a struct called Rectangle with width and height
// // Implement methods for Rectangle:
// // 1. new() - constructor that returns a Rectangle
// // 2. area() - calculates and returns the area
// // 3. perimeter() - calculates and returns the perimeter
// // 4. is_square() - returns true if it's a square

// use core::num;
// use std::{num::IntErrorKind, ptr::read_unaligned};

// struct Rectangle{
//     width: u32,
//     height: u32
// }

// impl Rectangle{
//     fn new(width:u32, height:u32) -> Rectangle{
//         return Rectangle{width, height}
//     }
//     fn area(rect: Rectangle)-> u32{
//         rect.height*rect.width
//     }
//     fn perimeter(rect: Rectangle)-> u32{
//         2 * (rect.height+rect.width)
//     }
//     fn is_square(rect: Rectangle) ->bool{
//         if rect.height== rect.width{
//             return  true;
//         }
//         else{
//             return  false;
//         }
//     }
// }

// // Create a function that:
// // 1. Takes a string representing a number
// // 2. Converts it to an integer
// // 3. Multiplies it by 2
// // 4. Returns Result<i32, String> (Ok for success, Err for invalid input)
// // 5. Handle both success and error cases in main()
// fn stringer(a:String) -> Result<i32, String>{
//    let number:i32 = match a.parse::<i32>() {
//         Ok(n) => n * 2,
//         Err(_) => return Err("There is an error".to_string())
//    };
//    Ok(number)  
// }

// // Create an enum Shape that can be either:
// // - Circle(radius)
// // - Square(side)
// // - Triangle(base, height)
// // 
// // Create a function that:
// // 1. Takes a Shape as parameter
// // 2. Uses match to calculate and return the area
// // 3. Test it with different shapes


fn main(){
}

