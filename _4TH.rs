fn main(){
    
    // Array - fixed length, known at compile time
    
    let arr: [u32; 3] = [1, 2, 3];
    println!("Arr[2] = {}",arr[2]);
    
    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 9;

    let arr: [u32; 10] = [0; 10];
    println!("{:?}", arr);
    
    let nums: [i32; 10] = [-1,1,-2,2,-3,3,-4,4,-5,5];
    let s = &nums[0..3]; // First three elements
    println!("First three elements: {:?}", s);

    let s = &nums[3..7];
    println!("Middle four elements: {:?}", s);
}
