extern crate rmdb;
extern crate time;

use rmdb::storage::*;
use rmdb::measurement::*;
use rmdb::utils::*;
use time::*;

fn main() {
	bench_write_one_thread_32768_sensors_32768_measurements();

	assert_eq!(Some(8), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 7, 8, 9, 11], 10));

	let values = [4, 4, 4, 4, 4];
	let s = 3;
	get_slice_index_which_is_greater_or_equal(&values, s);

	// let values = [1, 2, 3, 3, 3, 3, 3];
	// let s = 3;
	// get_righter_greater_or_equal(&values, s);

	// let values = [1, 2, 3, 5, 6, 7, 8];
	// let s = 4;
	// get_righter_greater_or_equal(&values, s);

	// let values = [1, 2, 3, 5, 6, 7, 8];
	// let s = 9;
	// get_righter_greater_or_equal(&values, s);
}
fn bench_write_one_thread_32768_sensors_32768_measurements(){
	let time_before = precise_time_ns();

	const NUMBER_OF_SENSORS: u64 = 32768;
	let storage = DummyStorage::new(NUMBER_OF_SENSORS);
	for i in 0..NUMBER_OF_SENSORS{
		let m = Measurement::new(i, i, i as f64, 1, 1);
		storage.write(m);
	}
	let time_after = precise_time_ns();
	let diff = time_after - time_before;
	println!("bench_write_one_thread_32768_measurements: {} ms", (diff as f64) / (1000000.0f64));
}
fn bench_write_16_threads_32768_sensors_1048576_measurements(){
	const NUMBER_OF_SENSORS: u64 = 20;
	let ranges = get_equable_ranges(NUMBER_OF_SENSORS, 2);

	let time_before = precise_time_ns();

	
	let storage = DummyStorage::new(NUMBER_OF_SENSORS);
	for i in 0..NUMBER_OF_SENSORS{
		let m = Measurement::new(i, i, i as f64, 1, 1);
		storage.write(m);
	}
	let time_after = precise_time_ns();
	let diff = time_after - time_before;
	println!("bench_write_one_thread_32768_measurements: {} ms", (diff as f64) / (1000000.0f64));
}

