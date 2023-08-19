pub struct Sample<V> {
    pub value: V,
    pub logpdf: f64
}

impl<V> Sample<V> {
    pub fn new(value: V) -> Self {
        Sample { value, logpdf: 0. }
    }
}

impl<V: Index<Addr> + Clone> Trace<V,V> for Sample<V> {
    fn get_data(&self) -> &V { &self.value }
    fn get_retv(&self) -> &V { &self.value }
    fn logpdf(&self) -> f64 { self.logpdf }
}
