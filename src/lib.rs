use self::population_changes::PopulationRule;
use crate::weighted_structure_building::{WeightedStructure, WeightedVector, WeightedElement};
use crate::population_changes::{StepFunction, TimeStep};
use crate::population_description::PopulationState;
use rand::rngs::ThreadRng;
use rand::RngCore;
use std::borrow::{BorrowMut, Borrow};

mod population_description;
mod population_changes;
mod weighted_structure_building;

pub struct PopulationModel{
    rules : Vec<Box<dyn PopulationRule>>,
}
impl PopulationModel{
    pub fn new( rules : Vec<Box<dyn PopulationRule>>) -> PopulationModel{
        PopulationModel{
            rules,
        }
    }

    pub fn get_transitions(&self, rg: ThreadRng, now: f64, pop_state: PopulationState) -> Box<dyn WeightedStructure<StepFunction<PopulationState>>> {
        let mut wv: WeightedVector<StepFunction<PopulationState>> = WeightedVector::init();
        for population_rule  in self.rules.iter(){
            let pop_state1 = pop_state.clone();
            let pop_state2 = pop_state.clone();
            let rgc = rg.clone();
            let tra = population_rule.apply(rgc, now, pop_state1);
            if let Some(pop_tra) = tra {
                let rate = pop_tra.get_rate();
                let fnc = Box::new(|rg: ThreadRng, now: f64, dt: f64|{
                    pop_state2.update_population_state(pop_tra.apply(rg))
                });
                let step_function :StepFunction<PopulationState> = StepFunction::new(fnc);
                let we = WeightedElement::new(rate,step_function);
                wv.add_weighted_element(we)
            }
        }
        Box::new(wv)
    }
    pub fn next(&self, mut rg: ThreadRng, time : f64, pop_state: PopulationState) -> Option<TimeStep<PopulationState>>{
        let mut rgc = rg.clone();
        let transitions =self.get_transitions(rg, time, pop_state);
        let total_weight =  transitions.get_total_weight();
        if total_weight == 0.0{
            return None
        }
        //calcolo dt con funzione
        let dt =  PopulationModel::sample_exponential_distribution (total_weight, & mut rgc);
        let select = rgc.next_u64() as f64 * total_weight;
        let we = transitions.select(select);
        return match we {
            None => { None }
            Some(we_inside) => {
                let we_inside =we_inside.get_s().clone();
                Some(TimeStep::new(time, we_inside.step(rgc, time, dt)))
            }
        }
    }
    fn sample_exponential_distribution (total : f64, rg: &mut ThreadRng) -> f64{
        if total <= 0.0{
            panic!("Error")
        }
        let second_part = (1 as f64) /  (rg.next_u64() as f64);
        let log_second_part = second_part.log2();
        let result = (1.0 / total) * log_second_part;
        result
    }
}