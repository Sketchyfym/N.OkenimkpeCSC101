	fn main() {
		let ta:f64 = 450_000.00;
		let tq:f64 = 2.0;
		let ma:f64 = 1_500_000.00;
		let mq:f64 = 1.0;
		let ha:f64 = 750_000.00;
		let hq:f64 = 3.0;
		let da:f64 = 2_850_000.00;
		let dq:f64 = 3.0;
		let aa:f64 = 250_000.00;
		let aq:f64 = 1.0;


		// sum
		let sum:f64 = (ta * tq) + (ma * mq) + (ha * hq) + (da * dq) + (aa * aq);
		println!("Sum = {}", sum);

		// average
		let average:f64 = (ta + ma + ha + da + aa) / (tq + mq + hq + dq + aq);
		println!("Average = {}", average);
 	}

	