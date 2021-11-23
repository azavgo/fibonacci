use goldenratio::GoldenRatio;  

pub struct Fibonacci {
    n: usize, // Fibonacci sequence number starts from 0, i.e. 0, 1, 2, 3, 4 ...
    x: usize,
}

impl Fibonacci {
    // Calculates Fibonacci number at position n in the Fibonacci sequence
    pub fn new(n: usize) -> Self {
        let fi = (GoldenRatio::gr_const()).fi();  
        let x = ((fi.powi(n.try_into().unwrap()) - (1.0 - fi).powi(n.try_into().unwrap()))/5.0_f64.sqrt()) as usize;
        Self {n: n, x: x}
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn sequence(&self) -> Vec<usize> {
        let mut s: Vec<usize> = Vec::new(); 
        for _i in 0..self.n() {
            s.push((Fibonacci::new(_i)).x());
        }    
        s
    }
}
