pub fn vecs(){
   //vec init usuing macros

   let n = vec![1,2,3,4];
   println!("{:?}", n);


    //defining and initializing vector
   let mut vec: Vec<i32> = Vec::new();
   vec.push(1);
   vec.push(2);
   vec.push(3);
   vec.push(4);
   println!("{:?}", vec);

    let even_vec = to_even(&vec);
   println!("{:?}", even_vec);
   println!("{:?}",to_even(&n));
   println!("{:?}", vec);
   println!("{:?}", n);
   
  



}
//creating vector of even elements from og vector.
fn to_even(vec: &Vec<i32>)->Vec<i32>{
    let mut new_vec = Vec::new();
    for val in vec{
        if val % 2 ==0{
            new_vec.push(*val);
        }
    }
    return new_vec;
}
