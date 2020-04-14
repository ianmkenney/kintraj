use kintraj;
use gnuplot::{Figure, Caption, Color};

fn main() {
	let total_time: f64 = 100.0;
	let dt: f64 = 0.001;
	let n: u64 = (total_time / dt) as u64;

	println!("Generating {} points", n);
	println!("Simulation time of {} with timestep of {}", total_time, dt);

	let mut res = kintraj::Timeseries::new();
	let mut counter = 0;
    let sg = kintraj::SignalGenerator::new(1.0, 1.0, dt);

    println!("Expected free energy difference is {:.7} kT", sg.delta_f());
    for state in sg {
    	if counter >= n {break;}
    	res.push(counter as f64 * dt, state);
    	counter += 1;
    }

    println!("Simulated free energy difference was {:.7} kT", res.delta_f());

	let mut figure = Figure::new();
    figure.axes2d()
    .lines(res.get_time(), res.get_state(), &[Color("black")]);
    figure.show();
}