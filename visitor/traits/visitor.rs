trait visitor {
	u8 myInt;

	fn myfunc(&self) -> u8 {
		return self.myInt;
	}
}
