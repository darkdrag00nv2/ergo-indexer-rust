/// A service providing an access to the Ergo network.
pub trait ErgoNetwork {
    /// Get height of the best block.
    fn get_best_height(&self) -> u64;
}
