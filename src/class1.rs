// [Rust Workshop] Class 1 - Basic of Rust

pub mod class1 {
    fn is_sub_array(org_arr: &[i32], sub_arr: &[i32]) -> bool {
        if org_arr.len() < sub_arr.len() {
            return false;
        }

        for i in 0..(org_arr.len() - sub_arr.len()) {
            if is_equal_array(&org_arr[i..(i+sub_arr.len())], sub_arr) {
                return true;
            }
        }
        return false;
    }

    fn is_equal_array(arr1: &[i32], arr2: &[i32]) -> bool{
        if arr1.len() != arr2.len() {
            return false;
        }
        for i in 0..arr1.len() {
            if arr1[i] != arr2[i] {
                return false;
            }
        }
        return true;
    }

    pub fn exercise1() {
        let org_arr = [1, 6, 3, 5, 6, 8, 10, 11];
        let sub_arr = [6, 8, 10];
        match is_sub_array(&org_arr, &sub_arr) {
            true => {
                println!("{:?} is subarray of {:?}", sub_arr, org_arr);
            },
            false => {
                println!("{:?} is not subarray of {:?}", sub_arr, org_arr);
            }
        }
    }
}