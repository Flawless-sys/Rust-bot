
struct Pixel {
    x: u32,
    y: u32,
    color: u8
}

struct Map {

}

struct Worker {
    token: String, // the refesh token
    username: String, // Reddit username
    password: String, // Reddit Password

    client_id: String,  // Reddit developer ID
    secret_key: String, // Reddit developer secret
    place_in: usize, // The unix time
}


impl Worker {
    pub fn new(
        token: String, 
        username: String,
        password: String,

        client_id: String,
        secret_key: String,
        place_in: usize,

    ) -> Worker {

        Worker {
            token: token, 
            username: username,
            password: password,

            client_id: client_id,
            secret_key: secret_key,
            place_in: place_in,
        }

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

    pub fn format_into_json(self,) {
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




// color_map = {
//     "#FF4500": 2,  # bright red
//     "#FFA800": 3,  # orange
//     "#FFD635": 4,  # yellow
//     "#00A368": 6,  # darker green
//     "#7EED56": 8,  # lighter green
//     "#2450A4": 12,  # darkest blue
//     "#3690EA": 13,  # medium normal blue
//     "#51E9F4": 14,  # cyan
//     "#811E9F": 18,  # darkest purple
//     "#B44AC0": 19,  # normal purple
//     "#FF99AA": 23,  # pink
//     "#9C6926": 25,  # brown
//     "#000000": 27,  # black
//     "#898D90": 29,  # grey
//     "#D4D7D9": 30,  # light grey
//     "#FFFFFF": 31,  # white
// }

// name_map = {
//     2: "Bright Red",
//     3: "Orange",
//     4: "Yellow",
//     6: "Dark Green",
//     8: "Light Green",
//     12: "Dark Blue",
//     13: "Blue",
//     14: "Cyan",
//     18: "Dark Purple",
//     19: "Purple",
//     23: "Pink",
//     25: "Brown",
//     27: "Black",
//     29: "Grey",
//     30: "Light Grey",
//     31: "White",
// }



fn main() {
    println!("Hello, world!");
}
