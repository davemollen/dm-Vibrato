pub struct Delta {
  z: f32,
}

impl Delta {
  pub fn new() -> Self {
    Self {
      z: 1., // forces a trigger on initial load
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let output = input - self.z;
    self.z = input;
    output
  }
}
