extern crate ndarray;

use self::ndarray::{Array1, Array2, Axis};

pub struct LogisticRegression {
    weights: Array1<f32>,
    learning_rate: f32,
    learning_rate_reduction_rate: f32,
}

#[derive(Debug)]
pub struct LogisticRegressionStatics {
    pub count: usize,
    pub correct_count: usize,
    pub actual_positive_count: usize,
    pub predict_positive_count: usize,
    pub correct_positive_count: usize,
}

impl LogisticRegressionStatics {
    pub fn new() -> Self {
        LogisticRegressionStatics {
            count: 0,
            correct_count: 0,
            actual_positive_count: 0,
            predict_positive_count: 0,
            correct_positive_count: 0,
        }
    }

    pub fn precision_rate(&self) -> f32 {
        self.correct_positive_count as f32 / self.predict_positive_count as f32
    }

    pub fn recall_rate(&self) -> f32 {
        self.correct_positive_count as f32 / self.actual_positive_count as f32
    }

    pub fn correct_rate(&self) -> f32 {
        (self.correct_count as f32) / (self.count as f32)
    }

    pub fn f_value(&self) -> f32 {
        let precision_rate = self.precision_rate();
        let recall_rate = self.recall_rate();
        (2f32 * precision_rate * recall_rate) / (precision_rate + recall_rate)
    }

    pub fn add_statics(mut self, statics: &LogisticRegressionStatics) -> Self {
        self.count += statics.count;
        self.correct_count += statics.correct_count;
        self.actual_positive_count += statics.actual_positive_count;
        self.predict_positive_count += statics.predict_positive_count;
        self.correct_positive_count += statics.correct_positive_count;
        self
    }
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

    pub fn get_weights(&self) -> Array1<f32> {
        self.weights.clone()
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
                println!("laeraning {}, cost {} learning_rate {}", i, self.cost(features_vec, answers), self.learning_rate);
            }
        }
        println!("end learning");
        self
    }

    pub fn get_statics(&self, features_vec: &Array2<f32>, answers: &Array1<f32>)
        -> LogisticRegressionStatics {
        let mut statics = LogisticRegressionStatics::new();
        let predict_answers = self.predict(features_vec);
        for (predict, answer) in predict_answers.iter().zip(answers.iter()) {
            let predict_answer = if *predict > 0.5f32 { 1f32 } else { 0f32 };
            statics.count += 1;
            let correct = predict_answer == *answer;
            let positive = *answer == 1f32;
            let predict_positive = predict_answer == 1f32;
            if correct { statics.correct_count += 1; }
            if positive { statics.actual_positive_count += 1; }
            if predict_positive { statics.predict_positive_count += 1; }
            if correct && predict_positive { statics.correct_positive_count += 1; }
        }
        statics
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
            learning_rate: 0.9f32,
            learning_rate_reduction_rate: 0.9999f32,
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