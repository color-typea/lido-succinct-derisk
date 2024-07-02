pub type HashElement = [u8; 32];
pub type HashFunction = dyn Fn(&HashElement, &HashElement) -> HashElement;

pub const MAX_TREE_HEIGHT: usize = 40;
pub type Zerohashes = [HashElement; MAX_TREE_HEIGHT];
