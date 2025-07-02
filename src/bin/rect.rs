
struct Rectangle{
    height: u32,
    width: u32
}

impl  Rectangle {
    fn wtf_upretry(&self) -> u32{
        self.height*self.width
    }
}
fn main()
{
    let rect1 = Rectangle{height: 2, width: 2};
    print!("{}", rect1.height * rect1.width);
    print!("{}", rect1.wtf_upretry())
}