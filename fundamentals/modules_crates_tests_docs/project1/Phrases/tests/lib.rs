#[cfg(test)]
mod tests
{
  extern crate Phrases;

  #[test]
//  #[should_panic]
//  #[ignore]
  fn english_greeting_correct()
  {
    assert_eq!("hello", Phrases::greetings::english::hello());
  }

}