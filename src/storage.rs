use measurement::*;
use container::*;
use std::sync;

pub trait Storage{
	fn write(&self, m: Measurement);	
	fn read_by_current_time(&self, id: u64) -> Option<Measurement>;
	fn read_all_by_current_time(&self) -> Vec<Measurement>;
	fn read_all_by_time_interval(&self, begin: u64, end: u64) -> Vec<Measurement>;
	fn read_some_by_time_interval(&self, ids: &[u64], begin: u64, end: u64);
}
#[derive(Debug)]
pub struct DummyStorage  {
	sensors: u64,
	containers: Vec<sync::RwLock<Container>>,
}
impl DummyStorage{
	 pub fn new(sensors: u64) -> DummyStorage{
	 	if sensors <= 0{
	 		panic!("sensors '{}' out of range! Must be > 0", sensors);
	 	}
	 	let storage = sensors as usize;
	 	let mut v = Vec::<sync::RwLock<Container>>::with_capacity(storage);
	 	for i in 0..sensors{
	 		v.push(sync::RwLock::new(Container::new(i as u64)));
	 	}
	 	DummyStorage { sensors: sensors, containers: v}
	 }
	 
}
pub fn check_arg(sensors: u64, id: u64){
	if id >= sensors{
		panic!("id[{}] is out of range! sensors: {}", id, sensors);
	}
}
impl Storage for DummyStorage {	
	fn write(&self, m: Measurement){
		check_arg(self.sensors, m.id);
		let idx = m.id as usize;
		let mut container = self.containers[idx].write().unwrap();
		container.add(m);
	}
	fn read_by_current_time(&self, id: u64) -> Option<Measurement>{
		check_arg(self.sensors, id);
		let idx = id as usize;
		let container = self.containers[idx].read().unwrap();
		return container.max_measurement.get().clone();
	}
	fn read_all_by_current_time(&self) -> Vec<Measurement>{		
		unimplemented!();
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
	use measurement::*;
	
	use storage::Storage;
	use storage::DummyStorage;	

	fn write_measurements(sensors: u64, measurements: u64) -> Rc<DummyStorage>{
		let storage = DummyStorage::new(sensors);
		for i in 0..sensors{
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
			assert_eq!(1, c.max_time.get());
			assert_eq!(1, c.min_time.get());
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
			assert_eq!(2, c.max_time.get());
			assert_eq!(1, c.min_time.get());
			
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
			assert_eq!(NUMBER_OF_MEASUREMENTS, c.max_time.get());
			assert_eq!(1, c.min_time.get());
			for j in 1..NUMBER_OF_MEASUREMENTS + 1{
				let m = Measurement::new(i, j, 1.0f64, 1, 1);
				let values_idx = j - 1;
				let ref saved_m = c.values[values_idx as usize];
				assert_eq!(&m, saved_m);
			}
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
	// #[bench]
	// fn bench_write_one_thread_many_measurements(b: &mut Bencher){
	// 	const NUMBER_OF_SENSORS: u64 = 32768;

	// 	let storage = DummyStorage::new(NUMBER_OF_SENSORS);
	// 	for i in 0..NUMBER_OF_SENSORS{
	// 		let m = Measurement::new(i, i, i as f64, 1, 1);
	// 		storage.write(m);	
	// 	}
	// }
}
