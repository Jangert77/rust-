fn main(){
    let a1 = b'a';
    let a2 = b'A';
    let z1 = b'z';
    let z2 = b'Z';
    println!("'a'到'Z'之间的字符为");
    let mut n = a1 - 1;
    while n > z2{
        println!("{}",n as char);
        n = n - 1;
    }
    println!("'A'到'z'之间的字符为");
    for i in a2+1..z1{
        println!("{}",i as char);
    }
}