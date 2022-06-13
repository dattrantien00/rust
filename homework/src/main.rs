fn main() {
    //ex1
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
    println!("baitap1");
    check_sublist(&org_arr, &sub_arr);
    println!("------------------------------------------------");


    //ex2
    let input = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");
    let sublist= String::from("is is");
    println!("baitap2");
    count_sublist(input,sublist);
}

//ex1
fn check_sublist(a:&[i32],b:&[i32])
{
    for element in b.iter()
    {
        let mut check=0;
        for i in a.iter()
        {
            if element ==i
            {
                check=1;
                break;
            }
        }
        if check==0
        {
            println!("chuoi 2 khong phai chuoi con chuoi 1");
        }
    }
    println!("chuoi 2 la chuoi con chuoi 1");
}

//ex2

fn count_sublist(input:String,sub:String)
{
    print!("so lan xuat hien cua chuoi {} la {}",sub,input.matches(&sub).count());
}