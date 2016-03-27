pub mod measurement;
pub mod container;
pub mod storage;
pub mod utils;

// use std::cell::Cell;
// use rmdb::measurement::*;


// #[derive(Debug)]
// pub struct Container{
// 	pub min_time: Cell<i64>,
// 	pub max_time: Cell<i64>,
// 	pub values: Vec<Measurement>,
// 	pub id: i32
// }
// impl Container {
//   pub fn add(&mut self, m: Measurement){
//   	if self.min_time.get() > m.time{
//   		self.min_time.set(m.time);
//   	}
//   	if self.max_time.get() < m.time{
//   		self.max_time.set(m.time);
//   	}
//   	self.values.push(m);
//   }
//   pub fn new(id: i32) -> Container{
//   	Container {
//   		id: id,
//   		min_time: Cell::new(std::i64::MAX),
//   		max_time: Cell::new(std::i64::MIN),
//   		values: vec![]
//   	}
//   }
// }

// #[cfg(test)]
// mod test {
//     #[test]
//     fn it_works() {
//     }
// }
