use core::ops::Index;

pub struct Vec2D<T>{
    pub data: Vec<T>,
    pub h: usize,
    pub w: usize,
}

impl<T> Vec2D<T>{
    pub fn new(data: Vec<T>, w: usize, h: usize) -> Vec2D<T>{
        assert!(data.len() == ((w*h) as usize), "data dimentions mismatch");
        return Vec2D{
            data: data,
            w: w,
            h: h,
        }
    }
    fn get_index(&self, x: usize, y: usize) -> Option<usize>{
        if x < self.w && y < self.h {
            return Some(x + (self.w * y));
        }else{
            return None;
        }
    }
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
}
impl<T> Index<(usize, usize)> for Vec2D<T>{
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output{
        return &self.data[self.get_index(index.0, index.1).unwrap()];
    }
}
