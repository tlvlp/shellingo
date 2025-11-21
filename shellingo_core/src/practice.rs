use rand::seq::SliceRandom;

pub fn get_random_index_sequence(vector_size: usize) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut indices = (0..vector_size as u16).collect::<Vec<u16>>();
    indices.shuffle(&mut rng);
    indices
}