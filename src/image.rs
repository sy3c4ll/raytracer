use crate::{Pixel, Rgb};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Image<P: Pixel, const W: usize, const H: usize>([[P; W]; H]);

impl<P: Copy + Default + Pixel, const W: usize, const H: usize> Default for Image<P, W, H> {
    fn default() -> Self {
        Self([[Default::default(); W]; H])
    }
}

impl<P: Pixel, const W: usize, const H: usize> Index<[usize; 2]> for Image<P, W, H> {
    type Output = P;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[index[1]][index[0]]
    }
}

impl<P: Pixel, const W: usize, const H: usize> IndexMut<[usize; 2]> for Image<P, W, H> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[index[1]][index[0]]
    }
}

impl<P: Pixel, const W: usize, const H: usize> IntoIterator for Image<P, W, H> {
    type Item = [P; W];
    type IntoIter = std::array::IntoIter<[P; W], H>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, P: Pixel, const W: usize, const H: usize> IntoIterator for &'a Image<P, W, H> {
    type Item = &'a [P; W];
    type IntoIter = std::slice::Iter<'a, [P; W]>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, P: Pixel, const W: usize, const H: usize> IntoIterator for &'a mut Image<P, W, H> {
    type Item = &'a mut [P; W];
    type IntoIter = std::slice::IterMut<'a, [P; W]>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<P: Pixel, const W: usize, const H: usize> Image<P, W, H> {
    pub fn to_pbm_p1(&self) -> Vec<u8> {
        let mut buf = format!("P1\n{W} {H}\n").into_bytes();
        buf.reserve(W * H);
        for row in self {
            for pixel in row {
                buf.push(if pixel.to_bit() { b'1' } else { b'0' });
            }
        }
        buf.push(b'\n');
        buf
    }
    pub fn to_ppm_p6(&self) -> Vec<u8> {
        let mut buf = format!("P6\n{W} {H}\n255\n").into_bytes();
        buf.reserve(W * H * 3);
        for row in self {
            for pixel in row {
                let Rgb { r, g, b } = pixel.to_rgb();
                buf.push(r);
                buf.push(g);
                buf.push(b);
            }
        }
        buf.push(b'\n');
        buf
    }
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
