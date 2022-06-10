// [Rust Workshop] Class 1 - Basic of Rust

pub mod class1 {
    pub fn is_sub_array(org_arr: &[i32], sub_arr: &[i32]) -> bool {
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
}