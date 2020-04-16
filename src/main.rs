use kintraj;
use gnuplot::{Figure, Caption, Color};
use clap::clap_app;

fn main() {

    let matches = clap_app!(myapp => 
        (@arg PLOT: -p --plot "Set to plot the signal")
        ).get_matches();
	let total_time: f64 = 1.0;
	let dt: f64 = 1e-5;
	let n: u64 = (total_time / dt) as u64;

	println!("Generating {} points", n);
	println!("Simulation time of {} with timestep of {}", total_time, dt);

	let mut res = kintraj::Timeseries::new();
	let sg = kintraj::SignalGenerator::new(0.3, 0.01, dt);

    println!("Expected free energy difference is {:.7} kT", sg.delta_f());
    
    for state in sg {
    	if res.length() >= n {break;}
    	res.push(res.length() as f64 * dt, state);
    }

    println!("Simulated free energy difference was {:.7} kT", res.delta_f());

    match matches.occurrences_of("PLOT") {
        0 => {},
        _ => {
            let mut figure = Figure::new();
            figure.axes2d()
            .lines(res.get_time(), res.get_state(), &[Color("black")]);
            figure.show(); 
        }
    } 
}