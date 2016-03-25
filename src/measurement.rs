#[derive(Debug, Copy, Clone)]
pub struct Measurement{
	pub id: u64,
	pub time: u64,
	pub value: f64,
	pub tag: i32,
	pub user: i32
}
impl Measurement {
    pub fn new(id: u64, time: u64, value: f64, tag: i32, user: i32) -> Measurement{
    	Measurement{ id: id, time: time, value: value, tag: tag, user: user}
    }
}
impl PartialEq for Measurement {
    fn eq(&self, other: &Measurement) -> bool{
    	return self.id == other.id && self.time == other.time &&
    		self.value == other.value && self.tag == other.tag &&
    		self.user == other.user;
    }
    fn ne(&self, other: &Measurement) -> bool {
    	return !self.eq(other);
    }
}
