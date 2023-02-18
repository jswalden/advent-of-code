use std::ops::{Index, IndexMut};

pub struct Matrix2D<T>
where
    T: Default + Clone,
{
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Matrix2D<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Matrix2D<T> {
        Matrix2D {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix2D<T>
where
    T: Clone + Default,
{
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix2D<T>
where
    T: Clone + Default,
{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.data[x + y * self.width]
    }
}
