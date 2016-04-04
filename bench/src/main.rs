extern crate rmdb;
extern crate time;

use rmdb::*;
use rmdb::bplustree::*;
use rmdb::storage::*;
use rmdb::measurement::*;
use time::*;
use rmdb::utils::*;
use std::thread;
use std::sync::Arc;
use std::rc::Rc;
use std::fmt::Debug	;


use std::env;


fn main() {
	split_helper(&mut vec![7, 10, 13], &mut vec![1, 2, 3], &0);
	println!("");
	
	split_helper(&mut vec![7, 10, 13], &mut vec![1, 2, 3], &9);
	println!("");
	
	split_helper(&mut vec![7, 10, 13], &mut vec![1, 2, 3], &12);
	println!("");
	
	split_helper(&mut vec![7, 10, 13], &mut vec![1, 2, 3], &14);
	println!("");

	split_helper(&mut vec![1, 2, 3, 4, 5, 6], &mut vec![1, 2, 3, 4, 5, 6], &7);
	println!("");

	split_helper(&mut vec![1, 2, 3, 4, 5, 6], &mut vec![1, 2, 3, 4, 5, 6], &0);
	println!("");

	split_helper(&mut vec![1, 3, 4, 5, 6, 7], &mut vec![1, 2, 3, 4, 5, 6], &2);
	println!("");

	return;
	let mut btree = BPlusTree::<i32, i32>::new(4);
	btree.insert(4, 1);
	println!("{:?}", btree);
	btree.insert(3, 1);
	btree.insert(2, 1);
	btree.insert(1, 1);

	println!("{:?}", btree);
	
	return;
}
// 	bench_write_one_thread_32768_sensors_1_measurements();
// 	let storage = bench_write_16_threads_32768_sensors_32_measurements();
// 	bench_write_16_threads_32768_sensors_6144_measurements();
// 	bench_write_16_threads_320_sensors_630000_measurements();
// 	bench_read_all_sensors_by_current_time(&storage);
// }
// fn bench_write_one_thread_32768_sensors_1_measurements(){
// 	let time_before = precise_time_ns();
// 	const NUMBER_OF_SENSORS: u64 = 32768;
// 	let storage = DummyStorage::new(NUMBER_OF_SENSORS);
// 	for i in 0..NUMBER_OF_SENSORS{
// 		let m = Measurement::new(i, i, i as f64, 1, 1);
// 		storage.write(m);
// 	}
// 	let time_after = precise_time_ns();
// 	let diff = time_after - time_before;
// 	println!("bench_write_one_thread_32768_measurements: {} ms", (diff as f64) / (1000000.0f64));
// }

// fn bench_write_16_threads_32768_sensors_32_measurements() -> Arc<DummyStorage>{
// 	const NUMBER_OF_SENSORS: u64 = 32768;
// 	const NUMBER_OF_MEASUREMENTS_PER_SENSOR: u64 = 32;
	
// 	let time_before = precise_time_ns();
// 	let mut threads = Vec::<thread::JoinHandle<()>>::new();
// 	let storage = Arc::new(DummyStorage::new(NUMBER_OF_SENSORS));
// 	let ranges = get_equable_ranges(NUMBER_OF_SENSORS, 16);		
// 	for range in ranges{
// 		let storage_copy = storage.clone();
// 		let t = thread::spawn(move || {
// 			for i in range.0..range.1 + 1{
// 				for k in 0..NUMBER_OF_MEASUREMENTS_PER_SENSOR{
// 					let m = Measurement::new(i as u64, k as u64, 1.0f64, 1, 1);
// 					storage_copy.write(m);
// 				}					
// 			}				
// 		});
// 		threads.push(t);
// 	}
// 	for t in threads{
// 		t.join();
// 	}
// 	let time_after = precise_time_ns();
// 	let diff = time_after - time_before;
// 	println!("bench_write_16_threads_32768_sensors_32_measurements: {} ms", (diff as f64) / (1000000.0f64));
// 	return storage;
// }
// fn bench_write_16_threads_32768_sensors_6144_measurements(){
// 	const NUMBER_OF_SENSORS: u64 = 32768;
// 	const NUMBER_OF_MEASUREMENTS_PER_SENSOR: u64 = 6144;
	
// 	let time_before = precise_time_ns();
// 	let mut threads = Vec::<thread::JoinHandle<()>>::new();
// 	let storage = Arc::new(DummyStorage::new(NUMBER_OF_SENSORS));
// 	let ranges = get_equable_ranges(NUMBER_OF_SENSORS, 16);		
// 	for range in ranges{
// 		let storage_copy = storage.clone();
// 		let t = thread::spawn(move || {
// 			for i in range.0..range.1 + 1{
// 				for k in 0..NUMBER_OF_MEASUREMENTS_PER_SENSOR{
// 					let m = Measurement::new(i as u64, k as u64, 1.0f64, 1, 1);
// 					storage_copy.write(m);
// 				}					
// 			}				
// 		});
// 		threads.push(t);
// 	}
// 	for t in threads{
// 		t.join();
// 	}
// 	let time_after = precise_time_ns();
// 	let diff = time_after - time_before;
// 	let ms = (diff as f64) / (1000000.0f64);
// 	let total = NUMBER_OF_SENSORS * NUMBER_OF_MEASUREMENTS_PER_SENSOR;
// 	let speed = ((1000 * total) as f64) / ms;
// 	println!("bench_write_16_threads_32768_sensors_6144_measurements: {} ms. Total number of written measurements: {}. Speed is: {} per second", 
// 		ms, total, speed);
// }
// fn bench_write_16_threads_320_sensors_630000_measurements(){
// 	const NUMBER_OF_SENSORS: u64 = 320;
// 	const NUMBER_OF_MEASUREMENTS_PER_SENSOR: u64 = 630000;
	
// 	let time_before = precise_time_ns();
// 	let mut threads = Vec::<thread::JoinHandle<()>>::new();
// 	let storage = Arc::new(DummyStorage::new(NUMBER_OF_SENSORS));
// 	let ranges = get_equable_ranges(NUMBER_OF_SENSORS, 16);		
// 	for range in ranges{
// 		let storage_copy = storage.clone();
// 		let t = thread::spawn(move || {
// 			for i in range.0..range.1 + 1{
// 				for k in 0..NUMBER_OF_MEASUREMENTS_PER_SENSOR{
// 					let m = Measurement::new(i as u64, k as u64, 1.0f64, 1, 1);
// 					storage_copy.write(m);
// 				}					
// 			}				
// 		});
// 		threads.push(t);
// 	}
// 	for t in threads{
// 		t.join();
// 	}
// 	let time_after = precise_time_ns();
// 	let diff = time_after - time_before;
// 	let ms = (diff as f64) / (1000000.0f64);
// 	let total = NUMBER_OF_SENSORS * NUMBER_OF_MEASUREMENTS_PER_SENSOR;
// 	let speed = ((1000 * total) as f64) / ms;
// 	println!("bench_write_16_threads_320_sensors_630000_measurements: {} ms. Total number of written measurements: {}. Speed is: {} per second", 
// 		ms, total, speed);
// }
// fn bench_read_all_sensors_by_current_time(storage: &DummyStorage){
// 	let time_before = precise_time_ns();
// 	let mut total = 0;
// 	for i in 0..32{
// 		total += storage.read_all_by_current_time().len();
// 	}
// 	let time_after = precise_time_ns();
// 	let diff = time_after - time_before;
// 	println!("bench_read_all_sensors_by_current_time: {} ms. Total: {}", (diff as f64) / (1000000.0f64), total);
// }
