#![allow(dead_code)]


fn partition<T>(v: &mut Vec<T>, left: isize, right: isize) -> isize 
where T: Ord
{
    let pivot = right as usize;
    let mut i = left - 1;
    let mut j = right;

    loop {
        i += 1;
        while v[i as usize] < v[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && v[j as usize] > v[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            v.swap(i as usize, j as usize);
        }

    }


    v.swap(i as usize, pivot);

    return i as isize;
} 

fn impl_quick_sort<T>(v: &mut Vec<T>, left: isize, right: isize)
where T: Ord
{
    
    if left < right {
        let pivot_index = partition(v, left, right);
        impl_quick_sort(v, left, pivot_index - 1);
        impl_quick_sort(v, pivot_index + 1, right);
    }

}

pub fn quick_sort<T>(v: &mut Vec<T>) 
where T: Ord
{
    let len = v.len();
    if len > 1 {
        impl_quick_sort(v, 0, (len - 1) as isize);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        quick_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        quick_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }


}

