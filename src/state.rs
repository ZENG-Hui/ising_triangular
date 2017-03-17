use rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use cpython::{Python, PyResult, PyErr};
use cpython::exc::TypeError;
use cpython::buffer::PyBuffer;

pub struct State {
    size: (usize, usize),
    spins: Vec<i32>,
    energy: i32,
}

#[allow(dead_code)]
impl State {
    // constructeur avec tous les spins up
    pub fn all_up(h: usize, w: usize) -> State {
        State {
            size: (h, w),
            spins: vec![1; w*h],
            energy: -3 * (w * h) as i32,
        }
    }

    // constructeur à partir d'un PyBuffer
    pub fn from_pybuffer(py: Python, buffer: &PyBuffer) -> PyResult<State> {
        if buffer.dimensions() != 2 {
            return Err(PyErr::new::<TypeError, _>(py, "Not rank 2"));;
        }
        let mut state = State {
            size: (buffer.shape()[0], buffer.shape()[1]),
            spins: buffer.to_vec(py)?,
            energy: 0, // valeur invalide
        };
        state.compute_energy(); // corrige l'énergie
        Ok(state)
    }

    pub fn copy_to_pybuffer(&self, py: Python, buffer: &PyBuffer) -> PyResult<()> {
        buffer.copy_from_slice(py, &self.spins)
    }

    fn compute_energy(&mut self) {
        self.energy = 0;
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                let s = self.spins[y * self.size.1 + x];
                let (x, y) = (x as isize, y as isize);
                let nei = self.get_spin(x + 1, y) + self.get_spin(x, y + 1) +
                          self.get_spin(x + 2 * (y % 2) - 1, y + 1);
                self.energy -= s * nei;
            }
        }
    }

    fn get_spin(&self, x: isize, y: isize) -> i32 {
        let x = (x + self.size.1 as isize) as usize % self.size.1;
        let y = (y + self.size.0 as isize) as usize % self.size.0;
        self.spins[y * self.size.1 + x]
    }

    pub fn sweep(&mut self, temp: f64) -> f64 {
        let mut counter = 0;

        // energy_new - energy_old = -12, -8, -4, 0, 4, 8, 12
        let probas =
            [1.0, 1.0, 1.0, 1.0, (-4.0 / temp).exp(), (-8.0 / temp).exp(), (-12.0 / temp).exp()];


        for _ in 0..self.spins.len() {
            counter += self.try_flip(&probas);
        }
        counter as f64 / self.spins.len() as f64
    }

    fn try_flip(&mut self, probas: &[f64]) -> usize {
        let mut rng = rand::thread_rng();

        let x = Range::new(0isize, self.size.1 as isize).ind_sample(&mut rng);
        let y = Range::new(0isize, self.size.0 as isize).ind_sample(&mut rng);

        let nei = self.get_spin(x + 1, y) + self.get_spin(x - 1, y) + self.get_spin(x, y + 1) +
                  self.get_spin(x, y - 1) +
                  self.get_spin(x + 2 * (y % 2) - 1, y + 1) +
                  self.get_spin(x + 2 * (y % 2) - 1, y - 1);

        // energy = - sum s_i s_j
        // d_e = energy_new - energy_old
        let d_e = 2 * self.get_spin(x, y) * nei;

        // nei = -6, -4, -2, 0, 2, 4, 6
        // d_e = -12, -8, ... 12
        // d_e / 4 = -3, -2, -1, 0, 1, 2, 3
        // d_e / 4 + 3 = 0, 1, 2, 3, 4, 5, 6

        if rng.next_f64() < probas[(d_e / 4 + 3) as usize] {
            let i = y as usize * self.size.1 + x as usize;
            self.spins[i] = -self.spins[i];
            // energy_new = energy_old + d_e
            self.energy += d_e;
            1
        } else {
            0
        }
    }

    pub fn try_swap(s1: &mut State, temp1: f64, s2: &mut State, temp2: f64) -> bool {
        let exponent = (s1.energy - s2.energy) as f64 * (1.0 / temp1 - 1.0 / temp2);
        rand::thread_rng().next_f64() < exponent.exp()
    }

    pub fn get_energy(&self) -> i32 {
        self.energy
    }
}
