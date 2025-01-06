Try creating:

A trait called Shareable that:

Requires a method called share that returns a String
Has a default method called get_platform that returns "Unknown"
```
trait Shareable{
    fn share(&self) -> String;
    fn get_platform(&self) -> String{
        String::from("Unknown")
    }
}

```
Create two structs:

Post with fields content and likes (String and i32)
Photo with fields caption and filters (both String)

```struct Post{
    content:String,
    likes: i32
}

struct Photo{
    caption:String,
    filters:String
}
```
Implement the Shareable trait for both types:
```
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
```
Post's share should return "Sharing post: {content} with {likes} likes"
Photo's share should return "Sharing photo: {caption} with filter {filters}"
Override get_platform for Post to return "Twitter" but let Photo use the default

------------------------------------------------------


A generic struct called SocialFeed that:

Can hold a vector of any type that implements Shareable
Make sure to use the correct generic syntax with trait bounds

```struct SocialFeed <T:Shareable>{
    feed: Vec<T>
}
```

Write an implementation for SocialFeed that includes:
A method called add_item that adds a new item to the feed
A method called share_all that shares all items
```impl <T:Shareable> SocialFeed<T>{
    fn add_item(&self, T){
        self.feed.push(T);
    }

    fn share_all(&self){
        for item in self.feed{
            println("{}".item)
        }
    }

}
```

------------------------------------
---> Main messups were I didnt use the functionality to print them out. Since I inherited the share thingy, I should have use that cuz it prints out different stuff differently.
Say I have item in my feed right. If I say print item and item is. Post and a Photo struct how will it print it?
It doesnt know how to. So use the share htingy. that way
it will display according to its type. Prettyyy koool.

------------------
**final try**
A trait called Countable that:

Has a required method called count that returns an i32
Has a default method called get_unit that returns a String "items"
```
trait Countable{
    fn count(&self) -> i32;
    fn get_uint(&self) -> String
    {
        String::from("items")
    }
}

<!-- Create two structs:

Box with fields width, height, and items (all i32)
Bag with fields weight and capacity (both i32) -->
struct Box{
    width:i32,
    height:i32,
    items:i32
}

struct Bag{
    weight:i32,
    capacity:i32
}
```
Implement the Countable trait for both types where:

Box's count returns items
Bag's count returns capacity
Override get_unit for Box to return "boxes"
```
impl Countable for Box{
    fn count(&self) -> i32
    {
        return self.items
    }
    fn get_unit(&self) -> String{
        String::from("boxes")
    }
}

impl Countable for Bag{
    fn count(&self) -> i32{
        return self.capacity
    }
}
```
Create a generic struct called Inventory that:

Can hold a vector of any type that implements Countable
Has a method called add_storage that adds a new storage unit
Has a method called total_count that sums up all counts

```
struct Inventory<T:Countable>{
    item: Vec<T>
}

impl<T:Countable> Inventory<T>{
    fn add_storage(&mut self, T){
        self.push(T)
    }

    fn total_count(&self) -> i32{
        let mut total = 0
        for abcd in &self.item{
            total += abcd.count();
        }
        return total
    }
}
```


----------------------------------
I got the hang of it IG