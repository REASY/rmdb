use measurement::*;
use container::*;
use std::sync;
use utils::*;

pub trait Storage{
	fn write(&self, m: Measurement);	
	fn read_by_current_time(&self, id: u64) -> Option<Measurement>;
	fn read_all_by_current_time(&self) -> Vec<Option<Measurement>>;
	fn read_by_time_interval(&self, id: u64, begin: u64, end: u64) -> Option<Vec<Measurement>>;
	fn read_all_by_time_interval(&self, begin: u64, end: u64) -> Vec<Measurement>;
	fn read_some_by_time_interval(&self, ids: &[u64], begin: u64, end: u64);
}
#[derive(Debug)]
pub struct DummyStorage  {
	sensors_count: u64,
	containers: Vec<sync::RwLock<Container>>,
}
impl DummyStorage{
	 pub fn new(sensors_count: u64) -> DummyStorage{
	 	if sensors_count == 0{
	 		panic!("sensors_count must be > 0");
	 	}
	 	let storage = sensors_count as usize;
	 	let mut v = Vec::<sync::RwLock<Container>>::with_capacity(storage);
	 	for i in 0..sensors_count{
	 		v.push(sync::RwLock::new(Container::new(i as u64)));
	 	}
	 	DummyStorage { sensors_count: sensors_count, containers: v}
	 }
	 
}
pub fn check_arg(sensors_count: u64, id: u64){
	if id >= sensors_count{
		panic!("id[{}] is out of range! sensors_count: {}", id, sensors_count);
	}
}
impl Storage for DummyStorage {	
	fn write(&self, m: Measurement){
		check_arg(self.sensors_count, m.id);
		let idx = m.id as usize;
		let mut container = self.containers[idx].write().unwrap();
		container.add(m);
	}
	fn read_by_current_time(&self, id: u64) -> Option<Measurement>{
		check_arg(self.sensors_count, id);
		let idx = id as usize;
		let container = self.containers[idx].read().unwrap();
		return container.max_measurement.clone();
	}
	fn read_all_by_current_time(&self) -> Vec<Option<Measurement>>{		
		let mut result = Vec::<Option<Measurement>>::with_capacity(self.sensors_count as usize);
		for i in 0..self.sensors_count{
			result.push(self.read_by_current_time(i));
		}
		return result;
	}
	fn read_by_time_interval(&self, id: u64, begin: u64, end: u64) -> Option<Vec<Measurement>>{
		check_arg(self.sensors_count, id);
		let idx = id as usize;
		let container = self.containers[idx].read().unwrap();
		println!("[{}, {}]. min_time: {}, max_time: {}", begin, end, container.min_time, container.max_time);
		if begin >= container.min_time || (begin < container.min_time && end >= container.min_time) {
			let mut result = Vec::<Measurement>::new();
			let index = get_slice_index_which_is_greater_or_equal2(&container.values, begin).unwrap();
			println!("Index: {}", index);
			for i in index..container.values.len(){
				let m = container.values[i];
				if m.time > end{ break; }
				result.push(m);
			}
			return Some(result);			
		}
		else { return None;}
	}
	fn read_all_by_time_interval(&self, begin: u64, end: u64) -> Vec<Measurement>{		
		unimplemented!();
	}
	fn read_some_by_time_interval(&self, ids: &[u64], begin: u64, end: u64){
		unimplemented!();
	}
}

#[cfg(test)]
mod tests {
	// use test::Bencher;
	use std::rc::Rc;
	use std::thread;
	use std::sync::Arc;

	use measurement::*;	
	use storage::Storage;
	use storage::DummyStorage;	
	use utils::*;

	fn write_measurements(sensors_count: u64, measurements: u64) -> Rc<DummyStorage>{
		let storage = DummyStorage::new(sensors_count);
		for i in 0..sensors_count{
			for j in 1..measurements + 1{
				let m = Measurement::new(i, j, 1.0f64, 1, 1);
				storage.write(m);				
			}
		}
		return Rc::new(storage);
	}

	#[test]
	fn write_one_thread_one_measurement_per_sensor(){
		const NUMBER_OF_SENSORS: u64 = 1000;

		let storage = DummyStorage::new(NUMBER_OF_SENSORS);
		for i in 0..NUMBER_OF_SENSORS{
			let m = Measurement::new(i, 1, 1.0f64, 1, 1);
			storage.write(m);
		}
		for i in 0..NUMBER_OF_SENSORS{
			let idx = i as usize;
			let c = storage.containers[idx].read().unwrap();
			assert_eq!(1, c.max_time);
			assert_eq!(1, c.min_time);
			let ref saved_m = c.values[0];
			let m = Measurement::new(i as u64, 1, 1.0f64, 1, 1);
			assert_eq!(&m, saved_m);
		}
	}

