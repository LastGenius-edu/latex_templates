let array: [i32; 6] = [0, 1, 2, 3, 4, 5];

let slice: &[i32] = &array[0..3]; 

for x in slice {
	println!("{}", x):
}
