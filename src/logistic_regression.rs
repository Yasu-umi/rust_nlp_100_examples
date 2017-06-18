extern crate ndarray;

use self::ndarray::{Array1, Array2, Axis};

pub struct LogisticRegression {
    weights: Array1<f32>,
    learning_rate: f32,
    learning_rate_reduction_rate: f32,
}

impl LogisticRegression {
    pub fn new(feature_len: usize, learning_rate: f32, learning_rate_reduction_rate: f32)
        -> Self {
        LogisticRegression {
            weights: Array1::<f32>::zeros((feature_len)),
            learning_rate: learning_rate,
            learning_rate_reduction_rate: learning_rate_reduction_rate,
        }
    }

    pub fn sigmoid(value: &f32) -> f32 {
        1f32 / (1f32 + (-value).exp())
    }

    pub fn predict<'a>(&self, features_vec: &Array2<f32>)
        -> Array1<f32> {
        Array1::from_iter(
            features_vec.dot(&self.weights)
                .iter()
                .map(LogisticRegression::sigmoid)
        )
    }

    pub fn update(&mut self, features_vec: &Array2<f32>, answers: &Array1<f32>)
        -> &mut Self {
        let predict_answers = self.predict(features_vec);
        let features_vec_len = features_vec.len_of(Axis(0)) as f32;
        let diff = predict_answers - answers;
        let theta = features_vec.t().dot(&diff) * (self.learning_rate / features_vec_len);
        self.weights = self.weights.clone() - theta;
        self
    }

    pub fn cost(&self, features_vec: &Array2<f32>, answers: &Array1<f32>)
        -> f32 {
        let predict_answers = self.predict(features_vec);
        let costs = (0f32 - answers) * Array1::from_iter(predict_answers.iter().map(|p| p.ln())) - (1f32 - answers) * Array1::from_iter((1f32 - predict_answers).iter().map(|p| p.ln()));
        costs.iter().fold(0f32, |p, c| p + c) / answers.len_of(Axis(0)) as f32
    }

    pub fn learn(&mut self, features_vec: &Array2<f32>, answers: &Array1<f32>, n: usize)
        -> &mut Self {
        println!("start learning");
        println!("laeraning 0, cost {}", self.cost(features_vec, answers));
        for i in 1..(n+1) {
            self.update(features_vec, answers);
            self.learning_rate = self.learning_rate * self.learning_rate_reduction_rate;
            if i % 10 == 0 {
                println!("laeraning {}, cost {}", i, self.cost(features_vec, answers));
            }
        }
        println!("end learning");
        self
    }
}

pub struct LogisticRegressionBuilder<FeatureLenType> {
    pub learning_rate: f32,
    pub learning_rate_reduction_rate: f32,
    pub feature_len: FeatureLenType,
}

impl LogisticRegressionBuilder<()> {
    pub fn new() -> Self {
        LogisticRegressionBuilder {
            learning_rate: 0.6f32,
            learning_rate_reduction_rate: 0.99f32,
            feature_len: (),
        }
    }
}

impl LogisticRegressionBuilder<usize> {
    pub fn build(self) -> LogisticRegression {
        LogisticRegression {
            learning_rate: self.learning_rate,
            learning_rate_reduction_rate: self.learning_rate_reduction_rate,
            weights: Array1::<f32>::zeros((self.feature_len)),
        }
    }
    pub fn learning_rate(self, learning_rate: f32) -> Self {
        LogisticRegressionBuilder {
            learning_rate: learning_rate,
            learning_rate_reduction_rate: self.learning_rate_reduction_rate,
            feature_len: self.feature_len,
        }
    }
    pub fn learning_rate_reduction_rate(self, learning_rate_reduction_rate: f32) -> Self {
        LogisticRegressionBuilder {
            learning_rate: self.learning_rate,
            learning_rate_reduction_rate: learning_rate_reduction_rate,
            feature_len: self.feature_len,
        }
    }
}

impl<FeatureLenType> LogisticRegressionBuilder<FeatureLenType> {
    pub fn feature_len(self, feature_len: usize) -> LogisticRegressionBuilder<usize> {
        LogisticRegressionBuilder {
            learning_rate: self.learning_rate,
            learning_rate_reduction_rate: self.learning_rate_reduction_rate,
            feature_len: feature_len,
        }
    }
}