#[derive(Clone, Debug)]
pub struct User {
	id: u64,
}

impl User {
	pub fn new(id: u64) -> Self {
		Self { id }
	}
}

impl User {
	pub fn id(&self) -> u64 {
		self.id
	}
}
