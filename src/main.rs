fn bubble_sort(input_arr: &mut [i64], reverse: bool) {
    for i in 0..input_arr.len() {
        for j in i + 1..input_arr.len() {
            let we_should_swap = match reverse {
                true => input_arr[i] < input_arr[j],
                false => input_arr[i] > input_arr[j],
            };
            if we_should_swap {
                let tmp = input_arr[i];
                input_arr[i] = input_arr[j];
                input_arr[j] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use rand::Rng;

    #[test]
    fn test_asc_order() {
        let mut input_arr = [1, 100, -1, 0, -200, 20, 30, 10000];
        let output_arr = [-200, -1, 0, 1, 20, 30, 100, 10000];
        bubble_sort(&mut input_arr, false);
        assert_eq!(input_arr, output_arr);
    }

    #[test]
    fn test_desc_order() {
        let mut input_arr = [1, 100, -1, 0, -200, 20, 30, 10000];
        let output_arr = [10000, 100, 30, 20, 1, 0, -1, -200];
        bubble_sort(&mut input_arr, true);
        assert_eq!(input_arr, output_arr);
    }

    #[test]
    fn test_asc_order_with_rand() {
        let mut rng = rand::thread_rng();
        let mut input_arr: Vec<i64> = (0..1000).map(|_| rng.gen_range(-1000, 1000)).collect();
        let mut output_arr = input_arr.clone();
        output_arr.sort_unstable();
        bubble_sort(&mut input_arr, false);
        assert_eq!(input_arr, output_arr);
    }

    #[test]
    fn test_desc_order_with_rand() {
        let mut rng = rand::thread_rng();
        let mut input_arr: Vec<i64> = (0..1200).map(|_| rng.gen_range(-1300, 1100)).collect();
        let mut output_arr = input_arr.clone();
        output_arr.sort_unstable_by(|a, b| b.cmp(a));
        bubble_sort(&mut input_arr, true);
        assert_eq!(input_arr, output_arr);
    }
}
