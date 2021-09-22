
use rand::{thread_rng, Rng};
use tesina_population::{ PopulationState,  ReactionRule, Population};

fn main(){
    const s: usize = 0;
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

    fn generate_rules() -> Vec<Box<ReactionRule>>{
        let rule_s_i  = Box::new(ReactionRule::new(String::from("S -> I"),
                                                   vec![Population::new(s), Population::new(I)],
                                                   vec![Population::new(I), Population::new(I)],
                                                   |se|{
            let r = se.get_cardinality(s) as f64;
            let res= R as f64 * LAMBDA_MEET * PROB_TRANSMISSION * ((se.get_cardinality(I) as f64)/ N);
            res
        }));
        let rule_i_r = Box::new(ReactionRule::new(String::from("I -> R"),
                                                  vec![Population::new(I)],
                                                  vec![ Population::new(R)],
                                                  |se|{
                                             let r = se.get_cardinality(I) as f64;
                                             let res = R as f64 * LAMBDA_R;
                                             res
                                         }));
        let mut vect_to_return = Vec::new();
        vect_to_return.push(rule_s_i);
        vect_to_return.push(rule_i_r);
        vect_to_return

    }

    /*
     public final static int S = 0;
    public final static int I = 1;
    public final static int R = 2;

    public final static int SCALE = 100;
    public final static int INIT_S = 99 * SCALE;
    public final static int INIT_I = 1 * SCALE;
    public final static int INIT_R = 0 * SCALE;
    public final static double N = INIT_S + INIT_I + INIT_R;

    public final static double LAMBDA_MEET = 4;
    public final static double PROB_TRANSMISSION = 0.1;
    public final static double LAMBDA_R = 1 / 15.0;

    public static PopulationRegistry createPopulationRegistry(EvaluationEnvironment environment) {
        return PopulationRegistry.createRegistry("S", "I", "R");
    }

    public static PopulationState geneateState(double... args) {
        return new PopulationState(new int[] { INIT_S, INIT_I, INIT_R });
    }

    public static List<PopulationRule> generateRules(EvaluationEnvironment environment, PopulationRegistry registry) {
        PopulationRule rule_S_I = new ReactionRule("S->I", new Population[] { new Population(S), new Population(I) },
                new Population[] { new Population(I), new Population(I) },
                (t, s) -> s.getOccupancy(S) * PROB_TRANSMISSION * LAMBDA_MEET * (s.getOccupancy(I) / N));

        PopulationRule rule_I_R = new ReactionRule("I->R", new Population[] { new Population(I) },
                new Population[] { new Population(R) }, (t, s) -> s.getOccupancy(I) * LAMBDA_R);

        LinkedList<PopulationRule> rules = new LinkedList<>();
        rules.add(rule_S_I);
        rules.add(rule_I_R);
        return rules;
    }
     */

    let f: f64 = thread_rng().gen::<f64>();
    let fa: f64 = thread_rng().gen::<f64>();
    let faa: f64 = thread_rng().gen::<f64>();
    let faaaaa: f64 = thread_rng().gen::<f64>();

    println!("{:?}",f);
    println!("{:?}",fa);
    println!("{:?}",faaaaa);
    println!("{:?}",faa);
    println!("{:?}",thread_rng().gen_range(0..=1));

}