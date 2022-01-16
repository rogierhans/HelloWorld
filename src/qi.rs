
pub struct QuadraticInterval {
    pub From: f64,
    pub To: f64,
    pub A: f64,
    pub B: f64,
    pub C: f64,
    pub ZID: usize,
}

impl QuadraticInterval {
    // pub fn print(&self) {
    //     println!(
    //         "[{},{}] ({},{},{}) - {}",
    //         self.From, self.To, self.A, self.B, self.C, self.ZID
    //     );
    // }

    // pub fn MinimumHack(&self) -> f64 {
    //     -self.B / (2.0 * self.C)
    // }
    pub fn CustomClone(&self) -> QuadraticInterval {
        QuadraticInterval {
            From: self.From,
            To: self.To,
            A: self.A,
            B: self.B,
            C: self.C,
            ZID: self.ZID,
        }
    }

    pub fn MinimumAtInterval(&self) -> f64 {
        let minimum = -self.B / (2.0 * self.C);
        //println!("{}",minimum);
        if minimum < self.From {
            return self.From;
        } else if minimum > self.To {
            return self.To;
        } else {
            return minimum;
        }
    }

    pub fn MinimumAtIntervalRestricted(&self, max: f64) -> f64 {
        let minimum = self.MinimumAtInterval();
        if max < self.From {
            panic!("oh no!");
        }
        if minimum > max {
            return max;
        } else {
            return minimum;
        }
    }

    pub fn GetValue(&self, p: f64) -> f64 {
        self.A + p * self.B + (p * p * self.C)
    }

    pub fn ValueMinimum(&self) -> f64 {
        let minimum = self.MinimumAtInterval();
        return self.GetValue(minimum);
    }

    pub fn ValueMinimumRestriced(&self, max: f64) -> f64 {
        let minimum = self.MinimumAtIntervalRestricted(max);
        return self.GetValue(minimum);
    }

    // double QuadraticInterval::GetValueInt(double p)
    // {
    // 	if (p <= To && From <= To)
    // 		return A + p * B + (p * p * C);
    // 	else
    // 		return  numeric_limits<double>::max();
    // }

    // void QuadraticInterval::Print() {
    // 	cout << From << " " << To << " " << A << " " << B << " " << C << " " << endl;
    // }
}
