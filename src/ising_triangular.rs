extern crate rand;

#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python, PyObject};
use cpython::buffer::PyBuffer;

mod state;
use state::State;

py_module_initializer!(ising_triangular,
                       initising_triangular,
                       PyInit_ising_triangular,
                       |py, m| {
                           m.add(py, "__doc__", "This module is implemented in Rust.")?;
                           m.add(py, "sweep", py_fn!(py, sweep(numpy: &PyObject, temp: f64)))?;
                           m.add(py, "energy", py_fn!(py, energy(numpy: &PyObject)))?;
                           Ok(())
                       });

fn sweep(py: Python, numpy: &PyObject, temp: f64) -> PyResult<f64> {
    let buffer = PyBuffer::get(py, numpy)?;

    let mut state = State::from_pybuffer(py, &buffer)?;

    let accept_ratio = state.sweep(temp);

    state.copy_to_pybuffer(py, &buffer)?;
    Ok(accept_ratio)
}

fn energy(py: Python, numpy: &PyObject) -> PyResult<i32> {
    let buffer = PyBuffer::get(py, numpy)?;

    let state = State::from_pybuffer(py, &buffer)?;
    Ok(state.get_energy())
}
