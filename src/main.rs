
//------------------------------------------
//-----------   MAKING RESTFUL API ---------
// ----------------------------------------

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate reqwest;
use std::collections::HashMap;   //for collection of data
use std::error::Error;           //for error handling
use std::io::prelude::*;        // for input output operations
use std::fs::File;              // files for file handlings
use std::path::Path;            //Path for directory
extern crate rustc_serialize;
use rustc_serialize::json::Json; // does serialization in JSON format ; basically meaning  
use std::io::Read;               //to read data from file

// http method get
#[get("/")]

// entry point of get request on this route
fn hello() -> String{
    let path = Path::new("api.json");
    let display = path.display();
    
    println!("{:?}{}",path,display);

    let mut file = match File::create(path){
        // for creating api.json file if not already present
        Ok(file) => file,
        Err(_) => panic!("Could not create the file"),
    };
    match reqwest::get("http://api.openweathermap.org/data/2.5/weather?q=Karachi&appid="){
        Ok(mut response) => {    // getting response of API
            match response.text(){          // for writting response into text file
                Ok(text) => match file.write_all(text.as_bytes()){ //writting all response to the file
                    Ok(_) => println!("Data Written In File"),
                    Err(e) => println!("The error in file"),
                }
                Err(_) => println!("The response is not comming from the main server(API)"), 
            }
        }
        Err(_) => println!("Server Couldnot establish the connection with API"),
    }

    let mut file = match File::open(&path){
        Ok(file) => file,
        Err(e) => panic!("The file open error : {}", e),
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();  //Error handling
    //result comming 
    let json = Json::from_str(&buffer).unwrap(); //conversion of data from string to JSON

    let result = format!("The Temperature of Karachi is : {} ", json.find_path(&["main"]).unwrap());  //format is used to manage string

    result
}


fn main(){
    rocket::ignite().mount("/",routes![hello]).launch();
    //ignite starts the server 
}
