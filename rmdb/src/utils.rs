extern crate num_cpus;

use std::fmt::Debug;
use container::InternalMeasurement;

pub fn get_ranges(sensors: usize, threads: usize) -> Vec<(usize, usize)>{
	let mut threads = threads;
	if threads > sensors{
		threads = sensors;
	}	
	let mut result = Vec::<(usize, usize)>::new();
	let range_size = sensors / threads;
	let mut begin = 0;
	for i in 1..threads{
		result.push((begin, range_size * i - 1));
		begin = range_size * i;
	}
	result.push((begin, sensors-1));
	return result;
}

pub fn get_equable_ranges(sensors: u64, threads: u64) -> Vec<(u64, u64)>{
	if threads > sensors{
		panic!("threads {} must be <= sensors {}", threads, sensors);
	}
	if sensors % threads != 0{
		panic!("sensors {} must divide evenly into threads {}", sensors, threads);
	}
	let mut result = Vec::<(u64, u64)>::new();
	let range_size = sensors / threads;
	let mut begin = 0;
	for i in 1..threads{
		result.push((begin, range_size * i - 1));
		begin = range_size * i;
	}
	result.push((begin, sensors-1));
	return result;
}
pub fn lower_bound<T: Ord + Debug>(values: &[T], s: T) -> Option<usize>{
	let len = values.len();
	if len == 0 { return None; }
	if len == 1{
		if values[0] >= s{ return Some(0); }
		else { return None; }
	}
	let mut low = 0;
	let mut high = len - 1;
	let mut mid = 0;
	let mut prev_mid = 0;
	loop{
		mid = low + (high - low) / 2;
		if values[mid] < s { low = mid; }
		else { high = mid; }
		if prev_mid == mid { break; }
		prev_mid = mid;
	}
	if mid < len - 1{
		if values[mid] < s { mid = mid + 1; }
	}
	if mid == len - 1 && values[mid] < s { return None;	}
	else { return Some(mid); }	
}
pub fn lower_bound2(values: &[InternalMeasurement], s: u64) -> Option<usize>{
	let len = values.len();
	if len == 0 { return None; }
	if len == 1{
		if values[0].time >= s{ return Some(0); }
		else { return None; }
	}
	let mut low = 0;
	let mut high = len - 1;
	let mut mid = 0;
	let mut prev_mid = 0;
	loop{
		mid = low + (high - low) / 2;
		if values[mid].time < s{ low = mid; }
		else { high = mid; }
		if prev_mid == mid { break; }
		prev_mid = mid;
	}
	if mid < len - 1{
		if values[mid].time < s { mid = mid + 1;}
	}
	if mid == len - 1 && values[mid].time < s { return None;}
	else { return Some(mid); }	
}
pub fn get_cpus_number() -> usize{
	return num_cpus::get();
}
#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn get_ranges_test1(){
		let ranges = get_ranges(16, 8);
		assert_eq!(8, ranges.len());
		assert_eq!((0, 1), ranges[0]);
		assert_eq!((2, 3), ranges[1]);
		assert_eq!((4, 5), ranges[2]);
		assert_eq!((6, 7), ranges[3]);
		assert_eq!((8, 9), ranges[4]);
		assert_eq!((10, 11), ranges[5]);
		assert_eq!((12, 13), ranges[6]);
		assert_eq!((14, 15), ranges[7]);
	}
	#[test]
	fn get_ranges_test2(){
		let ranges = get_ranges(17, 8);
		assert_eq!(8, ranges.len());
		assert_eq!((0, 1), ranges[0]);
		assert_eq!((2, 3), ranges[1]);
		assert_eq!((4, 5), ranges[2]);
		assert_eq!((6, 7), ranges[3]);
		assert_eq!((8, 9), ranges[4]);
		assert_eq!((10, 11), ranges[5]);
		assert_eq!((12, 13), ranges[6]);
		assert_eq!((14, 16), ranges[7]);
	}
	#[test]
	fn get_ranges_test3(){
		let ranges = get_ranges(8, 17);
		assert_eq!(8, ranges.len());
		assert_eq!((0, 0), ranges[0]);
		assert_eq!((1, 1), ranges[1]);
		assert_eq!((2, 2), ranges[2]);
		assert_eq!((3, 3), ranges[3]);
		assert_eq!((4, 4), ranges[4]);
		assert_eq!((5, 5), ranges[5]);
		assert_eq!((6, 6), ranges[6]);
		assert_eq!((7, 7), ranges[7]);
	}
	#[test]
	fn get_ranges_test4(){
		let ranges = get_ranges(19, 3);
		assert_eq!(3, ranges.len());
		assert_eq!((0, 5), ranges[0]);
		assert_eq!((6, 11), ranges[1]);
		assert_eq!((12, 18), ranges[2]);
	}
	#[test]
	fn get_equable_ranges_odd_sensors_odd_threads(){
		let ranges = get_equable_ranges(33, 11);
		assert_eq!(11, ranges.len());
		assert_eq!((0, 2), ranges[0]);
		assert_eq!((3, 5), ranges[1]);
		assert_eq!((6, 8), ranges[2]);
		assert_eq!((9, 11), ranges[3]);
		assert_eq!((12, 14), ranges[4]);
		assert_eq!((15, 17), ranges[5]);
		assert_eq!((18, 20), ranges[6]);
		assert_eq!((21, 23), ranges[7]);
		assert_eq!((24, 26), ranges[8]);
		assert_eq!((27, 29), ranges[9]);
		assert_eq!((30, 32), ranges[10]);
	}
	#[test]
	fn get_equable_ranges_even_sensors_even_threads(){
		let ranges = get_equable_ranges(10, 2);
		assert_eq!(2, ranges.len());
		assert_eq!((0, 4), ranges[0]);
		assert_eq!((5, 9), ranges[1]);

		let ranges = get_equable_ranges(8, 4);
		assert_eq!(4, ranges.len());
		assert_eq!((0, 1), ranges[0]);
		assert_eq!((2, 3), ranges[1]);
		assert_eq!((4, 5), ranges[2]);
		assert_eq!((6, 7), ranges[3]);
	}
	#[test]
	fn get_righter_less_or_equal_empty_slice(){		
		assert_eq!(None, lower_bound(&[], 0));
	}
	#[test]
	fn lower_bound_with_len_1_without_searching_element(){		
		assert_eq!(None, lower_bound(&[1], 2));
	}
	#[test]
	fn lower_bound_with_len_1_with_searching_element(){		
		assert_eq!(Some(0), lower_bound(&[1], 1));
		assert_eq!(Some(0), lower_bound(&[1], 0));
	}
	#[test]
	fn lower_bound_with_len_2_without_searching_element(){		
		assert_eq!(None, lower_bound(&[1, 2], 3));
		assert_eq!(None, lower_bound(&[1, 2], 4));
	}
	#[test]
	fn lower_bound_with_len_2_with_searching_element(){		
		assert_eq!(Some(0), lower_bound(&[1, 2], 1));
		assert_eq!(Some(1), lower_bound(&[1, 2], 2));
		assert_eq!(Some(0), lower_bound(&[1, 2], 0));
	}
	#[test]
	fn lower_bound_with_len_2_with_searching_element_many_matches(){		
		assert_eq!(Some(0), lower_bound(&[1, 1], 1));
		assert_eq!(Some(0), lower_bound(&[2, 2], 2));
	}
	#[test]
	fn lower_bound_with_len_3_without_searching_element(){		
		assert_eq!(None, lower_bound(&[1, 2, 3], 4));
		assert_eq!(None, lower_bound(&[1, 1, 1], 2));
	}
	#[test]
	fn lower_bound_with_len_3_with_searching_element(){		
		assert_eq!(Some(0), lower_bound(&[1, 2, 3], 1));
		assert_eq!(Some(1), lower_bound(&[1, 2, 3], 2));
		assert_eq!(Some(2), lower_bound(&[1, 2, 3], 3));
	}
	#[test]
	fn lower_bound_with_len_3_with_searching_element_many_matches(){
		assert_eq!(Some(0), lower_bound(&[1, 1, 1], 1));
		assert_eq!(Some(1), lower_bound(&[1, 2, 2], 2));
	}
	#[test]
	fn lower_bound_with_len_4_without_searching_element(){		
		assert_eq!(None, lower_bound(&[1, 2, 3, 4], 5));
		assert_eq!(None, lower_bound(&[4, 4, 4, 4], 5));
	}
	#[test]
	fn lower_bound_with_len_4_with_searching_element(){
		assert_eq!(Some(0), lower_bound(&[1, 2, 3, 4], 1));
		assert_eq!(Some(1), lower_bound(&[1, 2, 3, 4], 2));
		assert_eq!(Some(2), lower_bound(&[1, 2, 3, 4], 3));
		assert_eq!(Some(3), lower_bound(&[1, 2, 3, 4], 4));
	}
	#[test]
	fn lower_bound_with_len_4_with_searching_element_many_matches(){
		assert_eq!(Some(0), lower_bound(&[1, 1, 1, 1], 1));
		assert_eq!(Some(1), lower_bound(&[1, 2, 2, 4], 2));
		assert_eq!(Some(2), lower_bound(&[1, 2, 3, 3], 3));
		assert_eq!(Some(0), lower_bound(&[4, 4, 4, 4], 3));
	}
	#[test]
	fn lower_bound_with_searching_element_many_matches(){
		assert_eq!(Some(2), lower_bound(&[1, 2, 3, 3, 3, 3, 3, 3, 3], 3));
		assert_eq!(Some(7), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 8], 8));
		assert_eq!(Some(3), lower_bound(&[1, 2, 3, 4, 4, 6, 7, 8, 8], 4));
	}
	#[test]
	fn lower_bound_with_searching_element(){
		assert_eq!(Some(0), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 1));
		assert_eq!(Some(1), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 2));
		assert_eq!(Some(2), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3));
		assert_eq!(Some(3), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 4));
		assert_eq!(Some(4), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 5));
		assert_eq!(Some(5), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 6));
		assert_eq!(Some(6), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 7));
		assert_eq!(Some(7), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 8));
		assert_eq!(Some(8), lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 9));
	}
	#[test]
	fn lower_bound_test(){
		assert_eq!(None, lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 10));
		assert_eq!(None, lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 11));
		assert_eq!(None, lower_bound(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 12));
		assert_eq!(Some(8), lower_bound(&[1, 2, 3, 4, 5, 7, 8, 9, 11], 10));
		assert_eq!(Some(5), lower_bound(&[1, 2, 3, 4, 5, 7, 8, 9, 11], 6));
		assert_eq!(Some(8), lower_bound(&[1, 2, 3, 4, 4, 4, 4, 4, 6], 5));
		assert_eq!(Some(3), lower_bound(&[1, 2, 3, 4, 4, 4, 4, 4, 6], 4));
	}
}