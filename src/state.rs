use rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use cpython::{Python, PyResult, PyErr};
use cpython::exc::TypeError;
use cpython::buffer::PyBuffer;

pub struct State {
    size: (usize, usize),
    spins: Vec<i32>,
}

#[allow(dead_code)]
impl State {
    // constructeur avec tous les spins up
    pub fn all_up(h: usize, w: usize) -> State {
        State {
            size: (h, w),
            spins: vec![1; w*h],
        }
    }

    // constructeur à partir d'un PyBuffer
    pub fn from_pybuffer(py: Python, buffer: &PyBuffer) -> PyResult<State> {
        if buffer.dimensions() != 2 {
            return Err(PyErr::new::<TypeError, _>(py, "Not rank 2"));;
        }
        Ok(State {
            size: (buffer.shape()[0], buffer.shape()[1]),
            spins: buffer.to_vec(py)?,
        })
    }

    pub fn copy_to_pybuffer(&self, py: Python, buffer: &PyBuffer) -> PyResult<()> {
        buffer.copy_from_slice(py, &self.spins)
    }

    pub fn compute_energy(&self) -> i32 {
        let mut energy = 0;
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                let s = self.spins[y * self.size.1 + x];
                let (x, y) = (x as isize, y as isize);
                let nei = self.get_spin(x + 1, y) + self.get_spin(x, y + 1) +
                          self.get_spin(x + 2 * (y % 2) - 1, y + 1);
                energy -= s * nei;
            }
        }
        energy
    }

    pub fn sweep(&mut self, temp: f64, n: Option<usize>) -> i32 {
        let mut delta_energy = 0;

        // energy_new - energy_old = -12, -8, -4, 0, 4, 8, 12
        let probas =
            [1.0, 1.0, 1.0, 1.0, (-4.0 / temp).exp(), (-8.0 / temp).exp(), (-12.0 / temp).exp()];

        for _ in 0..n.unwrap_or(self.spins.len()) {
            delta_energy += self.try_flip(&probas);
        }
        delta_energy
    }

    fn try_flip(&mut self, probas: &[f64]) -> i32 {
        let mut rng = rand::thread_rng();

        let x = Range::new(0isize, self.size.1 as isize).ind_sample(&mut rng);
        let y = Range::new(0isize, self.size.0 as isize).ind_sample(&mut rng);

        let x_ = x + 2 * (y % 2) - 1; // x of diagonal neighbor
        let nei = self.get_spin(x + 1, y) + self.get_spin(x - 1, y) + self.get_spin(x, y + 1) +
                  self.get_spin(x, y - 1) +
                  self.get_spin(x_, y + 1) + self.get_spin(x_, y - 1);

        // energy = - sum s_i s_j
        // d_e = energy_new - energy_old
        let d_e = 2 * self.get_spin(x, y) * nei;

        // nei = -6, -4, -2, 0, 2, 4, 6
        // d_e = -12, -8, ... 12
        // d_e / 4 = -3, -2, -1, 0, 1, 2, 3
        // d_e / 4 + 3 = 0, 1, 2, 3, 4, 5, 6

        if rng.next_f64() < probas[(d_e / 4 + 3) as usize] {
            let i: usize = y as usize * self.size.1 + x as usize;
            self.spins[i] = -self.spins[i];
            // energy_new = energy_old + d_e
            d_e
        } else {
            0
        }
    }

    fn get_spin(&self, x: isize, y: isize) -> i32 {
        let x: usize = (x + self.size.1 as isize) as usize % self.size.1;
        let y: usize = (y + self.size.0 as isize) as usize % self.size.0;
        self.spins[y * self.size.1 + x]
    }
}
