extern crate matrix;

use crate::matrix::tests::*;
// use crate::matrix::Matrix;
// use crate::matrix::Vector;

fn main() {
    // Create a new vector
    // let mut vector1 = Vector::new(vec![1.0, 2.0, 3.0]);
    // let vector2 = Vector::from(&[4.0, 5.0, 6.0]);
    // // Add the vectors
    // vector1.add(&vector2);
    // vector1.print();

    // let matrix1 = Matrix::from([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [6.0, 7.0, 8.0]]);
    // matrix1.print();

    // let matrix2 = Matrix::from([[1.0, 2.0, 3.0]]);
    // matrix2.print();

    // let matrix3 = Matrix::from([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0]]);
    // matrix3.print();

    // row_echelon_test();
    rank_test();

    let display = glium::glutin::WindowBuilder::new()
        .with_title("OpenGL with Rust")
        .build_glium()
        .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => {}
            }
        }
    }
}
