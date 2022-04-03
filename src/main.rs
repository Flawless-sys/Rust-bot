use std::fs;
use std::{collections::HashMap, thread::AccessError};
use serde;
use serde::{Deserialize, Serialize};
use::serde_json::{Value};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::SystemTime;

// {
//     "operationName": "setPixel",
//     "variables": [
//         "input": [
//             "actionName": "r/replace:set_pixel",
//             "PixelMessageData": [
//                 "coordinate": ["x": {x}, "y": y],
//                 "colorIndex": color_index_in,
//                 "canvasIndex": canvas_index,
//             ],
//         ]
//     ],
//     "query": ""
// }




struct Pixel {
    x: u32,
    y: u32,
    color: u8
}

#[derive(Debug, Deserialize, Clone)]
    // "{\"access_token\": \"470131870547-TOKEN\", \"token_type\": \"bearer\", \"expires_in\": 3600, \"scope\": \"*\"}"

struct TokenResponse {
    error: Option<String>,
    access_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<u32>,
    scope: Option<String>,
}


#[derive(Clone)]
struct Worker {
    token: String, // the refesh token
    refreshin: usize,

    username: String, // Reddit username
    password: String, // Reddit Password
    proxy: String,

    client_id: String,  // Reddit developer ID
    secret_key: String, // Reddit developer secret
    place_in: usize, // The unix time

    // workers: HashMap<String, String>,
}


#[derive(Debug, Serialize, Deserialize)]
struct Account {
    grant_type: String,
    username: String,
    password: String,
}

impl Worker {
    pub fn new(
        token: String, 
        username: String,
        password: String,
        proxy: String,
        refreshin: usize,

        client_id: String,
        secret_key: String,
        place_in: usize,

    ) -> Self {

        Self {
            token: token, 
            username: username,
            password: password,
            proxy: proxy,
            refreshin: refreshin,

            client_id: client_id,
            secret_key: secret_key,
            place_in: place_in,

        }

    }

    async fn place_pixel(&mut self, x: u16, y: u16, color_index: u8) -> Result<(), Box<dyn std::error::Error>> {
       

        Ok(())
    }
    

    async fn refresh_token(mut self) -> Result<(TokenResponse), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
    
        let user = Account {
            grant_type: String::from("password"),
            username: self.username.into(),
            password: self.password.into(),
        };
        
        let resp = client
            .post("https://ssl.reddit.com/api/v1/access_token")
            .header("User-Agent", "chrome")
            .header("content-type", "application/x-www-form-urlencoded")
            .form(&user)
            .basic_auth(self.client_id, Some(self.secret_key))
            .send()
            .await?;

        // "{\"access_token\": \"470131870547-JRCq3uYGdv8zgeSfwlgQMpcXc3OWCg\", \"token_type\": \"bearer\", \"expires_in\": 3600, \"scope\": \"*\"}"
        let js: TokenResponse = resp.json().await?;

        self.token = js.access_token.clone().unwrap();

        Ok(js)
    
    }



}


impl Pixel {
    pub fn new(x: u32, y: u32, color: u8) -> Pixel {
        Pixel {
            x, 
            y,
            color
        }
    }


    pub fn place_pixel(self) {

    }

    pub fn format_into_json(self) {
        // For reddit to place
    }

    pub fn get_colorindex_hex(self, hex: &str) -> u8 {
        match hex {
            "#FF4500" => 2, 
            "#FFA800" => 3,  
            "#FFD635" => 4,  
            "#00A368" => 6,   
            "#7EED56" => 8,   
            "#2450A4" => 12,  
            "#3690EA" => 13,  
            "#51E9F4" => 14,  
            "#811E9F" => 18,   
            "#B44AC0" => 19,   
            "#FF99AA" => 23,  
            "#9C6926" => 25,  
            "#000000" => 7, 
            "#898D90" => 29,  
            "#D4D7D9" => 30,  
            "#FFFFFF" => 31,
            _ => panic!("Error got another hex value.")  
        }
    }
}



fn task(Workers: Vec::<Worker>) {


    loop {
        
    }

    
}



#[tokio::main]
async fn main() {
    let data = fs::read_to_string("config.json").expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    println!("{}", res);
  
    /*
        Log into reddit account and add refresh token to a table
        Place pixel
        wait 5:10 to 5:30 minutes to combat anti-bot
    */




    // let worker = Worker::new(String::from(""),String::from("0x0112"),String::from("^"),String::from(""),12574689, String::from("fdaW7SfgARsWGKIXo3awjw"),String::from("i5vLrD3rPBGMKGrBO51wUbYJs5Yunw"),245234523 );
    // worker.refresh_token().await;
}
