use std;
use std::cell::Cell;
use std::sync::RwLock;
use measurement::*;

#[derive(Debug)]
pub struct Container{
	pub min_time: u64,
	pub max_time: u64,
	pub	min_measurement: Option<Measurement>,
	pub	max_measurement: Option<Measurement>,
	pub values: Vec<Measurement>,
	pub id: u64
}
impl Container {
  pub fn add(&mut self, m: Measurement){
  	if self.min_time > m.time{
  		self.min_time = m.time;
  		self.min_measurement = Some(m);
  	}
  	if self.max_time < m.time{
  		self.max_time = m.time;
  		self.max_measurement = Some(m);
  	}
  	self.values.push(m);
  }
  pub fn new(id: u64) -> Container{
  	Container {
  		id: id,
  		min_time: std::u64::MAX,
  		max_time: std::u64::MIN,
  		min_measurement: None,
  		max_measurement: None,
  		values: vec![]
  	}
  }
}

#[cfg(test)]
mod test {
	use measurement::*;
	use container::*;
	#[test]
	fn add_one() {
	    let mut c = Container::new(1);
	    let m = Measurement::new(1, 1, 1.0f64, 1, 1);
	    c.add(m);

	    assert_eq!(1, c.max_time);
	    assert_eq!(1, c.min_time);

	    assert_eq!(m, c.max_measurement.unwrap());
	    assert_eq!(m, c.min_measurement.unwrap());

	    let ref saved_m = c.values[0];    
	    assert_eq!(&m, saved_m);
	}

	#[test]
	fn add_two() {
	    let mut c = Container::new(1);
	    let m1 = Measurement::new(1, 1, 1.0f64, 1, 1);
	    c.add(m1);

	    let m2 = Measurement::new(1, 2, 2.0f64, 1, 1);
	    c.add(m2);
	    
	    assert_eq!(1, c.min_time);
	    assert_eq!(2, c.max_time);    

	    assert_eq!(m2, c.max_measurement.unwrap());
	    assert_eq!(m1, c.min_measurement.unwrap());

	    let ref saved_m = c.values[0];    
	    assert_eq!(&m1, saved_m);
	    let ref saved_m = c.values[1];    
	    assert_eq!(&m2, saved_m);
	}
	#[test]
	fn add_many() {
		const NUMBER_OF_MEASUREMENTS: u64 = 1000;
	    let mut c = Container::new(1);

	    for i in 0..NUMBER_OF_MEASUREMENTS+1{
	    	let m = Measurement::new(1, i, 1.0f64, 1, 1);
	    	c.add(m);
	    }   
	    
	    assert_eq!(0, c.min_time);
	    assert_eq!(NUMBER_OF_MEASUREMENTS, c.max_time);
	    
	    let m_max = Measurement::new(1, NUMBER_OF_MEASUREMENTS, 1.0f64, 1, 1);
	    let m_min = Measurement::new(1, 0, 1.0f64, 1, 1);
	    assert_eq!(m_max, c.max_measurement.unwrap());
	    assert_eq!(m_min, c.min_measurement.unwrap());

	    let mut counter = 0;
	    for v in &c.values{
	    	let m = Measurement::new(1, counter, 1.0f64, 1, 1);
	    	counter += 1;
	    	assert_eq!(&m, v);
	    }
	}
}