enum Compare{
  Less,
  Greater,
  Equal
}
impl Compare{
  fn compare(val1: i8, val2: i8)-> Compare{
    if val1 == val2 {
      return Compare::Equal
    }
    else if val1 > val2 {
      return Compare::Greater
    }
    Compare::Less
  }
}
fn main() {
  let alice: [i8; 3] = [1, 2, 3]; 
  let bob: [i8; 3] = [3, 2, 1];
  let mut result = [0i8; 2];

  use Compare::*;
  for indx in 0..3 {
    let res = Compare::compare(alice[indx], bob[indx]);
    match res {
      Less => result[1] = result[1]+1,
      Greater => result[0] = result[0]+1,
      _=> println!("equal..."),
    }
  };

  println!("result {:?}", result);
}
