//file that it will have all population logical rules such as complain generation, population generate, demography and many others.

use super::MemoryData;

struct Person {
    id: String,
    birthdate: String,
    name: String,
    gender: String
}

struct Complain {
    id: String,
    title: String,
    description: String,
    author: Person,
    date: String
}

pub fn generate_population(memory_data: &MemoryData) {

    //TODO get the birthrate and mortality rate.
    //TODO verify if there is a policy about the controll of birth
    //TODO apply the calculation of birthrate and mortality rate.
    //TODO return an object with the new habitants and who was dead. In addition to it, the new demograph

}

pub fn generate_complainings(memory_data: &MemoryData) {
    
}