// start and end are valid index in array.
fn merge_sort(arr: & mut[u32], start: usize, end: usize) {
    let len = end - start + 1;
    if len > 1 {
        let mid = start + ((end - start) / 2);
        merge_sort(arr, start, mid);
        merge_sort(arr, mid+1, end);

        let mut dup = vec![0; len];

        {
            let (mut si, mut ei, mut i) = (start, mid+1, 0);
            while si <= mid && ei <= end {
                if arr[si] < arr[ei] {
                    dup[i] = arr[si];
                    si = si + 1;
                } else {
                    dup[i] = arr[ei];
                    ei = ei + 1;
                }
                i = i + 1
            }
            while si <= mid {
                dup[i] = arr[si];
                i = i + 1;
                si = si + 1;
            }
            while ei <= end {
                dup[i] = arr[ei];
                i = i + 1;
                ei = ei + 1;
            }
        }

        let (mut ai, mut di) = (start, 0);
        while ai <= end {
            arr[ai] = dup[di];
            ai = ai + 1;
            di = di + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_sort() {
        let mut actual = vec![1,2,3,5,4];
        let last_index = actual.len()-1;
        merge_sort(actual.as_mut_slice(), 0, last_index);
        assert_eq!(actual, [1,2,3,4,5]);
    }

    #[test]
    fn my_sort2() {
        let mut actual = vec![];
        let last_index = 0;
        merge_sort(actual.as_mut_slice(), 0, last_index);
        assert_eq!(actual, []);
    }

    #[test]
    fn my_sort3() {
        let mut actual = vec![1];
        let last_index = actual.len()-1;
        merge_sort(actual.as_mut_slice(), 0, last_index);
        assert_eq!(actual, [1]);
    }

    #[test]
    fn my_sort4() {
        let mut actual = vec![1,0];
        let last_index = actual.len()-1;
        merge_sort(actual.as_mut_slice(), 0, last_index);
        assert_eq!(actual, [0,1]);
    }

    #[test]
    fn my_sort5() {
        let mut actual = vec![5,4,3,2,1];
        let last_index = actual.len()-1;
        merge_sort(actual.as_mut_slice(), 0, last_index);
        assert_eq!(actual, [1,2,3,4,5]);
    }
}
