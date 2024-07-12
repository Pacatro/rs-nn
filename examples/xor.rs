use ndarray::array;
use rs_nn::{
    NN,
    ActivationType,
    Cost,
    layers::{Activation, Dense}
};

fn main() {
    let train_data = array![
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];

    let labels = array![
        [0.0],
        [1.0],
        [1.0],
        [0.0],
    ];

    let mut nn = NN::new()
        .add(Dense::new(2, 3))
        .add(Activation::new(ActivationType::TANH))
        .add(Dense::new(3, 1))
        .add(Activation::new(ActivationType::TANH));

    nn.train(Cost::MSE, &train_data, &labels, 300, 0.1, true);

    for input in train_data.rows() {
        let pred = nn.predict(&input.to_owned());
        let out = if pred[(0, 0)] < 0.5 { 0 } else { 1 };
        println!("{} --> {}", input, out)
    }
}