mod class1;

fn main() {
    let org_arr = [1, 6, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    match class1::class1::is_sub_array(&org_arr, &sub_arr) {
        true => {
            println!("{:?} is subarray of {:?}", sub_arr, org_arr);
        },
        false => {
            println!("{:?} is not subarray of {:?}", sub_arr, org_arr);
        }
    }
}
