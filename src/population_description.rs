use super::population_changes::Update;

pub trait State{}

#[derive(Clone)]
pub struct PopulationState{
     population: f64,
    population_vector : Vec<u32>,
}

impl State for PopulationState{}

impl PopulationState {

   fn new(population: f64, population_vector: Vec<u32>) -> PopulationState {
        PopulationState{
            population,
            population_vector,
        }
    }


   pub fn create_from_states(states: Vec<u32>) -> PopulationState{
       let population: u32 = states.iter().sum();
       PopulationState::new(population as f64, states)
   }

    pub fn get_cardinality(&self, i: usize) -> u32 {
        *get_with_handle(self.population_vector.get(i))

    }

    pub fn get_fraction(&self, i: usize) -> f64{
       let res = self.get_cardinality(i);
        res as f64 / self.population
    }

    pub fn get_population_vector(&self) -> &Vec<u32>{
        &self.population_vector
    }

    pub fn update_population_state(self, update: Update) -> PopulationState{
        let mut new_population_vector = self.population_vector.clone();
        let mut population = self.population;
        for (index, value) in update.get_updates(){
            let new_value = *get_with_handle(new_population_vector.get(*index as usize)) as i32 + *value;
            match new_value {
                x if x < 0 => panic!("Dopo l'update la popolazione risultante per una data specie è sotto lo 0"),
                _ => {
                    new_population_vector[*index as usize] = new_value as u32;
                    population += *value as f64;
                }
            }
        }
        PopulationState::new(population, new_population_vector)
        }
    }

pub struct Population{
    index: usize,
    size: u32,
}

impl Population{
    pub fn new(index: usize) -> Population{
        Population{
            index,
            size : 1,
        }
    }
    pub fn get_size(&self) -> u32{
        self.size
    }
    pub fn get_index(&self)-> usize{
        self.index
    }
}

    fn get_with_handle<T>(res: Option<&T> ) -> &T {
        match res {
            Some(res) => res,
            None => panic!("index out of bound"),
        }
    }






