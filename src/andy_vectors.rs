use core::ops::Index;
use core::ops::IndexMut;

pub struct Vec2D<T> {
    pub data: Vec<T>,
    pub h: usize,
    pub w: usize,
}

impl<T> Vec2D<T> {
    pub fn new(data: Vec<T>, w: usize, h: usize) -> Vec2D<T> {
        assert!(data.len() == (w * h), "data dimentions mismatch");
        Vec2D { data, w, h }
    }
    fn is_in_range(&self, index: (usize, usize)) -> bool {
        index.0 < self.w && index.1 < self.h
    }
    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        (x >= 0 && x < self.w.try_into().unwrap()) && (y >= 0 && y < self.h.try_into().unwrap())
    }

    fn get_index(&self, index: (usize, usize)) -> Option<usize> {
        if self.is_in_range(index) {
            Some(index.0 + (self.w * index.1))
        } else {
            None
        }
    }
    /*
    pub fn get_row(&self, y: usize) -> &[T]{
        let start: usize = self.get_index(0, y).unwrap();
        let end:   usize = start + self.w;
        return &self.data[start..end];
    }
    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]>{
        return (0..self.h)
            .map(|y| self.get_row(y))
            .into_iter();
    }
    */
}
impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let i = self.get_index(index).unwrap();
        &self.data[i]
    }
}
impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let i = self.get_index(index).unwrap();
        &mut self.data[i]
    }
}
