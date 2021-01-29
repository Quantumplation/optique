pub fn next_up(v: f64) -> f64 {
  if v.is_infinite() && v > 0. { v }
  else if v == -0. { 0. }
  else {
    let mut bits = v.to_bits();
    if v > 0. { bits += 1 }
    else      { bits -= 1 }
    f64::from_bits(bits)
  }
}

pub fn next_down(v: f64) -> f64 {
  if v.is_infinite() && v < 0. { v }
  else if v == 0. { -0. }
  else {
    let mut bits = v.to_bits();
    if v > 0. { bits -= 1 }
    else      { bits += 1 }
    f64::from_bits(bits)
  }
}