fn bubble_sort(input_arr: &mut [i64], reverse: bool) {
    let mut tmp: i64;

    for i in 0..input_arr.len() {
        for j in i + 1..input_arr.len() {
            if reverse && (input_arr[i] < input_arr[j]) || (input_arr[i] > input_arr[j]) {
                tmp = input_arr[i];
                input_arr[i] = input_arr[j];
                input_arr[j] = tmp;
            }
        }
    }
}

fn main() {
    let mut input_arr = [1, 100, -1, 0, -200, 20, 30, 10000];

    bubble_sort(&mut input_arr, false);
    println!("Hello, ascending order: {:?}", input_arr);

    bubble_sort(&mut input_arr, true);
    println!("Hello, descending order: {:?}", input_arr);
}
