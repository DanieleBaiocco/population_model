

pub trait WeightedStructure<T> {

    fn cln(&self) ->Box<dyn WeightedStructure<T>>;
    fn get_total_weight(&self) -> f64;

    //obbligato a mettere la reference a self, perchè il compilatore
    // non sa quanto è grande self essendo un trait object
    fn select(&self, w: f64)-> Option<WeightedElement<T>>;

    //idem qua
    fn add(&mut self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>>;

//forse aggiungere il getAll
}

pub struct WeightedElement<T>{
    w: f64,
    t: T,
}
impl <T: Clone> WeightedElement<T>{
    pub fn new(w: f64, t: T) -> WeightedElement<T>{
        WeightedElement{
            w,
            t
        }
    }
    pub fn residual(&self, nw: f64) -> WeightedElement<T>{
        let w = self.w - nw;
        let t = self.t.clone();
        WeightedElement::new(w,t)
    }
    fn cln_as_weighted_item(&self) -> Box<WeightedElement<T>>{
        let w = self.w;
        let t = self.t.clone();
        Box::new(WeightedElement::new(w,t ))
    }
}

impl <T: 'static + Clone> WeightedStructure<T> for WeightedElement<T>{
    fn cln(&self) -> Box<dyn WeightedStructure<T>> {
        let w = self.w;
        let t = self.t.clone();
        Box::new(WeightedElement::new(w,t ))
    }

    fn get_total_weight(&self) -> f64 {
        self.w
    }

    fn select(&self, w: f64) -> Option<WeightedElement<T>> {
        if w <= self.w {
            let res = self.cln_as_weighted_item();

            return Some(*res)
        };
        None
    }

    fn add(&mut self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
        let we = self.cln();
        Box::new(ComposedWeightedStructure::new(
            Some(we), ws))
    }
}

pub struct WeightedVector<T>{
    total_weight: f64,
    vec: Vec<WeightedElement<T>>,
}
impl <T> WeightedVector<T>{
    pub fn new(total_weight: f64, vec: Vec<WeightedElement<T>>) -> WeightedVector<T>{
        WeightedVector{
            total_weight,
            vec,
        }
    }
}
impl <T: 'static + Clone> WeightedStructure<T> for WeightedVector<T>{
    fn cln(&self) -> Box<dyn WeightedStructure<T>> {
        let mut v = Vec::new();
        for el in self.vec.iter(){
            let t = el.t.clone();
            let we = WeightedElement::new(el.w, t);
            v.push(we);
        }
         Box::new(WeightedVector::new(self.total_weight, v))
    }

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


    fn add(&mut self, ws: Option<Box<dyn WeightedStructure<T>>>) -> Box<dyn WeightedStructure<T>> {
       let wv = self.cln();
        if let None = ws {
            return wv
        }
        Box::new(ComposedWeightedStructure::new(
            Some(wv), ws))
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
    pub fn new(left: Option<Box<dyn WeightedStructure<T>>>,
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
    fn set_left (&mut self, left: Option<Box<dyn WeightedStructure<T>>>){
        self.left = left
    }
    fn set_right(&mut self, right: Option<Box<dyn WeightedStructure<T>>>){
        self.right = right
    }

}
impl <T: 'static> WeightedStructure<T> for ComposedWeightedStructure<T>{

    fn cln(&self) -> Box<dyn WeightedStructure<T>> {
        let left = match self.left{
            None => { None }
            Some(ref res) => { Some(res.cln())}
        };
        let right = match self.right {
            None => { None }
            Some(ref res) => { Some (res.cln())}
        };
        Box::new(ComposedWeightedStructure::new(left,right))
    }

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
                None => {return self.cln()}
                Some(res) => {
                    let increment = res.get_total_weight();
                    if let None = self.left{
                        return res;
                    }
                    if let None = self.right{
                        self.right = Some(res);
                        self.total_weight += increment;
                        return self.cln()
                    }
                    let mut cln_l = self.left.as_ref().unwrap().cln();
                    let mut cln_r = self.right.as_ref().unwrap().cln();
                    if increment >= cln_l.get_total_weight() && increment >= cln_r.get_total_weight(){
                        return Box::new(ComposedWeightedStructure::new(Some(self.cln()), Some(res)))
                    }
                    if cln_l.get_total_weight() < cln_r.get_total_weight(){
                        self.left = Some(cln_l.add(Some(res)));
                    }
                    else {
                        self.right = Some (cln_r.add(Some(res)));
                    }
                    self.total_weight += increment;
                    self.cln()
                }
            }
        }
}