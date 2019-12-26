use crate::tree::{Pair, MerkleError};

trait StaticTraversal<P, C> {
    fn traverse(anchor: P) -> C;
}

trait Traversal<P, C> {
    fn traverse(&self, anchor: P) -> Result<C, MerkleError::NavigationError>;
}

impl<P, C> Traversal<P, C> for dyn StaticTraversal<P, C> {
    fn traverse(&self, anchor: P) -> Result<C, MerkleError::NavigationError> {
        Ok(Self::traverse(anchor))
    }
}

struct Left<P, C> {}

impl<P: Pair<C>, C> StaticTraversal<P, C> for Left<P, C> {
    fn traverse(anchor: P) -> C {
        anchor.left()
    }
}

struct Right<P, C> {}

impl<P: Pair<C>, C> StaticTraversal<P, C> for Right<P, C> {
    fn traverse(anchor: P) -> C {
        anchor.right()
    }
}

struct Noop<P> {}

impl<P> StaticTraversal<P, P> for Noop<P> {
    fn traverse(anchor: P) -> P {
        anchor
    }
}

// TODO: macro to derive static traversal for a constant gindex

