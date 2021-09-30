pub struct PopulationElement<T>{
    w: f64,
    t: T,
}

impl <T> PopulationElement<T>{
    pub fn new(w: f64, t: T) -> PopulationElement<T>{
        PopulationElement{
            w,
            t,
        }
    }
    pub fn get_w(&self) -> f64{
        self.w
    }
    pub fn get_t(self) -> T{
        self.t
    }

}

pub struct PopulationVector <T>{
    elements : Vec<PopulationElement<T>>,
    total_weight: f64,
}

impl <T> PopulationVector<T>{

    pub fn init() -> PopulationVector<T>{
        PopulationVector{
            elements : Vec::new(),
            total_weight: 0.0,
        }
    }
    pub fn new (elements: Vec<PopulationElement<T>>) -> PopulationVector<T>{
        let mut total_weight = 0.0;
        elements.iter().for_each(|elem| {
            total_weight += elem.get_w();
        });
        PopulationVector{
            elements,
            total_weight,
        }
    }

    pub fn add(&mut self, element: PopulationElement<T>){
        self.elements.push(element);
    }

    pub fn select(self, w: f64) -> Option<PopulationElement<T>>{
        let mut total = 0.0;
        for elem in self.elements {
            total += elem.w;
            if w <= total {
                return Some(elem)
            }
        }
        None
    }

    pub fn get_total_weight(&self) -> f64{
        self.total_weight
    }
}