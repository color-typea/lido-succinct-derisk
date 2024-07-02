pub mod hashing;
pub mod ssz;

pub mod utils {

    pub fn pad_zeroes<const A: usize, const B: usize>(arr: &[u8; A]) -> [u8; B] {
        assert!(B >= A); //just for a nicer error message, adding #[track_caller] to the function may also be desirable
        let mut b: [u8; B] = [0; B];
        b[..A].copy_from_slice(arr);
        b
    }

    use std::convert::AsMut;

    pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where
        A: Sized + Default + AsMut<[T]>,
        T: Clone,
    {
        let mut a: A = Default::default();
        <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
        a
    }
}
