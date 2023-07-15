use std::fs::File;
use std::io::Write;


fn main () {   
    
    let mut file_thing = File::create("hello.txt").expect("error");


    loop{
        File::write(&mut file_thing, b"Delete this file afterwards if you want to run this again :D \n").expect("error");
    }

}
