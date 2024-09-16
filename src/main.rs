mod fibo;


fn main(){
  //let ans = fibo::fibo(1);
  for i in 0..11{
    println!("{}", fibo::rec_fibo(i));
  }
  //println!("{}", ans);


}
