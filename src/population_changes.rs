use super::population_description::PopulationState;
use std::collections::HashMap;
use crate::population_description::{Population, State};

pub trait PopulationRule{
    /// se la regola non è applicabile ritorna None
    fn apply(&self, population_state: &PopulationState) -> Option<Update>;
}

pub struct ReactionRule{
    name: String,
    reactans: Vec<Population>,
    rate_function: fn(&PopulationState)-> f64,
    update: Update,
}

impl ReactionRule{
    pub fn new(name: String, reactans: Vec<Population>,
               products: Vec<Population>, rate_function: fn(&PopulationState)->f64) -> ReactionRule {
        let name_for_update = name.clone();
        let update = ReactionRule::build_update(name_for_update, &reactans, products);
        ReactionRule {
            name,
            reactans,
            rate_function,
            update,
        }
    }

    fn build_update(name: String, reactans: &Vec<Population>, products: Vec<Population>) -> Update{
        let mut update_to_return = Update::new(name);
        reactans.iter().for_each(|r|{
            update_to_return.consume(&(r.get_index() as u32), r.get_size())
        });
        products.iter().for_each(|r|{
            update_to_return.produce(&(r.get_index() as u32), r.get_size())
        });
        update_to_return
    }

    fn is_enabled(&self, population_state: &PopulationState) -> bool{
        for r in self.reactans.iter() {
            if population_state.get_cardinality(r.get_index()) < r.get_size() {
                return false
            }
        }
        return true
    }
}


impl PopulationRule for ReactionRule{
    fn apply(&self, population_state: &PopulationState) -> Option<Update> {
        return match self.is_enabled(&population_state) {
            true => {
                let rate = (self.rate_function)(population_state);
                if rate <= 0.0 { return None }
                let mut update_to_return = self.update.clone();
                update_to_return.set_rate(rate);
                Some(update_to_return)
            }
            false => { None }
        }
    }
}

#[derive(Clone)]
pub struct Update{
    update: HashMap<u32, u32>,
    name: String,
    rate: f64,
}

impl Update{

    pub fn new(name: String) -> Update {
        Update{
            update: HashMap::new(),
            name,
            rate: 0.0,
        }
    }

    pub fn get_updates(&self) -> Vec<(&u32, &u32)> {
        let res: Vec<(&u32, &u32)> = self.update.iter().collect();
        res
    }

    pub fn add(&mut self, i: &u32, c: u32, p: u32){
        if c != p {
            let res =  self.update.get(i);
            match res {
                Some(result) => {
                    self.update.insert(*i, result + p - c)
                },
                None => self.update.insert(*i, p - c),
            };
        }
    }

    pub fn produce(&mut self, i: &u32, p: u32){
        self.add(i, 0, p);
    }

    pub fn consume(&mut self, i: &u32, c: u32){
        self.add(i, c, 0);
    }
    pub fn get_single_update(&self, i: &u32) -> &u32 {
        self.update.get(i).unwrap_or(&0)
    }

    pub fn set_rate(&mut self, rate:f64){
        self.rate = rate
    }
    pub fn get_rate(&self) -> f64{
        self.rate
    }
}

pub struct StepFunction<S: State>{
    step: Box<dyn FnOnce()->S>,
}
impl <S: State> StepFunction<S>{
     pub fn new ( step: Box<dyn FnOnce()->S>) -> StepFunction<S>{
         StepFunction{
             step,
         }
    }
    pub fn step(self) -> S{
        (self.step)()
    }
}


