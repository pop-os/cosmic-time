mod cards;
mod helpers;
mod toggler;

pub use cards::Cards;
pub use helpers::cards;
pub use helpers::id;
pub use helpers::lazy;
pub use helpers::{chain, toggler};
pub use toggler::Toggler;
/// The macro used to cleanly and efficently build an animation chain.
/// Works for ann Id's that implement `into_chain` and `into_chain_with_children`
#[macro_export]
macro_rules! chain{
  ($id:expr) => {
    $id.clone().into_chain()
  };
  ($id:expr, $($x:expr),+ $(,)?) => {
    $id.clone().into_chain_with_children(vec![$($x),+])
  };
}

/// The macro used to clean up animation's view code.
#[macro_export]
macro_rules! anim{
  ($id:expr, $($x:expr),+ $(,)?) => {
    $id.clone().as_widget($($x),+)
  };
}

/// Repeat Limit
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Repeat {
    /// Never repeat
    #[default]
    Never,
    /// Repeat forever
    Forever,
}
