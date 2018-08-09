extern crate num_traits;
extern crate rand;

use num_traits::NumCast;
use rand::distributions::{Distribution, Normal};
use std::marker::PhantomData;

fn vec_add<T>(x: &mut [T], y: &[T])
where
    T: num_traits::Float,
{
    assert_eq!(x.len(), y.len());
    for (u, v) in x.iter_mut().zip(y.iter()) {
        *u = *u + *v;
    }
}

fn vec_sub<T>(x: &mut [T], y: &[T])
where
    T: num_traits::Float,
{
    assert_eq!(x.len(), y.len());
    for (u, v) in x.iter_mut().zip(y.iter()) {
        *u = *u - *v;
    }
}

fn vec_square_ew<T>(x: &mut [T])
where
    T: num_traits::Float,
{
    for u in x.iter_mut() {
        *u = *u * *u;
    }
}

fn vec_scalar_mul<T>(x: &mut [T], k: T)
where
    T: num_traits::Float,
{
    for u in x.iter_mut() {
        *u = *u * k;
    }
}

fn vec_scalar_div<T>(x: &mut [T], k: T)
where
    T: num_traits::Float,
{
    for u in x.iter_mut() {
        *u = *u / k;
    }
}

fn mean_variance<T, S: AsRef<[T]>>(data: &[S]) -> (Vec<T>, Vec<T>)
where
    T: num_traits::Float + ::std::fmt::Debug,
    S: ::std::fmt::Debug,
{
    let n: T = num_traits::NumCast::from(data.len()).unwrap();
    let mut mean = vec![T::zero(); data[0].as_ref().len()];

    for x in data.iter() {
        vec_add(&mut mean, x.as_ref());
    }
    vec_scalar_div(&mut mean, n);

    let mean = mean;
    let mut var = vec![T::zero(); mean.len()];

    for x in data.iter() {
        let mut x: Vec<T> = x.as_ref().iter().cloned().collect();
        vec_sub(&mut x, &mean);
        vec_square_ew(&mut x);
        vec_add(&mut var, &x);
    }
    vec_scalar_div(&mut var, n - T::one());
    let var = var;

    (mean, var)
}

#[derive(Debug)]
struct NoiseSource<T> {
    dists: Vec<Normal>,
    rng: rand::ThreadRng,
    _marker: PhantomData<fn() -> T>,
}

impl<T> NoiseSource<T>
where
    T: num_traits::Float,
{
    fn new(var: &[T]) -> Self {
        let dists = var.iter()
            .map(|&v| Normal::new(0.0, NumCast::from(v).unwrap()))
            .collect();
        NoiseSource {
            dists: dists,
            rng: rand::thread_rng(),
            _marker: PhantomData,
        }
    }

    fn sample(&mut self) -> Vec<T> {
        let mut noises = Vec::with_capacity(self.dists.len());
        for i in 0..self.dists.len() {
            noises.push(NumCast::from(self.dists[i].sample(&mut self.rng)).unwrap());
        }
        noises
    }
}

pub fn add_noise<T, S>(data: &mut [S], ratio: T)
where
    S: ::std::fmt::Debug + AsRef<[T]> + AsMut<[T]>,
    T: ::std::fmt::Debug + num_traits::Float,
{
    let (_mean, mut var) = mean_variance(data);
    vec_scalar_mul(&mut var[..], ratio);

    let mut src = NoiseSource::new(&var[..]);
    for row in data.as_mut().iter_mut() {
        let noise = src.sample();
        vec_add(row.as_mut(), &noise[..]);
    }
}
