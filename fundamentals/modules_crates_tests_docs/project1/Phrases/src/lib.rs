pub mod greetings
{
	pub mod english;
	
	pub mod japanese
	{
		pub fn hello() -> String { return "konnichiwa".to_string(); }
		pub fn goodbye() -> String { return "sayonara".to_string(); }
	}
}