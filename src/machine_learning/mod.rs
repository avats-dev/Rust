mod cholesky;
mod k_means;
mod linear_regression;
mod loss_function;
mod optimization;
mod simpson;

pub use self::cholesky::cholesky;
pub use self::k_means::k_means;
pub use self::linear_regression::linear_regression;
pub use self::loss_function::mae_loss;
pub use self::loss_function::mse_loss;
pub use self::optimization::gradient_descent;
pub use self::optimization::Adam;
pub use self::simpson::simpsons_rule;
