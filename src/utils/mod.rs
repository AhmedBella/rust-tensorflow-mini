use crate::prelude::*;

//neural network
mod neural_network;
mod optimizer_SDG;
pub use neural_network::*;

// layer dense
mod layer_dense;
pub use layer_dense::*;

// activation relu
mod relu;
pub use relu::*;

// activation softmax
mod softmax;
pub use softmax::*;

// loss function
mod loss;
pub use loss::*;

// categorical cross entropy
mod categorical_cross_entropy;
pub use categorical_cross_entropy::*;

// optimizer

// stochastic gradient descent
mod optimizer_sdg;
pub use optimizer_sdg::*;

// data
mod data;
pub use data::*;

// visualize
mod visualize;
pub use visualize::*;