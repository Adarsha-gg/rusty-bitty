enum Shapes{
    Rectangle,
    Triangle,
    Square
}


fn main(){
    let s1 = Shapes::Square;
    let s2= Shapes::Rectangle;
    let a = "abcd";
    match s1{
        Shapes::Square => println!("tf"),
        _ => ()
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}