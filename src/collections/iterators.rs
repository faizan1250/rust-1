pub fn iters(){
    //  //in rust the iterators are lazy meaning they have no effect until we call methods that consumes iterators to use em up.

     //iterating using for loops
     let nums = vec![1,2,3];

     for i in &nums{
        println!("{}", i);
     }
     //iterating after creating an iter
     let iter = nums.iter();
     for i in iter{
        println!("{}", i);
     }
     //iterating using next.
     let mut itt = nums.iter();
     while let Some(val) = itt.next(){
        println!("{}", val);
     }
     println!("{:?}", nums);

     //mutable iterator to change data of vector
     let mut arr = vec![2,3,4];

     let it = arr.iter_mut();

     for i in it{
        *i = *i *2;
     }
     println!("{:?}", arr);
     
     
     //IntoIter
     //the into_iter() trait is used to convert a collection into an iterator that takes ownership of that collection. 
     let n = vec![7,8,9];
     let it = n.into_iter();

     for i in it{
        println!("{}", i);
     }
     //CONSUMING ADAPTERS-methods that call next are called consuming adapters as they uses up the iterator when called. eg is .sum();

     let x = vec![4,5,6];
     let it = x.iter();
     let sum: i32 = it.sum();
     println!("{}", sum);
     println!("{:?}", x);//since iter borrowed from vector and didnt took ownership, vec is still there in the heap. if we change iter to into_iter(), we willn't be able to print vec as into_iter() will take the ownership of the vector!

     //ITERATOR ADAPTERS-methods defined on the iterator trait that does not consume the iterator. they produce different iterators by chnging some aspect of the OG iterator. example- .map().

     let y = vec![6,5,4];
     let iet = y.iter();
     let it2 = iet.map(|x| x+1);
 

     for i in it2{
        println!("{}", i);
       
     }

     let ans = double_the_odd(vec![8,7,6,5]);
     println!("{:?}",ans); 
     
     
     
    }
     
     
     
     fn double_the_odd(v:Vec<i32>)->Vec<i32>{
        let mut vecc  = Vec::new(); 
        for i in v{
            if i %2!=0 {
               vecc.push(i);
            }
        }
        let iter = vecc.iter_mut();
         for i in iter{
            *i*=2;
         }
         return vecc;
     }