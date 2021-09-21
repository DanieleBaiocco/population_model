use rand::prelude::ThreadRng;
use super::population_description::PopulationState;
use std::collections::HashMap;
use crate::population_description::{Population, State};


pub trait PopulationRule{
    /// se la regola non è applicabile ritorna None
    fn apply(&self, rg: ThreadRng, now: f64, population_state: PopulationState) -> Option<PopulationTransition>;
}

pub struct ReactionRule{
    name: String,
    reactans: Vec<Population>,
    products: Vec<Population>,
    //levato il now credo dagli input della fn
    rate_function: fn(PopulationState)-> f64,
    update: Update,
}

impl ReactionRule{
    pub fn new(name: String, reactans: Vec<Population>,
               products: Vec<Population>, rate_function: fn(PopulationState)->f64,
               mut update: Update) -> ReactionRule {
        reactans.iter().for_each(|r|{
            update.consume(&(r.get_index() as u32), r.get_size())
        });
        products.iter().for_each(|r|{
            update.produce(&(r.get_index() as u32), r.get_size())
        });
        ReactionRule {
            name,
            reactans,
            products,
            rate_function,
            update,
        }
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

/*
prendo l'ownership di population_state sull'esecuzione del rate_function
 */
impl PopulationRule for ReactionRule{
    fn apply(&self, rg: ThreadRng, now: f64, population_state: PopulationState) -> Option<PopulationTransition> {
        match self.is_enabled(&population_state) {
            true => {
                let rate = (self.rate_function)(population_state);
                if rate <= 0.0 { return None }
                let update = self.update.clone();
                let name = self.name.clone();
                //problema quaù
                let d = |tr: ThreadRng| { update};
                Some(PopulationTransition::new(Box::new(d) , rate, name))
            }
            false => { return None}
        }
    }
}

#[derive(Clone)]
pub struct Update{
    update: HashMap<u32, u32>,
    name: String,
}

impl Update{

    pub fn new(name: String) -> Update {
        Update{
            update: HashMap::new(),
            name,
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
                None => self.update.remove(i),
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
}


pub struct PopulationTransition{
    transition_drift_function: Box< dyn FnOnce(ThreadRng) -> Update>,
    rate: f64,
    name: String,
}

impl PopulationTransition{
    /*
    non so se è necessario mettere 'a
     */
    pub fn new (transition: Box<dyn FnOnce(ThreadRng) -> Update>,
            rate: f64,
            name: String) ->  PopulationTransition {
        PopulationTransition{
            transition_drift_function: transition,
            rate,
            name,
        }
    }
    pub fn get_name(&self) -> &String{
        &self.name
    }
    pub fn get_rate(&self) -> f64{
        self.rate
    }
    pub fn apply(self, tr: ThreadRng) -> Update{
        (self.transition_drift_function)(tr)
    }
}

/*
assomiglia alla functional interface
 */

pub struct StepFunction<S: State>{
    step: Box<dyn FnOnce(ThreadRng,f64, f64)->S>,

}
impl <S: State> StepFunction<S>{
     pub fn new ( step: Box<dyn FnOnce(ThreadRng,f64, f64)->S>) -> StepFunction<S>{
         StepFunction{
             step,
         }
    }
    pub fn step(self, rg: ThreadRng, now: f64, dt:  f64) -> S{
        (self.step)(rg, now, dt)
    }
}
impl <S: State> Clone for StepFunction<S>{
    fn clone(&self) -> Self {
        todo!()
    }
}
pub struct TimeStep<S: State> {
    time : f64,
    value: S,
}

impl <S: State> TimeStep<S>{
    pub fn new(time : f64, value: S)-> TimeStep<S>{
        TimeStep{
            time,
            value,
        }
    }
    pub fn get_time(&self) -> f64{
        self.time
    }
    pub fn get_value(&self) -> &S{
        &self.value
    }
}
