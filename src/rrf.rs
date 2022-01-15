use crate::f;
use crate::suc;
pub struct RRF {
    pub Fs: Vec<Vec<f::F>>,
    pub UC: suc::SUC,
    pub stop: Vec<Vec<f64>>,
}

impl RRF {
    pub fn AddNew(&mut self, h: usize, startCost: f64) {
        let f = f::First(h, startCost, &self.UC);
        //f.Print();
        self.Fs[h].push(f);
    }

    pub fn Update(&mut self, t: usize) {
        let mut last_vec: Vec<f::F> = Vec::new();
        for elem in self.Fs[t - 1].iter_mut() {
            let mut f = elem.clone();
            f.NextPoints(&self.UC);
            f.IncreasePoints(t, &self.UC);
            last_vec.push(f);
        }
        self.Fs.push(last_vec);
    }
    pub fn GetBestStop(&mut self, h: usize) -> f64 {
        let mut bestStop = f64::INFINITY;
        for elem in self.Fs[h - 1].iter() {
            if (h - elem.StartIndex) >= self.UC.MinUpTime || elem.StartIndex == 0 {
                bestStop = f64::min(elem.BestEnd(&self.UC), bestStop);
            }
        }
        return bestStop;
    }
    pub fn BestValue(&mut self, h: usize) -> f64 {
        let mut bestValue = f64::INFINITY;
        for elem in self.Fs[h].iter() {
            bestValue = f64::min(bestValue, elem.BestValue());
        }
        return bestValue;
    }
    //  void RRF::Print(int t) {
    //     for (size_t i = 0; i < Fs[t].size(); i++)
    //     {
    //         //cout << "F" << i << endl;
    //         Fs[t][i].Print();
    //     }
    // }
    pub fn FillInDP(&mut self) {
        self.Fs.push(Vec::new());

        let mut newStop: Vec<f64> = Vec::new();
        for _ in 0..self.UC.MinDownTime {
            newStop.push(0.0);
        }
        self.stop.push(newStop);
        self.AddNew(0, self.UC.StartCost);
        for h in 1..self.UC.TotalTime {
            //println!("{}",h);
            let mut newStop: Vec<f64> = Vec::new();
            newStop.push(self.GetBestStop(h));
            for t in 1..self.UC.MinDownTime - 1 {
                newStop.push(self.stop[h - 1][t - 1]);
            }
            newStop.push(f64::min(
                self.stop[h - 1][self.UC.MinDownTime - 2],
                self.stop[h - 1][self.UC.MinDownTime - 1],
            ));
            self.stop.push(newStop);
            self.Update(h);
            let bestStart = f64::min(
                self.UC.StartCost,
                self.UC.StartCost + self.stop[h - 1][self.UC.MinDownTime - 1],
            );
            self.AddNew(h, bestStart);

        }

    }
    pub fn GetScore(&mut self) -> f64 {
        self.FillInDP();
        let mut bestValue = 0.0;
        for tau in 0..self.UC.MinDownTime {
            bestValue = f64::min(bestValue, self.stop[self.UC.LagrangeMultipliers.len() - 1][tau]);
        }
        bestValue = f64::min(bestValue, self.BestValue(self.UC.LagrangeMultipliers.len() - 1));
        return bestValue;
    }
}
