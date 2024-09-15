pub mod util_traits {
    use std::hash::Hash;

    pub trait Graph {}


    pub trait NodeTrait: Copy + Ord + Hash + Send + Sync {}
    impl<N> NodeTrait for N where N: Copy + Ord + Hash + Send + Sync {}

}