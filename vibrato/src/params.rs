mod smooth;
pub use smooth::{LinearSmooth, Smoother};
use {crate::MAX_DEPTH, smooth::LogarithmicSmooth};

#[derive(Clone, Copy)]
pub enum LfoShape {
  Sine,
  Triangle,
  SawUp,
  SawDown,
  Rectangle,
  SampleAndHold,
  Random,
  CurvedRandom,
}

pub struct Params {
  pub freq: LogarithmicSmooth,
  pub depth: f32,
  pub shape: LfoShape,
  pub chance: f32,
  pub time: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: LogarithmicSmooth::new(sample_rate, 0.25),
      depth: 0.,
      shape: LfoShape::Sine,
      chance: 0.,
      time: LinearSmooth::new(sample_rate, 30.),
      is_initialized: false,
    }
  }

  pub fn reset(&mut self) {
    self.is_initialized = false;
  }

  pub fn set(&mut self, freq: f32, depth: f32, shape: i32, chance: f32) {
    self.depth = depth * depth * MAX_DEPTH;
    self.shape = match shape {
      0 => LfoShape::Sine,
      1 => LfoShape::Triangle,
      2 => LfoShape::SawUp,
      3 => LfoShape::SawDown,
      4 => LfoShape::Rectangle,
      5 => LfoShape::SampleAndHold,
      6 => LfoShape::Random,
      7 => LfoShape::CurvedRandom,
      _ => panic!("Unknown lfo shape"),
    };
    self.chance = chance;

    if self.is_initialized {
      self.freq.set_target(freq);
    } else {
      self.freq.reset(freq);
      self.is_initialized = true;
    }
  }
}
