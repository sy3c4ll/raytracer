use crate::{Pixel, Rgb, Rgba};
use std::alloc::{Layout, alloc};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image<P: Pixel, const W: usize, const H: usize>(Box<[[P; W]; H]>);

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
    pub fn fill(px: P) -> Self {
        let layout = Layout::new::<[[P; W]; H]>();
        let ptr: *mut [[P; W]; H] = unsafe { alloc(layout) }.cast();
        let buf = unsafe { ptr.as_mut() }.expect("out of memory");
        for row in buf {
            for pixel in row {
                *pixel = px;
            }
        }
        Self(unsafe { Box::from_raw(ptr) })
    }
    pub fn fill_with(mut px: impl FnMut([usize; 2]) -> P) -> Self {
        let layout = Layout::new::<[[P; W]; H]>();
        let ptr: *mut [[P; W]; H] = unsafe { alloc(layout) }.cast();
        let buf = unsafe { ptr.as_mut() }.expect("out of memory");
        for (y, row) in buf.iter_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = px([x, y]);
            }
        }
        Self(unsafe { Box::from_raw(ptr) })
    }
    pub fn white() -> Self {
        Self::fill(P::white())
    }
    pub fn black() -> Self {
        Self::fill(P::black())
    }
    pub fn to_pbm_p1(&self) -> Vec<u8> {
        let mut buf = format!("P1\n{W} {H}\n").into_bytes();
        buf.reserve(W * H);
        for row in self {
            for pixel in row {
                buf.push(if pixel.to_bit() { b'1' } else { b'0' });
            }
        }
        buf
    }
    pub fn to_ppm_p6(&self) -> Vec<u8> {
        let mut buf = format!("P6\n{W} {H}\n255\n").into_bytes();
        buf.reserve(W * H * 3);
        for row in self {
            for pixel in row {
                let Rgb { r, g, b } = pixel.to_rgb();
                buf.extend([r, g, b]);
            }
        }
        buf
    }
    pub fn to_qoi(&self) -> Vec<u8> {
        #[inline]
        const fn hash(Rgba { r, g, b, a }: Rgba) -> u8 {
            r.wrapping_mul(3)
                .wrapping_add(g.wrapping_mul(5))
                .wrapping_add(b.wrapping_mul(7))
                .wrapping_add(a.wrapping_mul(11))
                & 0x3f
        }

        const QOI_OP_INDEX: u8 = 0x00; /* 00xxxxxx */
        const QOI_OP_DIFF: u8 = 0x40; /* 01xxxxxx */
        const QOI_OP_LUMA: u8 = 0x80; /* 10xxxxxx */
        const QOI_OP_RUN: u8 = 0xc0; /* 11xxxxxx */
        const QOI_OP_RGB: u8 = 0xfe; /* 11111110 */
        const QOI_OP_RGBA: u8 = 0xff; /* 11111111 */

        const QOI_MAGIC: [u8; 4] = *b"qoif";
        const QOI_HEADER_SIZE: usize = 14;
        const QOI_PADDING: [u8; 8] = *b"\0\0\0\0\0\0\0\x01";
        const QOI_PADDING_SIZE: usize = 8;

        let mut buf = Vec::with_capacity(QOI_HEADER_SIZE + W * H * 5 + QOI_PADDING_SIZE);

        buf.extend(QOI_MAGIC);
        buf.extend((W as u32).to_be_bytes());
        buf.extend((H as u32).to_be_bytes());
        buf.push(4);
        buf.push(1);

        let mut index = [Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }; 0x40];
        let mut run = 0;
        let mut px_prev = P::black();

        for row in self {
            for &px in row {
                if px == px_prev {
                    run += 1;
                    if run == 0x3e {
                        buf.push(QOI_OP_RUN | (run - 1));
                        run = 0;
                    }
                } else {
                    if run != 0 {
                        buf.push(QOI_OP_RUN | (run - 1));
                        run = 0;
                    }
                    let px = px.to_rgba();
                    let px_prev = px_prev.to_rgba();
                    let px_hash = hash(px.to_rgba());
                    if px == index[px_hash as usize] {
                        buf.push(QOI_OP_INDEX | px_hash);
                    } else {
                        index[px_hash as usize] = px;
                        if px.a != px_prev.a {
                            buf.extend([QOI_OP_RGBA, px.r, px.g, px.b, px.a]);
                        } else {
                            let var = Rgb {
                                r: px.r.wrapping_sub(px_prev.r),
                                g: px.g.wrapping_sub(px_prev.g),
                                b: px.b.wrapping_sub(px_prev.b),
                            };
                            let diff = Rgb {
                                r: var.r.wrapping_add(2),
                                g: var.g.wrapping_add(2),
                                b: var.b.wrapping_add(2),
                            };
                            let luma = Rgb {
                                r: var.r.wrapping_add(8).wrapping_sub(var.g),
                                g: var.g.wrapping_add(32),
                                b: var.b.wrapping_add(8).wrapping_sub(var.g),
                            };
                            if diff.r | diff.g | diff.b <= 0x03 {
                                buf.push(QOI_OP_DIFF | diff.r << 4 | diff.g << 2 | diff.b);
                            } else if luma.r | luma.b <= 0x0f && luma.g <= 0x3f {
                                buf.extend([QOI_OP_LUMA | luma.g, luma.r << 4 | luma.b]);
                            } else {
                                buf.extend([QOI_OP_RGB, px.r, px.g, px.b]);
                            }
                        }
                    }
                }
                px_prev = px;
            }
        }
        if run != 0 {
            buf.push(QOI_OP_RUN | (run - 1));
        }

        buf.extend(QOI_PADDING);

        buf
    }
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
