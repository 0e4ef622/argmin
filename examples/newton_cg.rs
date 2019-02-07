// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate argmin;
extern crate ndarray;
use argmin::prelude::*;
use argmin::solver::newton::NewtonCG;
use argmin::testfunctions::{rosenbrock_2d, rosenbrock_2d_derivative, rosenbrock_2d_hessian};
use ndarray::{Array, Array1, Array2};

#[derive(Clone)]
struct Rosenbrock {
    a: f64,
    b: f64,
}

impl ArgminOperator for Rosenbrock {
    type Parameters = Array1<f64>;
    type OperatorOutput = f64;
    type Hessian = Array2<f64>;

    fn apply(&self, p: &Self::Parameters) -> Result<Self::OperatorOutput, Error> {
        Ok(rosenbrock_2d(&p.to_vec(), self.a, self.b))
    }

    fn gradient(&self, p: &Self::Parameters) -> Result<Self::Parameters, Error> {
        Ok(Array1::from_vec(rosenbrock_2d_derivative(
            &p.to_vec(),
            self.a,
            self.b,
        )))
    }

    fn hessian(&self, p: &Self::Parameters) -> Result<Self::Hessian, Error> {
        let h = rosenbrock_2d_hessian(&p.to_vec(), self.a, self.b);
        Ok(Array::from_shape_vec((2, 2), h)?)
    }
}

fn run() -> Result<(), Error> {
    // Define cost function
    let cost = Rosenbrock { a: 1.0, b: 100.0 };

    // Define initial parameter vector
    // let init_param: Array1<f64> = Array1::from_vec(vec![1.2, 1.2]);
    let init_param: Array1<f64> = Array1::from_vec(vec![-1.2, 1.0]);

    // Set up solver
    let mut solver = NewtonCG::new(cost, init_param);

    // Set maximum number of iterations
    solver.set_max_iters(80);

    // Attach a logger
    solver.add_logger(ArgminSlogLogger::term());

    // Run solver
    solver.run()?;

    // Wait a second (lets the logger flush everything before printing again)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Print result
    println!("{}", solver.result());
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{} {}", e.as_fail(), e.backtrace());
        std::process::exit(1);
    }
}
