use rand::prelude::Rng;

pub struct Timeseries {
	time : Vec<f64>,
	state: Vec<i8>,
}

impl Timeseries {
	pub fn new() -> Timeseries {
		Timeseries {
			time : Vec::new(),
			state: Vec::new(),
		}
	}

	pub fn push(&mut self, t: f64, state: i8) -> () {
		self.time.push(t);
		self.state.push(state);
	}

	pub fn get_time(&self) -> &Vec<f64> {
		&self.time
	}

	pub fn get_state(&self) -> &Vec<i8> {
		&self.state
	}

	pub fn delta_f(&self) -> f64 {
		let mut n0: i64 = 0;
		let mut n1: i64 = 0;
		for i in self.state.iter() {
			match i {
				0 => n0 += 1,
				1 => n1 += 1,
				_ => panic!("Found an invalid state!"),
			};
		}
		(n0 as f64 / n1 as f64).ln()
	}
}

pub struct SignalGenerator {
	traj : Trajectory,
	pub delta_t : f64,
}

impl SignalGenerator {
	pub fn new(k01: f64, k10: f64, delta_t: f64) -> SignalGenerator {
		SignalGenerator {
			traj : Trajectory::new(k01, k10),
			delta_t,
		}
	}

	pub fn delta_f(&self) -> f64 {
		self.traj.delta_f()
	}
}

impl Iterator for SignalGenerator {
	type Item = i8;

	fn next(&mut self) -> Option<i8> {
		let mut rng = rand::thread_rng();
		let constant = match self.traj.signal {
			0 => self.traj.k01,
			_   => self.traj.k10,
		};
		self.traj.time += self.delta_t;
		let prob = prob_trans(constant, self.traj.time);
		let accept_prob: f64 = rng.gen();
		if accept_prob > prob {self.traj.swap_signal();}
		Some(self.traj.signal)
	}
}

pub struct Trajectory {
	time  : f64,
	k01	  : f64,
	k10   : f64,
	signal : i8,
}

impl Trajectory {
	pub fn new(k01: f64, k10: f64) -> Trajectory {
		Trajectory {
			time : 0.0,
			k01,
			k10,
			signal : 0,
		}
	}

	pub fn delta_f(self: &Self) -> f64 {
		(self.k10 / self.k01).ln()
	}

	fn swap_signal(self: &mut Self) -> () {
		self.time = 0.0;
		self.signal = match self.signal {
			0 => 1,
			1 => 0,
			_ => panic!("Found an invalid state!")
		};
	}

}

pub fn prob_trans(constant: f64, time: f64) -> f64 {
	1.0 - (-constant * time).exp()
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn time_zero() {
		assert_eq!(prob_trans(1.0, 0.0), 0.0);
	}

	#[test]
	fn equal_rates() {
		let traj = Trajectory::new(5.0, 5.0);
		assert_eq!(traj.delta_f(), 0.0);
	}
}