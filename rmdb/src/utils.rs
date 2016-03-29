use std::fmt::Debug;
use measurement::Measurement;

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
pub fn get_slice_index_which_is_greater_or_equal<T: Ord + Debug>(values: &[T], s: T) -> Option<usize>{
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
		// let ref v = values[mid];
		// println!("[{}, {}]. mid = {}. v = {:?}", low, high, mid, v);
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
pub fn get_slice_index_which_is_greater_or_equal2(values: &[Measurement], s: u64) -> Option<usize>{
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
		// let ref v = values[mid];
		// println!("[{}, {}]. mid = {}. v = {:?}", low, high, mid, v);
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
#[cfg(test)]
mod tests{
	use super::*;

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
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[], 0));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_1_without_searching_element(){		
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1], 2));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_1_with_searching_element(){		
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1], 1));
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1], 0));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_2_without_searching_element(){		
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 2], 3));
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 2], 4));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_2_with_searching_element(){		
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 2], 1));
		assert_eq!(Some(1), get_slice_index_which_is_greater_or_equal(&[1, 2], 2));
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 2], 0));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_2_with_searching_element_many_matches(){		
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 1], 1));
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[2, 2], 2));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_3_without_searching_element(){		
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 2, 3], 4));
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 1, 1], 2));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_3_with_searching_element(){		
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 2, 3], 1));
		assert_eq!(Some(1), get_slice_index_which_is_greater_or_equal(&[1, 2, 3], 2));
		assert_eq!(Some(2), get_slice_index_which_is_greater_or_equal(&[1, 2, 3], 3));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_3_with_searching_element_many_matches(){
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 1, 1], 1));
		assert_eq!(Some(1), get_slice_index_which_is_greater_or_equal(&[1, 2, 2], 2));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_4_without_searching_element(){		
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4], 5));
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[4, 4, 4, 4], 5));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_4_with_searching_element(){
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4], 1));
		assert_eq!(Some(1), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4], 2));
		assert_eq!(Some(2), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4], 3));
		assert_eq!(Some(3), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4], 4));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_len_4_with_searching_element_many_matches(){
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 1, 1, 1], 1));
		assert_eq!(Some(1), get_slice_index_which_is_greater_or_equal(&[1, 2, 2, 4], 2));
		assert_eq!(Some(2), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 3], 3));
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[4, 4, 4, 4], 3));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_searching_element_many_matches(){
		assert_eq!(Some(2), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 3, 3, 3, 3, 3, 3], 3));
		assert_eq!(Some(7), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 8], 8));
		assert_eq!(Some(3), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 4, 6, 7, 8, 8], 4));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_with_searching_element(){
		assert_eq!(Some(0), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 1));
		assert_eq!(Some(1), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 2));
		assert_eq!(Some(2), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3));
		assert_eq!(Some(3), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 4));
		assert_eq!(Some(4), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 5));
		assert_eq!(Some(5), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 6));
		assert_eq!(Some(6), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 7));
		assert_eq!(Some(7), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 8));
		assert_eq!(Some(8), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 9));
	}
	#[test]
	fn get_slice_index_which_is_greater_or_equal_test(){
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 10));
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 11));
		assert_eq!(None, get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 12));
		assert_eq!(Some(8), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 7, 8, 9, 11], 10));
		assert_eq!(Some(5), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 5, 7, 8, 9, 11], 6));
		assert_eq!(Some(8), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 4, 4, 4, 4, 6], 5));
		assert_eq!(Some(3), get_slice_index_which_is_greater_or_equal(&[1, 2, 3, 4, 4, 4, 4, 4, 6], 4));
	}
}