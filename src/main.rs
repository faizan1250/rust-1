use std::fs;


fn main(){
      let result = fs::read_to_string("a.txt");       
      match result {
        Ok(content) => {
          println!("file content: {}", content);
        },
        Err(err) => {
          println!("error: {}", err);
        }

      }  
      println!("hjjhdf");
}

 