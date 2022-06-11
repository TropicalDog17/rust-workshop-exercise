// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//             let sub_arr = [6,8,10];

// Bài tập 2 : Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho.
// Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.
// https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt
fn check_subarr(org_arr: &mut [i32], sub_arr: &mut [i32]) -> i32 {
    let org_arr_length = org_arr.len();
    let sub_arr_length = sub_arr.len();

    let mut i: usize = 0;
    let mut is_subarr = false;
    //Xet tat ca cac mang con cua org_arr co phan tu dau tien trung voi phan tu dau mang sub_arr
    while i <= org_arr_length - sub_arr_length {
        //Ensuring the subarray of org_arr 's length equals to sub_arr_length

        if org_arr[i] == sub_arr[0] {
            is_subarr = true;
            //Check if all remaining elements are identical, if not exit the loop, update is_subarr flag and check another subarray
            for n in 1..sub_arr_length {
                if org_arr[i + n] != sub_arr[n] {
                    is_subarr = false;
                    break;
                }
            }
        }
        //If the flag remain true after check all remaining element, then 2 arrays are identical
        if is_subarr == true {
            println!("True");
            break;
        }
        i += 1;
    }

    if is_subarr == false {
        println!("False");
        0
    } else {
        1
    }
}
fn main() {
    let mut org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let mut sub_arr = [6];

    check_subarr(&mut org_arr, &mut sub_arr);
}

#[cfg(test)]
mod tests {
    use crate::check_subarr;

    #[test]
    fn sub_arr_length_1() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut b = [3];
        assert_eq!(check_subarr(&mut a, &mut b), 1);
    }
    #[test]
    fn sub_arr_length_multiple_middle() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut b = [3, 4, 5, 6];
        assert_eq!(check_subarr(&mut a, &mut b), 1);
    }
    #[test]

    fn not_sub_arr_length_multiple_last() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut b = [7, 8, 4, 6];
        assert_eq!(check_subarr(&mut a, &mut b), 0);
    }
}
