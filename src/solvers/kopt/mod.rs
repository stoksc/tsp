use std::borrow::Borrow;
use std::time;

use rand::Rng;

use crate::common::Path;
use crate::metrizable::Metrizable;

impl<T: Metrizable + Clone + Borrow<T>> Path<T> {
    pub fn solve_kopt(&mut self, timeout: time::Duration) {
        let max_iter = self.path.len();
        let mut iter_without_impr = 0;
        let mut k = 2;
        let start_time = time::Instant::now();
        loop {
            match k_opt(k, self) {
                Some(_) => {
                    iter_without_impr = 0;
                    k = 2;
                }
                None => {
                    iter_without_impr += 1;
                    if iter_without_impr > max_iter {
                        k = 3;
                        iter_without_impr = 0;
                    }
                }
            }
            if start_time.elapsed() > timeout {
                break;
            }
        }
    }
}

pub fn k_opt<T>(k: usize, path: &mut Path<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    match k {
        2 => {
            let mut i = rand_index(path);
            let mut j = rand_index(path);

            if i == j {
                return None;
            }

            let mut ij = vec![i, j];
            ij.sort();
            i = ij[0];
            j = ij[1];

            two_opt(i, j, path)
        }
        3 => {
            let mut i = rand_index(path);
            let mut j = rand_index(path);
            let mut k = rand_index(path);

            if i == j || j == k {
                return None;
            }

            let mut ijk = vec![i, j, k];
            ijk.sort();
            i = ijk[0];
            j = ijk[1];
            k = ijk[2];

            three_opt(i, j, k, path)
        }
        _ => panic!("Not implemented"),
    }
}

#[inline]
pub fn two_opt<T>(i: usize, j: usize, path: &mut Path<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    let mut new_path = Vec::from(&path.path[..i]);
    let mut middle = Vec::from(&path.path[i..j]);
    middle.reverse();
    new_path.append(&mut middle);
    new_path.append(&mut Vec::from(&path.path[j..]));

    let new_path = Path { path: new_path };
    let prev_len = path.path_len();
    let post_len = new_path.path_len();

    if post_len < prev_len {
        path.path = new_path.path;
        Some(post_len - prev_len)
    } else {
        None
    }
}

#[inline]
pub fn three_opt<T>(i: usize, j: usize, k: usize, path: &mut Path<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    let a = &path.path[i % path.path.len()];
    let b = &path.path[(i + 1) % path.path.len()];
    let c = &path.path[j % path.path.len()];
    let d = &path.path[(j + 1) % path.path.len()];
    let e = &path.path[k % path.path.len()];
    let f = &path.path[(k + 1) % path.path.len()];

    let d0 = a.distance(&b) + c.distance(&d) + e.distance(&f);
    let d1 = a.distance(&c) + b.distance(&d) + e.distance(&f);
    let d2 = a.distance(&b) + c.distance(&e) + d.distance(&f);
    let d3 = a.distance(&d) + e.distance(&b) + c.distance(&f);
    let d4 = f.distance(&b) + c.distance(&d) + e.distance(&a);

    if d0 > d1 {
        let mut new_path = Vec::from(&path.path[..i]);
        let mut middle = Vec::from(&path.path[i..j]);
        middle.reverse();
        new_path.append(&mut middle);
        new_path.append(&mut Vec::from(&path.path[j..]));
        Some(-d0 + d1)
    } else if d0 > d2 {
        let mut new_path = Vec::from(&path.path[..j]);
        let mut middle = Vec::from(&path.path[j..k]);
        middle.reverse();
        new_path.append(&mut middle);
        new_path.append(&mut Vec::from(&path.path[k..]));
        Some(-d0 + d2)
    } else if d0 > d4 {
        let mut new_path = Vec::from(&path.path[..i]);
        let mut middle = Vec::from(&path.path[i..k]);
        middle.reverse();
        new_path.append(&mut middle);
        new_path.append(&mut Vec::from(&path.path[k..]));
        Some(-d0 + d4)
    } else if d0 > d3 {
        let mut new_path = Vec::from(&path.path[..i]);
        new_path.append(&mut Vec::from(&path.path[j..k]));
        new_path.append(&mut Vec::from(&path.path[i..j]));
        new_path.append(&mut Vec::from(&path.path[k..]));
        Some(-d0 + d3)
    } else {
        None
    }
}

pub fn rand_index<T>(path: &Path<T>) -> usize
where
    T: Metrizable,
{
    rand::thread_rng().gen_range(0, path.path.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Path;
    use crate::point::Point;

    #[test]
    fn test_two_opt() {
        let mut path = Path::from(&vec![
            Point::new(0., 0.),
            Point::new(1., 1.),
            Point::new(1., 0.),
            Point::new(0., 1.),
        ]);

        let two_opt_path = Path::from(&vec![
            Point::new(0., 0.),
            Point::new(1., 0.),
            Point::new(1., 1.),
            Point::new(0., 1.),
        ]);

        let result = two_opt(1, 3, &mut path);

        assert_ne!(None, result);
        assert_eq!(path, two_opt_path);
    }
}
