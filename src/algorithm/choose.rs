use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

pub trait Chooser {
    fn choose<'a, T>(&self, choices: &'a [T], weights: &[f64]) -> Option<&'a T>;
}

#[derive(Default, Clone)]
pub struct StochasticChooser;

impl StochasticChooser {
    fn normalise(&self, weights: &[f64]) -> Vec<f64> {
        let mut weights = weights.iter().map(|a| 10f64.powf(*a)).collect::<Vec<_>>();

        let sum = weights.iter().sum::<f64>();

        for weight in &mut weights {
            *weight /= sum;
        }

        weights
    }
}

impl Chooser for StochasticChooser {
    fn choose<'a, T>(&self, choices: &'a [T], weights: &[f64]) -> Option<&'a T> {
        let weights = self.normalise(weights);

        let mut rng = thread_rng();
        let dist = WeightedIndex::new(weights).unwrap();

        choices.get(dist.sample(&mut rng))
    }
}

#[derive(Default, Clone)]
pub struct GreedyChooser;

impl Chooser for GreedyChooser {
    fn choose<'a, T>(&self, choices: &'a [T], weights: &[f64]) -> Option<&'a T> {
        choices
            .iter()
            .zip(weights.iter())
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(choice, _)| choice)
    }
}

fn sigmoid(x: &f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_greedy_chooser() {
        let chooser = GreedyChooser::default();

        let choices: Vec<_> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let weights = vec![0.0; 10];

        for i in 0..10 {
            for _ in 0..100 {
                let mut new_weights = weights.clone();
                new_weights[i] = 1.0;

                assert_eq!(chooser.choose(&choices, &new_weights), Some(&i));
            }
        }
    }

    #[test]
    fn test_stochastic_chooser() {
        let chooser = StochasticChooser::default();

        let choices: Vec<_> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let weights = vec![f64::NEG_INFINITY; 10];

        for i in 0..10 {
            for _ in 0..100 {
                let mut new_weights = weights.clone();
                new_weights[i] = 1.0;

                eprintln!("weights {:?}", new_weights);
                assert_eq!(chooser.choose(&choices, &new_weights), Some(&i));
            }
        }
    }
}
