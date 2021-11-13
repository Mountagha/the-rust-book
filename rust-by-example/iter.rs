
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4, 5, 6, 7]);

    let greater_than_3: Vec<_> = v2.into_iter().filter(|x| x > &3).collect();
    assert_eq!(greater_than_3, vec![4, 5, 6, 7])
}
