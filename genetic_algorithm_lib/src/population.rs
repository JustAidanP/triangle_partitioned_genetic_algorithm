use crate::Member;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::borrow::Borrow;

pub struct Population<M: Member + Clone, const C: usize> {
    /// The members, M, of the population with the calculated fitness of each member,
    /// there are C members in the population.
    /// This is sorted by fitness, smallest first
    members: [M; C],
}

impl<M: Member + Clone, const C: usize> Population<M, C> {
    pub fn new(initial: [M; C]) -> Population<M, C> {
        assert!(
            C > 1,
            "There should be at least 2 members of the population"
        );
        Population { members: initial }
    }

    pub fn run(self: Self, metadata: &M::FitnessMetadata) -> EvaluatedPopulation<M, C> {
        // Gets the fitness of each member
        let mut members = self.members.map(|m| {
            let f = m.fitness(metadata);
            (m, f)
        });
        // Sorts by fitness
        members.sort_by(|(_, left), (_, right)| left.cmp(right));
        EvaluatedPopulation {
            member_fitness: members,
        }
    }
}

pub struct EvaluatedPopulation<M: Member + Clone, const C: usize> {
    /// The members, M, of the population with the calculated fitness of each member,
    /// there are C members in the population.
    /// This is sorted by fitness, smallest first
    member_fitness: [(M, u64); C],
}

impl<M: Member + Clone, const C: usize> EvaluatedPopulation<M, C> {
    /// Gets the best member of the population, along with its fitness
    pub fn best(&self) -> (&M, u64) {
        // Members is sorted, so best is the last element
        (
            &self.member_fitness[self.member_fitness.len() - 1].0,
            self.member_fitness[self.member_fitness.len() - 1].1,
        )
    }

    /// Gets the worst member of the population, along with its fitness
    pub fn worst(&self) -> (&M, u64) {
        // Members is sorted, so worst is the first element
        (&self.member_fitness[0].0, self.member_fitness[1].1)
    }

    /// Breeds the members of the population, according to their fitness, (natural selection),
    /// also applies random mutations to the members of the new population
    pub fn breed(mut self, metadata: &M::BreedMetadata) -> Population<M, C> {
        let mut rng = rand::thread_rng();
        let best = self.best().0;
        let best_fitness = self.best().1;
        let worst_fitness = self.worst().1;

        //Calculates the gradient to be used in the normalisation equation
        let gradient = 9.0 / (best_fitness - worst_fitness) as f64;
        let mut mating_pool = Vec::new();
        // Creates the mating pool
        for i in 0..(C - 1) {
            let (member, fitness) = &self.member_fitness[i];
            //Calculates the normalised fitness for the member, n = (f - s) * m + 10
            let normalised_fitness =
                (((fitness - best_fitness) as f64 * gradient + 1_f64) as u8).pow(2);
            // Adds the member into the mating pool based on its normalised fitness
            for i in 0..normalised_fitness {
                mating_pool.push(member);
            }
        }
        // Breeds the best member with elements from the pool
        let mut members =
            [0; C].map(|_| M::breed(best, mating_pool.choose(&mut rng).unwrap(), metadata));

        // The probability of selecting a member for breeding is proportional to its rank
        // let mut members = [0; C].map(|_| {
        //     // Picks two non-equal members, to breed, the selection is based on their fitness ranking
        //     // The proportional selection is based on the square of its rank, i.e. the worst member
        //     // is in the pool 1 time and the best member is in their C^2 times, or in other words,
        //     // the i'th member is in the selection pool i^2 times
        //     let choice = rng.gen_range(0..(C * (C + 1) * (2 * C + 1) / 6));
        //     let mut first = 0;
        //     // Finds the selected member
        //     for i in 0..C {
        //         // Checks if the choice is between the bounds of the current member's squared rank
        //         if (i * (i + 1) * (2 * i + 1) / 6) <= choice
        //             && choice < ((i + 1) * ((i + 1) + 1) * (2 * (i + 1) + 1) / 6)
        //         {
        //             first = i;
        //             break;
        //         }
        //     }
        //     // Chooses a second member
        //     let choice = rng.gen_range(0..((C - 1) * ((C - 1) + 1) * (2 * (C - 1) + 1) / 6));
        //     let mut second = 0;
        //     // Finds the selected member
        //     for i in 0..C {
        //         // Checks if the choice is between the bounds of the current member's squared rank
        //         if (i * (i + 1) * (2 * i + 1) / 6) <= choice
        //             && choice < ((i + 1) * ((i + 1) + 1) * (2 * (i + 1) + 1) / 6)
        //         {
        //             // Excludes our first choice
        //             second = if i >= first { i + 1 } else { i };
        //             break;
        //         }
        //     }
        //
        //     // Breeds the members together and mutates the resultant child a number of times
        //     M::breed(&self.members[first].0, &self.members[second].0)
        // });
        // // The first member is the best element
        // // members[0] = (*best).clone();

        Population { members }
    }
}
