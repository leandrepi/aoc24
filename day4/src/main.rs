use std::fs;

#[derive(Debug)]
pub struct CharArray {
    contents: Vec<u8>,
    width: usize,
    height: usize,
}

pub struct Kernel<T>
where
    T: Clone,
{
    data: Vec<T>,
    w: usize,
    h: usize,
}

fn matched<'a, T: 'a, I>(fst: I, snd: I) -> bool
where
    I: IntoIterator<Item = &'a T>,
    T: PartialEq,
{
    fst.into_iter().zip(snd).all(|(&ref p, &ref c)| p == c)
}

impl<T> Kernel<T>
where
    T: Clone + PartialEq,
{
    fn matches(&self, patterns: &[&[T]]) -> bool {
        patterns
            .iter()
            .any(|pat| matched(self.data.iter(), pat.iter()))
    }

    fn diag_matches(&self, patterns: &[&[T]]) -> u32 {
        assert!(self.h == self.w);
        let diag: Vec<T> = self
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| i % self.h == i / self.h)
            .map(|(_, c)| c.clone())
            .collect();
        let diag_match = patterns.iter().any(|pat| matched(diag.iter(), pat.iter())) as u32;
        let diag: Vec<T> = self
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| self.h - 1 - (i % self.h) == i / self.h)
            .map(|(_, c)| c.clone())
            .collect();
        let anti_match = patterns.iter().any(|pat| matched(diag.iter(), pat.iter())) as u32;
        diag_match + anti_match
    }
}

pub struct KernelConfig {
    kw: usize,
    kh: usize,
    stride: usize,
}

pub struct KernelIterator<'a, 'b, T>
where
    T: Clone,
{
    data: &'a [T],
    w: usize,
    h: usize,
    cursor: usize,
    config: &'b KernelConfig,
}

impl<'a, 'b, T> Iterator for KernelIterator<'a, 'b, T>
where
    T: Clone,
{
    type Item = Kernel<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let col = self.cursor % self.w;
        let config = self.config;
        if col + config.kw > self.w {
            self.cursor = self.cursor - col + self.h;
        }
        let row = self.cursor / self.w;
        let col = self.cursor % self.w;
        if row + config.kh > self.h || config.kw > self.w {
            return None;
        }
        let mut kernel_buf = Vec::with_capacity(config.kw * config.kh);
        for ki in row..(row + config.kh) {
            let start = ki * self.w + col;
            kernel_buf.extend_from_slice(&self.data[start..(start + config.kw)]);
        }
        self.cursor += config.stride;
        Some(Kernel {
            data: kernel_buf,
            w: config.kw,
            h: config.kh,
        })
    }
}

impl CharArray {
    fn from(raw: &str) -> Self {
        let mut lines = raw.lines().map(|l| l.trim()).filter(|l| l.len() > 0);
        let first = lines
            .next()
            .expect("Should have at least a non-empty line.");
        let mut contents = first.bytes().collect::<Vec<u8>>();
        let width = first.len();
        let mut height = 1;
        for line in lines {
            contents.extend(line.bytes());
            height += 1;
        }
        Self {
            contents,
            width,
            height,
        }
    }

    fn kernels<'a, 'b>(&'a self, config: &'b KernelConfig) -> KernelIterator<'a, 'b, u8> {
        KernelIterator {
            data: &self.contents,
            w: self.width,
            h: self.height,
            cursor: 0,
            config,
        }
    }

    fn count_part1(&self, pattern: &str) -> u32 {
        let pat_len = pattern.len();
        let rev = pattern.bytes().rev().collect::<Vec<u8>>();
        let pat = pattern.bytes().collect::<Vec<u8>>();
        let patterns = vec![pat.as_slice(), rev.as_slice()];
        let mut matches = self
            .kernels(&KernelConfig {
                kw: pat_len,
                kh: 1,
                stride: 1,
            })
            .filter(|k| k.matches(&patterns))
            .count() as u32;
        matches += self
            .kernels(&KernelConfig {
                kw: 1,
                kh: pat_len,
                stride: 1,
            })
            .filter(|k| k.matches(&patterns))
            .count() as u32;
        matches += self
            .kernels(&KernelConfig {
                kw: pat_len,
                kh: pat_len,
                stride: 1,
            })
            .map(|k| k.diag_matches(&patterns))
            .sum::<u32>();
        matches
    }

    fn count_part2(&self, pattern: &str) -> u32 {
        let pat_len = pattern.len();
        let rev = pattern.bytes().rev().collect::<Vec<u8>>();
        let pat = pattern.bytes().collect::<Vec<u8>>();
        let patterns = vec![pat.as_slice(), rev.as_slice()];
        self.kernels(&KernelConfig {
            kw: pat_len,
            kh: pat_len,
            stride: 1,
        })
        .map(|k| k.diag_matches(&patterns))
        .filter(|&m| m == 2)
        .count() as u32
    }
}

impl<Idx> std::ops::Index<Idx> for CharArray
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.contents[index]
    }
}

impl<Idx> std::ops::IndexMut<Idx> for CharArray
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.contents[index]
    }
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let processed = CharArray::from(&raw);
    let result_part1 = processed.count_part1("XMAS");
    println!("Day 5, part 1: {result_part1}");
    let result_part2 = processed.count_part2("MAS");
    println!("Day 5, part 1: {result_part2}");
}
