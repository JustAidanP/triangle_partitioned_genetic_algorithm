pub trait Member {
    type FitnessMetadata;
    type BreedMetadata;

    /// Obtains the fitness of the member, this is only requested once per generation
    fn fitness(&self, metadata: &Self::FitnessMetadata) -> u64;

    /// Breeds two members into a new member, also applies mutations
    fn breed(left: &Self, right: &Self, metadata: &Self::BreedMetadata) -> Self;
}
