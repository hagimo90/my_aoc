use num::complex::Complex;

fn mendelbrot_at_point(cx:f64,cy:f64,iters:usize) -> usize{

    let mut z = Complex::new(0.0,0.0);
    let c = Complex::new(cx,cy);
    for i  in 0..=iters {
        if z.norm()> 2.0 {
            return i;
        }
        z = z*z +c;
    }
    return iters;
}
fn main() {
    println!("Hello, world!");
}
