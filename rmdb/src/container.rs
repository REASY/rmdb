use std;

use measurement::*;

#[derive(Debug, Copy, Clone)]
pub struct InternalMeasurement{
	pub time: u64,
	pub value: f64,
	pub tag: i32,
	pub user: i32
}
impl InternalMeasurement {
    pub fn new(time: u64, value: f64, tag: i32, user: i32) -> InternalMeasurement{
    	InternalMeasurement{ time: time, value: value, tag: tag, user: user}
    }
    pub fn new_from_measurement(m: Measurement) -> InternalMeasurement{
    	InternalMeasurement{ time: m.time, value: m.value, tag: m.tag, user: m.user}
    }
    pub fn as_measurement(&self, id: u64) -> Measurement {
    	Measurement { id: id, time: self.time, value: self.value, tag: self.tag, user: self.user}
    }
    pub fn empty(time: u64) -> InternalMeasurement{
    	InternalMeasurement{ time: time, value: 0f64, tag: 0, user: 0}
    }
}
// use std::cmp::Ordering;
// impl Ord for InternalMeasurement{
// 	fn cmp(&self, other: &Self) -> Ordering{
// 		return self.time.cmp(&other.time);
// 	}
// }
// impl PartialOrd for InternalMeasurement{
// 	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
// impl PartialEq for InternalMeasurement {
//     fn eq(&self, other: &InternalMeasurement) -> bool{
//     	return self.time == other.time;
//     }
//     fn ne(&self, other: &InternalMeasurement) -> bool {
//     	return !self.eq(other);
//     }
// }
// impl Eq for InternalMeasurement{}

#[derive(Debug)]
pub struct Container{
	pub min_time: u64,
	pub max_time: u64,
	pub	min_measurement: Option<InternalMeasurement>,
	pub	max_measurement: Option<InternalMeasurement>,
	pub values: Vec<InternalMeasurement>,
	pub id: u64
}
impl Container {
  pub fn add(&mut self, m: Measurement){
  	let i_m = InternalMeasurement::new_from_measurement(m);
  	if self.min_time > m.time{
  		self.min_time = m.time;
  		self.min_measurement = Some(i_m);
  	}
  	if self.max_time < m.time{
  		self.max_time = m.time;
  		self.max_measurement = Some(i_m);
  	}
  	self.values.push(i_m);
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

	impl PartialEq for InternalMeasurement {
    fn eq(&self, other: &InternalMeasurement) -> bool{
    	return self.time == other.time &&
    		self.value == other.value && self.tag == other.tag &&
    		self.user == other.user;
    }
    fn ne(&self, other: &InternalMeasurement) -> bool {
    	return !self.eq(other);
    }
}

	#[test]
	fn add_one() {
	    let mut c = Container::new(1);
	    let m = Measurement::new(1, 1, 1.0f64, 1, 1);
	    c.add(m);

	    assert_eq!(1, c.id);
	    assert_eq!(1, c.max_time);
	    assert_eq!(1, c.min_time);

	    let m = InternalMeasurement::new_from_measurement(m);
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
	    
	    assert_eq!(1, c.id);
	    assert_eq!(1, c.min_time);
	    assert_eq!(2, c.max_time);    

	    let m2 = InternalMeasurement::new_from_measurement(m2);
	    let m1 = InternalMeasurement::new_from_measurement(m1);
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
	    
	    assert_eq!(1, c.id);
	    assert_eq!(0, c.min_time);
	    assert_eq!(NUMBER_OF_MEASUREMENTS, c.max_time);
	    
	    let m_max = InternalMeasurement::new(NUMBER_OF_MEASUREMENTS, 1.0f64, 1, 1);
	    let m_min = InternalMeasurement::new(0, 1.0f64, 1, 1);
	    assert_eq!(m_max, c.max_measurement.unwrap());
	    assert_eq!(m_min, c.min_measurement.unwrap());

	    let mut counter = 0;
	    for v in &c.values{
	    	let m = InternalMeasurement::new(counter, 1.0f64, 1, 1);
	    	counter += 1;
	    	assert_eq!(&m, v);
	    }
	}
}