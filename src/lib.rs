pub use crate::population_changes::PopulationRule;
pub use crate::population_changes::ReactionRule;
pub use crate::population_description::Population;
use crate::population_structure::{PopulationVector, PopulationElement};
use crate::population_changes::StepFunction;
pub use crate::population_description::PopulationState;
use rand::rngs::ThreadRng;
use rand::{RngCore, Rng};

mod population_description;
mod population_changes;
mod population_structure;

pub struct PopulationModel{
    rules : Vec<Box<dyn PopulationRule>>,
}
impl PopulationModel{
    pub fn new( rules : Vec<Box<dyn PopulationRule>>) -> PopulationModel{
        PopulationModel{
            rules,
        }
    }

    pub fn get_transitions(&self, pop_state: PopulationState) -> PopulationVector<StepFunction<PopulationState>> {
        let mut pv: PopulationVector<StepFunction<PopulationState>> = PopulationVector::init();
        for population_rule  in self.rules.iter(){
            let pop_state_to_move = pop_state.clone();
            let op_update = population_rule.apply(&pop_state);
            if let Some(update) = op_update {
                let rate = update.get_rate();
                let fnc = Box::new(||{
                    pop_state_to_move.update_population_state(update)
                });
                let step_function :StepFunction<PopulationState> = StepFunction::new(fnc);
                let pe = PopulationElement::new(rate,step_function);
                pv.add(pe)
            }
        }
        pv
    }
    pub fn next(&self, rg: &mut ThreadRng, pop_state: PopulationState) -> Option<PopulationState>{
        let transitions =self.get_transitions(pop_state);
        let total_weight =  transitions.get_total_weight();
        if total_weight == 0.0{
            return None
        }
        let select = rg.gen::<f64>() * total_weight;
        let we = transitions.select(select);
        return match we {
            None => { None }
            Some(we_inside) => {
                let we_inside =we_inside.get_t();
                Some(we_inside.step())
            }
        }
    }

}