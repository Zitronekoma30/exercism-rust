pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let mut start: usize = 0;
    let mut end: usize = array.len() - 1;

    while start <= end {
        let mid: usize = start + (end - start) / 2;

        if array[mid] == key {
            return Some(mid);
        } else if key > array[mid] {
            start = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            end = mid - 1;
        }
    }
    None
}
