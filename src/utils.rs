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
}