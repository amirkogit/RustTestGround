extern crate Phrases;
use Phrases::greetings::english;

fn main() 
{
   println!("English: {} {}", english::hello(), english::goodbye());
   println!("Japanese: {} {}", Phrases::greetings::japanese::hello(), Phrases::greetings::japanese::goodbye());
}
