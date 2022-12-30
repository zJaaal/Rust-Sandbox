use std::fmt::Debug;

#[derive(PartialEq, Debug)]
struct Groups<T>
where
    T: Debug,
{
    inner: Vec<T>,
}

impl<T> Groups<T>
where
    T: Debug,
{
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T>
where
    T: Debug,
{
    type Item = Vec<T>;

    // fn into_iter(&self) {

    // }

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }

        println!("{:?}", self.inner);

        let mut cursor = 1;

        let first = &self.inner[0];

        for value in &self.inner[1..] {
            if value == first {
                cursor += 1;
            } else {
                break;
            }
        }

        let items = self.inner.drain(0..cursor).collect();

        Some(items)
    }
}

pub fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];

    // println!("{:?}", Groups::new(data).into_iter().collect::<Vec<Vec<_>>>());
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // println!("{:?}", data2);
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}
