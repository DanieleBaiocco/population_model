
pub trait WeightedStructure<T> : Clone{

    fn get_total_weight(&self) -> f64;

    //obbligato a mettere la reference a self, perchè il compilatore
    // non sa quanto è grande self essendo un trait object
    fn select(&self, w: f64)-> Option<WeightedElement<T>>;

    //idem qua
    fn add(&self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>>;

//forse aggiungere il getAll
}

pub struct WeightedElement<T>{
    w: f64,
    t: T,
}
impl <T: Clone> WeightedElement<T>{
    fn new(w: f64, t: T) -> WeightedElement<T>{
        WeightedElement{
            w,
            t
        }
    }
    fn residual(&self, nw: f64) -> WeightedElement<T>{
        let w = self.w - nw;
        let t = self.t.clone();
        WeightedElement::new(w,t)
    }
}
impl <T: Clone> Clone for WeightedElement<T>{
    fn clone(&self) -> Self {
        let w = self.w;
        let t = self.t.clone();
        WeightedElement::new(w,t)
    }
}
impl <T: 'static + Clone> WeightedStructure<T> for WeightedElement<T>{
    fn get_total_weight(&self) -> f64 {
        self.w
    }

    fn select(&self, w: f64) -> Option<WeightedElement<T>> {
        if w <= self.w {
            let res = self.clone();
            return Some(res)
        };
        None
    }

    fn add(&self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
        let we = self.clone();
        Box::new(ComposedWeightedStructure::new(
            Some(Box::new(we)), ws))
    }
}

pub struct WeightedVector<T>{
    total_weight: f64,
    vec: Vec<WeightedElement<T>>,
}
impl <T> WeightedVector<T>{
    fn new(total_weight: f64, vec: Vec<WeightedElement<T>>) -> WeightedVector<T>{
        WeightedVector{
            total_weight,
            vec,
        }
    }
}
impl <T: Clone> Clone for WeightedVector<T>{
    fn clone(&self) -> Self {
        let mut v = Vec::new();
        for el in self.vec.iter(){
            let t = el.t.clone();
            let we = WeightedElement::new(el.w, t);
            v.push(we);
        }
        let wv = WeightedVector::new(self.total_weight, v);
    }
}
impl <T: 'static + Clone> WeightedStructure<T> for WeightedVector<T>{
    fn get_total_weight(&self) -> f64 {
        self.total_weight
    }

    fn select(&self, w: f64) -> Option<WeightedElement<T>> {
        let mut total = 0.0;
        for x in self.vec.iter() {
            total += x.get_total_weight();
            if w <= total {
                return Some(x.residual(w))
            }
        }
        None
    }


    fn add(&self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
       let wv = self.clone();
        if let None = ws {

            return Box::new(wv)
        }
        Box::new(ComposedWeightedStructure::new(
            Some(Box::new(wv)), ws))
    }
    //aggiungere l'add presente nel PopulationModel
}

pub struct ComposedWeightedStructure<T> {
    total_weight : f64,
    left: Option<Box<dyn WeightedStructure<T>>>,
    right: Option<Box<dyn WeightedStructure<T>>>,
}
impl <T> ComposedWeightedStructure<T>{
    //non capisco perche è moved
    fn new(left: Option<Box<dyn WeightedStructure<T>>>,
           right: Option<Box<dyn WeightedStructure<T>>>)-> ComposedWeightedStructure<T> {
        let mut total_weight = 0.0;
        //devo mettere some e none?
        if let Some(ref l) = left {total_weight += l.get_total_weight();};
        if let Some(ref r) = right { total_weight += r.get_total_weight();};
       ComposedWeightedStructure{
           total_weight,
           left,
           right,
        }
    }

}
impl <T> Clone for ComposedWeightedStructure<T>{
    fn clone(&self) -> Self {
        let left = self.left.clone();
        let right = self.right.clone();
        ComposedWeightedStructure::new(left,right)
    }
}
impl <T: 'static> WeightedStructure<T> for ComposedWeightedStructure<T>{
    fn get_total_weight(&self) -> f64 {
        self.total_weight
    }

    //non capisco questa parte
    fn select(& self, w: f64) -> Option<WeightedElement<T>> {
        if self.total_weight == 0.0 {
            return None
        }
        if let Some(ref l) =  self.left{
            if w < l.get_total_weight(){
                return l.select(w)
            }
        }
        if let Some(ref r) = self.right{
            return r.select(w-r.get_total_weight())
        }
        None
    }

        fn add(& mut self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
             match ws {
                None => {return Box::new(self.clone())}
                Some(res) => {
                    let increment = res.get_total_weight();
                    let mut left = self.left.unwrap_or_else(||{
                        return res
                    });
                    let mut right = self.right.unwrap_or_else(||{
                        self.right = Some(res);
                        self.total_weight += increment;
                        return Box::new(self.clone())
                    });
                    if increment >= left.get_total_weight() &&
                        increment >= right.get_total_weight(){
                        return Box::new(ComposedWeightedStructure::new(
                            Some(Box::new(self.clone())), Some(res)))
                    }
                    if left.get_total_weight() < right.get_total_weight(){
                        left = left.add(Some(res));
                    }
                    else {
                        right = right.add(Some(res));
                    }
                    self.total_weight += increment;
                    Box::new(self.clone())
                }
            }
        }
}