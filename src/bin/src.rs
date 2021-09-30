
use tesina_population::{PopulationState, ReactionRule, Population, PopulationModel, PopulationRule};
use rand::rngs::ThreadRng;
use rand::RngCore;
fn main(){
    const S: usize = 0;
    const I: usize = 1;
    const R: usize = 2;

    const SCALE: u32 = 100;
    const INIT_S: u32 = 99 * SCALE;
    const INIT_I: u32 = 1 * SCALE;
    const INIT_R: u32 = 0 * SCALE;

    const N: f64 = (INIT_S + INIT_R + INIT_I) as f64;

    const LAMBDA_MEET: f64 = 4.0;
    const PROB_TRANSMISSION: f64 = 0.1;
    const LAMBDA_R: f64 = 1.0/15.0;

    fn generate_population_state() -> PopulationState{
        PopulationState::create_from_states(vec![INIT_S, INIT_I, INIT_R])
    }

    fn generate_rules() -> Vec<Box<dyn PopulationRule>>{
        let rule_s_i  = Box::new(ReactionRule::new(String::from("S -> I"),
                                                   vec![Population::new(S), Population::new(I)],
                                                   vec![Population::new(I), Population::new(I)],
                                                   |se|{
            let r = se.get_cardinality(S) as f64;
            let res= r  * LAMBDA_MEET * PROB_TRANSMISSION * ((se.get_cardinality(I) as f64)/ N);
            res
        }));
        let rule_i_r = Box::new(ReactionRule::new(String::from("I -> R"),
                                                  vec![Population::new(I)],
                                                  vec![ Population::new(R)],
                                                  |se|{
                                             let r = se.get_cardinality(I) as f64;
                                             let res = r * LAMBDA_R;
                                             res
                                         }));
        let mut vect_to_return :Vec<Box<dyn PopulationRule>> = Vec::new();
        vect_to_return.push(rule_s_i);
        vect_to_return.push(rule_i_r);
        vect_to_return

    }

    let pop_state = generate_population_state();
    let rules = generate_rules();
    let pop_model = PopulationModel::new(rules);
    let generatore = &mut ThreadRng::default();
    let ops =pop_model.next( generatore, pop_state);
    let ps = ops.unwrap();
    println!("ciao bello");
    /*
    for (index, el) in ps.get_population_vector().iter().enumerate(){
        println!("indice {:?}, numero {:?}", index, el);
    }*/

}