extern crate rand;

#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python, PyObject};
use cpython::buffer::PyBuffer;

mod state;
use state::State;

// defines the python module
py_module_initializer!(ising_triangular,
                       initising_triangular,
                       PyInit_ising_triangular,
                       |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py,
             "sweep",
             py_fn!(py,
                    sweep(numpy: &PyObject, temp: f64, n_flips: Option<usize> = None)))?;
    m.add(py, "energy", py_fn!(py, energy(numpy: &PyObject)))?;
    Ok(())
});

fn sweep(py: Python, numpy: &PyObject, temp: f64, n_flips: Option<usize>) -> PyResult<i32> {
    let buffer = PyBuffer::get(py, numpy)?;

    let mut state = State::from_pybuffer(py, &buffer)?;

    let mut delta_energy = 0;
    py.allow_threads(|| { delta_energy = state.sweep(temp, n_flips); });

    state.copy_to_pybuffer(py, &buffer)?;
    Ok(delta_energy)
}

fn energy(py: Python, numpy: &PyObject) -> PyResult<i32> {
    let buffer = PyBuffer::get(py, numpy)?;

    let state = State::from_pybuffer(py, &buffer)?;
    Ok(state.compute_energy())
}
