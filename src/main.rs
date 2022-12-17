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

fn calculate_mandelbrot(iters:usize,x_min:f64,x_max:f64,y_min:f64,y_max:f64,width:usize,height:usize)-> Vec<Vec<usize>> {

    let set_height = y_max - y_min;
    let set_width = x_max - x_min;
    let mut rows = Vec::new();

    for img_y in 0..height {
        let mut row = Vec::new();
        for img_x in 0..width {
            let width_ = width as f64;
            let height_ = height as f64;
            let img_y_ = img_y as f64;
            let img_x_ = img_x as f64;

            let cx= x_min + set_width*(img_x_ /width_ );
            let cy = y_min + set_height*(img_y_ /height_ );
            let escaped_at = mendelbrot_at_point(cx,cy,iters);
            row.push(escaped_at);
         
    }
    rows.push(row);
}


    rows
}

fn render_mandelbrot (escape_vals :Vec<Vec<usize>> ) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => ':',
                11..=30 => '*',
                30..=100 => 'x',
                100..=200 => '%',
                200..=400 => '$',
                400..=700 => '#',
                _ => '@',
            };
            line.push(val);

            
        }
        println!("{}",line);
    }
}
fn main() {
    let mandelbrot = calculate_mandelbrot(1000,-2.5,1.0,-1.1,1.1,100,30);
    render_mandelbrot(mandelbrot );
}