	#[test]
	fn write_one_thread_two_measurements_per_sensor(){
		const NUMBER_OF_SENSORS: u64 = 1000;

		let storage = DummyStorage::new(NUMBER_OF_SENSORS);		
		for i in 0..NUMBER_OF_SENSORS{
			let m = Measurement::new(i, 1, 1.0f64, 1, 1);
			storage.write(m);
			let m = Measurement::new(i, 2, 1.0f64, 1, 1);
			storage.write(m);
		}
		for i in 0..NUMBER_OF_SENSORS{
			let idx = i as usize;
			let c = storage.containers[idx].read().unwrap();
			assert_eq!(2, c.max_time);
			assert_eq!(1, c.min_time);
			
			let ref saved_m = c.values[1];
			let m = Measurement::new(i as u64, 2, 1.0f64, 1, 1);
			assert_eq!(&m, saved_m);

			let ref saved_m = c.values[0];
			let m = Measurement::new(i as u64, 1, 1.0f64, 1, 1);
			assert_eq!(&m, saved_m);
		}
	}
	#[test]
	fn write_one_thread_many_measurements_per_sensor(){
		const NUMBER_OF_SENSORS: u64 = 1000;
		const NUMBER_OF_MEASUREMENTS: u64 = 10000;

		let storage = DummyStorage::new(NUMBER_OF_SENSORS);
		for i in 0..NUMBER_OF_SENSORS{
			for j in 1..NUMBER_OF_MEASUREMENTS + 1{
				let m = Measurement::new(i, j, 1.0f64, 1, 1);
				storage.write(m);				
			}
		}
		for i in 0..NUMBER_OF_SENSORS{
			let idx = i as usize;
			let c = storage.containers[idx].read().unwrap();
			assert_eq!(NUMBER_OF_MEASUREMENTS, c.max_time);
			assert_eq!(1, c.min_time);
			for j in 1..NUMBER_OF_MEASUREMENTS + 1{
				let m = Measurement::new(i, j, 1.0f64, 1, 1);
				let values_idx = j - 1;
				let ref saved_m = c.values[values_idx as usize];
				assert_eq!(&m, saved_m);
			}
		}
	}
	#[test]
	fn write_16_threads_many_measurements_per_sensor(){
		const NUMBER_OF_SENSORS: u64 = 128;
		const NUMBER_OF_MEASUREMENTS: u64 = 10000;

		let mut threads = Vec::<thread::JoinHandle<()>>::new();
		let storage = Arc::new(DummyStorage::new(NUMBER_OF_SENSORS));
		let ranges = get_equable_ranges(NUMBER_OF_SENSORS, 16);		
		for range in ranges{
			let storage_copy = storage.clone();
			let t = thread::spawn(move || {
				for i in range.0..range.1 + 1{
					for k in 0..NUMBER_OF_MEASUREMENTS{
						let m = Measurement::new(i as u64, k as u64, 1.0f64, 1, 1);
						storage_copy.write(m);
					}					
				}				
			});
			threads.push(t);
		}
		for t in threads{
			t.join();
		}
		println!("{:?}", storage);
		for i in 0..NUMBER_OF_SENSORS{
			let m = Measurement::new(i as u64, NUMBER_OF_MEASUREMENTS - 1 as u64, 1.0f64, 1, 1);
			let read_m = storage.read_by_current_time(i);
			assert_eq!(m, read_m.unwrap());
		}
	}
	#[test]
	fn read_by_current_time_test(){
		const NUMBER_OF_SENSORS: u64 = 1000;
		const NUMBER_OF_MEASUREMENTS: u64 = 10000;

		let storage = write_measurements(NUMBER_OF_SENSORS, NUMBER_OF_MEASUREMENTS);
		for i in 0..NUMBER_OF_SENSORS{
			let m = storage.read_by_current_time(i);
			let m_max = Measurement::new(i, NUMBER_OF_MEASUREMENTS, 1.0f64, 1, 1);
			assert_eq!(m_max, m.unwrap());
		}
	}
	#[test]
	fn read_by_time_interval_test(){
		const NUMBER_OF_SENSORS: u64 = 1000;
		const NUMBER_OF_MEASUREMENTS: u64 = 10000;

		let storage = write_measurements(NUMBER_OF_SENSORS, NUMBER_OF_MEASUREMENTS);
		for i in 0..NUMBER_OF_SENSORS{
			let result = storage.read_by_time_interval(i, 0, NUMBER_OF_MEASUREMENTS).unwrap();
			assert_eq!(NUMBER_OF_MEASUREMENTS as usize, result.len());

			let mut c = 0;
			for j in 1..NUMBER_OF_MEASUREMENTS + 1{
				let m = Measurement::new(i, j, 1.0f64, 1, 1);
				assert_eq!(m, result[c]);
				c += 1;
			}
		}
	}
}
