#[derive(Debug)]
pub struct CityData {
    pub min: f64,
    pub max: f64,
    sum: f64,
    count: usize,
}

impl Default for CityData {
    fn default() -> Self {
        CityData {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: f64::default(),
            count: usize::default(),
        }
    }
}

impl CityData {
    pub fn add(&mut self, num: f64) {
        self.min = self.min.min(num);
        self.max = self.max.max(num);
        self.sum += num;
        self.count += 1;
    }
    pub fn mean(&self) -> f64 {
        self.sum / (self.count as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::CityData;

    fn expect_near(a: f64, b: f64) {
        const PRECISION: f64 = 1e-9;
        assert!((a - b).abs() <= PRECISION, "{a} is different from {b}");
    }

    #[test]
    fn add_test() {
        let mut city_data = CityData::default();

        city_data.add(1.);
        expect_near(city_data.min, 1.);
        expect_near(city_data.max, 1.);
        expect_near(city_data.sum, 1.);
        assert_eq!(city_data.count, 1);
        expect_near(city_data.mean(), 1.);

        city_data.add(42.);
        expect_near(city_data.min, 1.);
        expect_near(city_data.max, 42.);
        expect_near(city_data.sum, 43.);
        assert_eq!(city_data.count, 2);
        expect_near(city_data.mean(), 21.5);
    }
}
