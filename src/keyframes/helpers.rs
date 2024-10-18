use crate::keyframes::Cards;
use crate::keyframes::Toggler;

use crate::MovementType;

/// Create a toggler keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn toggler(at: impl Into<MovementType>) -> Toggler {
    Toggler::new(at)
}

/// Create a cards keyframe.
/// Needs to be added into a chain. See [`crate::chain!`] macro.
pub fn cards(at: impl Into<MovementType>) -> Cards {
    Cards::new(at)
}

/// A slightly different import to clean up makeing lazy keyframes.
pub mod lazy {
    use crate::keyframes::Cards;
    use crate::keyframes::Toggler;
    use crate::MovementType;

    /// Create a lazy toggler keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn toggler(at: impl Into<MovementType>) -> Toggler {
        Toggler::lazy(at)
    }

    /// Create a lazy toggler keyframe.
    /// Needs to be added into a chain. See [`crate::chain!`] macro.
    pub fn cards(at: impl Into<MovementType>) -> Cards {
        Cards::lazy(at)
    }
}

/// A slightly different import to clean up makeing animation Ids.
pub mod id {
    pub use crate::keyframes::cards::Id as Cards;
    pub use crate::keyframes::toggler::Id as Toggler;
}

/// Direct access to `Chain`s for widget that may return an animation
/// in a message.
pub mod chain {
    pub use crate::keyframes::cards::Chain as Cards;
    pub use crate::keyframes::toggler::Chain as Toggler;
}
