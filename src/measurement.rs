use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Measurement {
    pub min: f32,
    pub max: f32,
    pub sum: f32,
    pub count: usize,
}
impl Measurement {
    #[inline(always)]
    pub fn new(temp: f32) -> Self {
        Self {
            min: temp,
            max: temp,
            sum: temp,
            count: 1,
        }
    }
    #[inline(always)]
    pub fn update(&mut self, temp: f32) {
        self.min = temp.min(self.min);
        self.max = temp.max(self.max);
        self.sum += temp;
        self.count += 1;
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.1}/{:.1}/{:.1}",
            self.min,
            (self.sum / self.count as f32),
            self.max
        )
    }
}
