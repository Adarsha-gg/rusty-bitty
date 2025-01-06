// Create a trait called Displayable that:

// Requires a method called show that returns a String
// Has a default method called version that returns the String "v1.0"

trait Displayable{
    fn show() -> String;
    fn version() -> String{
        "v1.0"
    }
}

// Create two structs:

// Movie with fields title and year
// Book with fields name and author

struct Movie{
  
    title:String,
    year:i32
}

struct Book{
    
    name:String,
    author:TString
}

// Implement the Displayable trait for both structs:
impl Displayable for Movie{
    fn show(&self) -> String{
        ("Movie: {} ({})",self.title,self.year)
    }

    fn version() -> String{
        "v2.0"
    }
}
// Movie's show should return "Movie: {title} ({year})"
// Book's show should return "Book: {name} by {author}"
impl Displayable for Book{
    fn show(&self) -> String{
        ("Book: {} ({})",self.name,self.author)
    }
}
// Let Book use the default version, but override version for Movie to return "v2.0"

struct Library<T:Displayable>{
    item: Vec<T>
} 

impl Library{
    fn display_all(){
        for abcd in item{
            println("{}", abcd)
        }
    }
}
// Create a generic struct called Library that:

// Can hold a vector of any type that implements Displayable
// Has a method called display_all that prints all items

---------------------------------------


Try creating:

A trait called Shareable that:

Requires a method called share that returns a String
Has a default method called get_platform that returns "Unknown"

trait Shareable{
    fn share(&self) -> String;
    fn get_platform(&self) -> String{
        String::from("Unknown")
    }
}


Create two structs:

Post with fields content and likes (String and i32)
Photo with fields caption and filters (both String)

struct Post{
    content:String,
    likes: i32
}

struct Photo{
    caption:String,
    filters:String
}

Implement the Shareable trait for both types:

impl Shareable for Post{
    fn share(&self) -> String{
        format!("Sharing post: {} with {} likes",self.content, self.likes)
    }

    fn get_platform(&self) -> String{
        from::String("Twitter")
    }
}

impl Shareable for Photo{
    fn share(&self) -> String{
        format!("Sharing photo: {} with filter {}",self.caption, self.filters)
    }

}

Post's share should return "Sharing post: {content} with {likes} likes"
Photo's share should return "Sharing photo: {caption} with filter {filters}"
Override get_platform for Post to return "Twitter" but let Photo use the default