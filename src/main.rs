use kintraj;
use gnuplot::{Figure, Caption, Color};

fn main() {
	let total_time: f64 = 100.0;
	let dt: f64 = 1e-5;
	let n: u64 = (total_time / dt) as u64;

	println!("Generating {} points", n);
	println!("Simulation time of {} with timestep of {}", total_time, dt);

	let mut res = kintraj::Timeseries::new();
	let sg = kintraj::SignalGenerator::new(0.0004, 0.0009, dt);

    println!("Expected free energy difference is {:.7} kT", sg.delta_f());
    
    for state in sg {
    	if res.length() >= n {break;}
    	res.push(res.length() as f64 * dt, state);
    }

    println!("Simulated free energy difference was {:.7} kT", res.delta_f());

    let mut figure = Figure::new();
    figure.axes2d()
    .lines(res.get_time(), res.get_state(), &[Color("black")]);
    figure.show();
}