fn quicksort<T: Ord>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let pi = partition(v);
    quicksort(&mut v[0..pi]);
    quicksort(&mut v[pi+1..]);
}

fn partition<T: Ord>(v: &mut [T]) -> usize {
    let pivot_index = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot_index {
        if v[j] <= v[pivot_index] {
            v.swap(i,j);
            i += 1;
        }
    }
    v.swap(i,pivot_index);
    i
}

fn main() {
    let mut v = [3, 14, 1, 7, 9, 8, 11, 6, 4, 2, 3, 4, 5, 6, 7, 8, 9];
    quicksort(&mut v);
    println!("{:?}", v);
}
