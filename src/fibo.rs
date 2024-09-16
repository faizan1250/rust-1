pub fn fibo(n:i64)->i64{
    let mut a:i64 = 0;
    let mut b:i64 = 1;
    if n ==0{
        return a;
    }
     if n==b {
        return b;
    }
    let mut c:i64 = 2;
    while c<=n{
        let temp:i64 = b;
        b = a+b;
        a =temp;
        c+=1;
    }
    return b;

}

pub fn rec_fibo(n:i64)->i64{
    if n==1 || n==0{
        return n;
    }
    return rec_fibo(n-1) + rec_fibo(n-2);
}