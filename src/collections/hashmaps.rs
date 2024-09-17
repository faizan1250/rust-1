use std::collections::HashMap;

pub fn maps(){
    /*METHODS
      1- insert
      2- get
      3- remove
      4- clear.  
        */
    let mut m1 = HashMap::new();
    m1.insert("faizan", 1);
    m1.insert("shibu", 2);
    m1.insert("arhan", 3); 
    let data = m1.get("shibuw");
    match data{
        Some(pos)=> println!("pos is {}", pos),
        None => println!("not found"),

    }
    let pair = vec![(String::from("faiz"), 1), (String::from("shibu"), 2), (String::from("faiz"),0)];
     let ans = qn_one(pair);
     println!("{:?}", ans);

}

fn qn_one(pairs: Vec<(String, i32)>)->HashMap<String,i32>{
    let mut h1 = HashMap::new();
    for (key, value) in pairs{
        h1.insert(key, value);
    }
    return h1;
}