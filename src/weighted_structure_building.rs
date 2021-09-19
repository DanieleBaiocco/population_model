
pub trait WeightedStructure<T>{

    fn get_total_weight(&self) -> f64;

    fn select(&self, w: f64)-> Option<&WeightedElement<T>>;

    fn add(&self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>>;

//forse aggiungere il getAll
}

pub struct WeightedElement<T>{
    w: f64,
    t: T,
}
impl <T : Clone> WeightedElement<T>{
    fn residual(&self, nw: f64) -> WeightedElement<T>{
        let w = self.w - nw;
        let t = self.t.clone();
        WeightedElement{
            w,
            t,
        }
    }
}
impl <T> WeightedStructure<T> for WeightedElement<T>{
    fn get_total_weight(&self) -> f64 {
        self.w
    }

    fn select(&self, w: f64) -> Option<&WeightedElement<T>> {
         match w <= self.w{
             true => { Some(&self) }
             false => { None }
         }
    }

    fn add(&self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
        Box::new(ComposedWeightedStructure::new(
            Some(Box::new(self)), ws))
    }
}

pub struct WeightedVector<T>{
    total_weight: f64,
    vec: Vec<WeightedElement<T>>,
}

impl <T> WeightedStructure<T> for WeightedVector<T>{
    fn get_total_weight(&self) -> f64 {
        self.total_weight
    }

    fn select(&self, w: f64) -> Option<&WeightedElement<T>> {
        let mut total = 0.0;
        for x in self.vec.iter() {
            total += x.get_total_weight();
            if w <= total {
                return Some(&x.residual(w))
            }
        }
        None
    }


    fn add(&self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
        if let None = ws {
            return Box::new(self)
        }
        Box::new(ComposedWeightedStructure::new(
            Some(Box::new(self)), ws))
    }
}

pub struct ComposedWeightedStructure<T>{
    total_weight : f64,
    left: Option<Box<dyn WeightedStructure<T>>>,
    right: Option<Box<dyn WeightedStructure<T>>>,
}
impl <T> ComposedWeightedStructure<T>{
    fn new(left: Option<Box<dyn WeightedStructure<T>>>,
           right: Option<Box<dyn WeightedStructure<T>>>)-> ComposedWeightedStructure<T> {
        let mut total_weight = 0.0;
        //devo mettere some e none?
        if let Some(_) = left {total_weight += left.get_total_weight();}
        if let Some(_) = right { total_weight += right.get_total_weight();}
       ComposedWeightedStructure{
            total_weight,
            left,
            right,
        }
    }

}
impl <T> WeightedStructure<T> for ComposedWeightedStructure<T>{
    fn get_total_weight(&self) -> f64 {
        self.total_weight
    }

    //non capisco questa parte
    fn select(&self, w: f64) -> Option<&WeightedElement<T>> {
        if self.total_weight == 0.0 {
            return None
        }
        if let None =  self.left{
            if w < self.left.get_total_weight() {
                return Some (&self.left.select(w))
            }
        }
        if let None = self.right{
            return Some(&self.right.select(w-self.left.get_total_weight()))
        }
        None
    }

    fn add(&mut self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
        match ws {
            None => {return Box::new(self)}
            Some(res) => {
                let increment = ws.get_total_weight();
                let mut left = self.left.unwrap_or_else(||{
                    return res
                });
                let mut right = self.right.unwrap_or_else(||{
                    self.right = Some(res);
                    self.total_weight += increment;
                    return Box::new(self)
                });
                if increment >= left.get_total_weight() &&
                    increment >= right.get_total_weight(){
                    return Box::new(ComposedWeightedStructure::new(
                        Some(Box::new(self)), Some(res)))
                }
                if left.get_total_weight() < right.get_total_weight(){
                    left = left.add(Some(res));
                }
                else {
                    right = right.add(Some(res));
                }
                self.total_weight += increment;
                Box::new(self)
            }
        }
    }
}