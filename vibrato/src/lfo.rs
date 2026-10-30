mod delta;
mod phasor;
use {
  crate::{params::LfoShape, shared::float_ext::FloatExt},
  delta::Delta,
  phasor::Phasor,
  std::f32::consts::{PI, TAU},
};

pub struct Lfo {
  phasor: Phasor,
  delta: Delta,
  is_enabled: bool,
  origin: f32,
  target: f32,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
      is_enabled: true,
      origin: 0.,
      target: 0.,
    }
  }

  pub fn reset(&mut self) {
    self.phasor.reset();
    self.delta.reset();
    self.is_enabled = true;
    self.origin = 0.;
    self.target = 0.;
  }

  pub fn initialize(&mut self, chance: f32) {
    self.is_enabled = fastrand::f32() <= chance;
  }

  pub fn process(&mut self, freq: f32, shape: LfoShape, chance: f32) -> f32 {
    let phase = self.phasor.process(freq);
    let trigger = self.delta.process(phase) < 0.;
    if trigger {
      self.is_enabled = fastrand::f32() <= chance;
    }

    match shape {
      LfoShape::Sine => {
        if !self.is_enabled {
          return 0.;
        }

        ((phase + 0.75) * TAU).fast_sin() * 0.5 + 0.5
      }
      LfoShape::Triangle => {
        if !self.is_enabled {
          return 0.;
        }

        let phase = Self::wrap(phase + 0.25);
        if phase > 0.5 {
          (phase - 0.5) * -2. + 1.
        } else {
          phase * 2.
        }
      }
      LfoShape::SawUp => {
        if !self.is_enabled {
          return 0.;
        }

        1. - phase
      }
      LfoShape::SawDown => {
        if !self.is_enabled {
          return 0.;
        }

        phase
      }
      LfoShape::Rectangle => {
        if !self.is_enabled {
          return 0.;
        }

        if phase > 0.5 {
          1.
        } else {
          0.
        }
      }
      LfoShape::SampleAndHold => {
        if trigger {
          self.target = if self.is_enabled { fastrand::f32() } else { 0. };
        }
        self.target
      }
      LfoShape::Random => {
        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled { fastrand::f32() } else { 0. };
        }
        self.linear_interp(phase)
      }
      LfoShape::CurvedRandom => {
        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled { fastrand::f32() } else { 0. };
        }
        self.cosine_interp(phase)
      }
    }
  }

  fn linear_interp(&self, mix: f32) -> f32 {
    self.origin + (self.target - self.origin) * mix
  }

  fn cosine_interp(&self, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).fast_cos()) * 0.5;
    self.origin + (self.target - self.origin) * cosine_mix
  }

  fn wrap(x: f32) -> f32 {
    if x >= 1. {
      x - 1.
    } else {
      x
    }
  }
}
