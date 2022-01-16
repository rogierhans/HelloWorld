use crate::qi;
use crate::suc;
pub struct F {
    pub Intervals: Vec<qi::QuadraticInterval>,
    pub StartIndex: usize,
}

impl F {
    // pub fn Print(&self) {
    //     for elem in self.Intervals.iter() {
    //         elem.print();
    //     }
    // }
    pub fn CustomClone(&self) -> F {
        F {
            Intervals: self.Intervals.iter().map(|e| e.CustomClone()).collect(),
            StartIndex : self.StartIndex
        }
    }

    pub fn IncreasePoints(&mut self, t: usize, UC: &suc::SUC) {
        for mut elem in &mut self.Intervals {
            elem.A += UC.A;
            elem.B += -UC.LagrangeMultipliers[t] + UC.B + UC.BM[t];
            //println!("{} {} {}", -UC.LagrangeMultipliers[t] , UC.B ,UC.BM[t]);
            elem.C += UC.C + UC.CM[t];
        }
    }
    pub fn BestValue(&self) -> f64 {
        let test = self.Intervals.iter().fold(f64::INFINITY, |minimum, elem| {
            if minimum <= elem.ValueMinimum() {
                minimum
            } else {
                elem.ValueMinimum()
            }
        });
        return test;
    }
    pub fn GetOptimalNode(&self) -> usize {
        let mut INDEX = 0;
        while -self.Intervals[INDEX].B > self.Intervals[INDEX].To * 2.0 * self.Intervals[INDEX].C
            && INDEX < self.Intervals.len() - 1
        {
            INDEX += 1;
        }
        return INDEX;
    }

    pub fn ShiftLeft(&mut self, index: usize, UC: &suc::SUC) {
        for (i, elem) in self.Intervals.iter_mut().enumerate() {
            if i <= index {
                elem.From = f64::max(UC.pMin, elem.From - UC.RampDown);
                elem.To = f64::max(UC.pMin, elem.To - UC.RampDown);
                elem.A = elem.A + UC.RampDown * elem.B + UC.RampDown * UC.RampDown * elem.C;
                elem.B = elem.B + UC.RampDown * elem.C * 2.0;
            }
        }
    }

    pub fn ShiftRight(&mut self, index: usize, UC: &suc::SUC) {
        for (i, elem) in self.Intervals.iter_mut().enumerate() {
            if i >= index {
                elem.From = f64::min(UC.pMax, elem.From + UC.RampUp);
                elem.To = f64::min(UC.pMax, elem.To + UC.RampUp);
                elem.A = elem.A - UC.RampUp * elem.B + UC.RampUp * UC.RampUp * elem.C;
                elem.B = elem.B - UC.RampUp * elem.C * 2.0;
            }
        }
    }

    pub fn Trim(&mut self) {
        while self.Intervals[0].From == self.Intervals[0].To {
            self.Intervals.remove(0);
        }
        while self.Intervals[self.Intervals.len() - 1].From
            == self.Intervals[self.Intervals.len() - 1].To
        {
            self.Intervals.pop();
        }
    }
    pub fn BestEnd(&self, UC: &suc::SUC) -> f64 {
        let test = self.Intervals.iter().filter(|e| e.From <= UC.SD).fold(
            f64::INFINITY,
            |minimum, elem| {
                if minimum <= elem.ValueMinimumRestriced(UC.SD) {
                    minimum
                } else {
                    elem.ValueMinimumRestriced(UC.SD)
                }
            },
        );
        return test;
    }
    pub fn NextPoints(&mut self, UC: &suc::SUC) {
        // self.Print();

        let Index = self.GetOptimalNode();
        let pStar = self.Intervals[Index].MinimumAtInterval();
        let To = self.Intervals[Index].To.clone();
        self.Intervals[Index].To = pStar;
        let midInterval = qi::QuadraticInterval {
            From: f64::max(pStar - UC.RampDown, UC.pMin),
            To: f64::min(pStar + UC.RampUp, UC.pMax),
            A: self.Intervals[Index].ValueMinimum(),
            B: 0.0,
            C: 0.0,
            ZID: self.StartIndex.clone(),
        };
        self.Intervals.insert(Index + 1, midInterval);
        let mut rightInterval = self.Intervals[Index].CustomClone();
        rightInterval.From = pStar;
        rightInterval.To = To;
        self.Intervals.insert(Index + 2, rightInterval);
        self.ShiftLeft(Index, UC);
        self.ShiftRight(Index + 2, UC);
        self.Trim();
    }
}
pub fn First(startIndex: usize, startCost: f64, UC: &suc::SUC) -> F {
    let mut f = F {
        StartIndex: startIndex,
        Intervals: Vec::new(),
    };

    if f.StartIndex == 0 {
        let q = qi::QuadraticInterval {
            From: f64::from(UC.pMin),
            To: f64::from(UC.pMax),
            A: 0.0,
            B: 0.0,
            C: 0.0,
            ZID: f.StartIndex.clone(),
        };
        f.Intervals.push(q);
    } else {
        let q = qi::QuadraticInterval {
            From: f64::from(UC.pMin),
            To: f64::from(UC.SU),
            A: startCost,
            B: 0.0,
            C: 0.0,
            ZID: f.StartIndex.clone(),
        };
        f.Intervals.push(q);
    }
    f.IncreasePoints(startIndex, UC);
    f
}
