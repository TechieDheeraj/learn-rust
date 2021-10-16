pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // With Non-premitives, if you assign another variable to a piece of data, the first variable
    // will no longer hold that value. You'll need to use a reference (&) to point to the resource.

    // Vector
    let arr3 = vec![1, 2, 3, 4];
    let arr4 = &arr3;
    println!("Values: {:?}", (arr1, arr2));
    println!("Values: {:?}", (&arr3, arr4));
}
