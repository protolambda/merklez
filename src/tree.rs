use ring::digest::{digest, SHA256};

pub enum MerkleError {
    TypeInputError,
    InputLengthError,
    NavigationError,
}

pub type Root = [u8; 32];

pub const ZERO: Root = Root();

pub type Hasher = fn(a: Root, b: Root) -> Root;

pub trait Node: Clone {
    fn merkle_root(&self, h: Hasher) -> Root;
}

impl Node for Root {
    fn merkle_root(&self, h: Hasher) -> Root {
        *self
    }
}

pub fn default_merkle_fn(a: Root, b: Root) -> Root {
    let input = a + b;
    Root(digest(&SHA256, input).as_ref().into())
}

pub const ZERO_NODE: Root = Root();

pub const ZERO_NODES: [Root; 256] = {
    let mut roots = [Root; 256](0: Root());
    for i in 0..(roots.len()-1) {
        roots[i+1] = default_merkle_fn(roots[i], roots[i])
    }
    roots
};

pub trait Pair<T: Node> {
    fn pair(l: T, r: T) -> &Self;

    fn mono(e: T) -> &Self {
        Self::pair(e.clone(), e)
    }

    fn left(&self) -> T {
        self.left.clone()
    }

    fn right(&self) -> T {
        self.right.clone()
    }

    fn rebind_left(&self, l: T) -> &Self {
        Self::pair(l,self.right())
    }
    fn rebind_right(&self, r: T) -> &Self {
        Self::pair(self.left(), r)
    }

    fn subtree_fill_depth(bottom: T, depth: u8) -> &Self {
        let mut out = bottom;
        for i in 0..depth {
            out = Self::pair(out, out)
        }
        out
    }

    fn subtree_fill_length(bottom: T, depth: u8, length: u64) -> Result<&Self, MerkleError::TypeInputError> {
        let anchor = u64(1) << depth;
        if length > anchor {
            return Err(MerkleError::TypeInputError)
        }
        if length == anchor {
            return Ok(Self::subtree_fill_depth(bottom, depth))
        }
        if depth == 1 {
            if length > 1 {
                return Ok(Self::mono(bottom))
            }
        }
        let pivot = anchor >> 1;
        if length <= pivot {
            let left = Self::subtree_fill_length(bottom, depth-1, length)?;
            return Ok(Self::pair(left, ZERO))
        } else {
            let left = Self::subtree_fill_depth(bottom, depth-1)?;
            let right = Self::subtree_fill_length(bottom, depth-1, length-pivot)?;
            return Ok(Self::pair(left, right))
        }
    }

    fn subtree_fill_contents(nodes: &[T], depth: u8) -> Result<T, MerkleError> {
        if depth < 64 && nodes.len() > (usize(1) << depth) {
            return Err(MerkleError::TypeInputError);
        }
        return match (depth, nodes.len()) {
            (_, 0) => Ok(ZERO_NODES[depth]),
            (0, 1) => Ok(&nodes[0]),
            (1, 1) => Ok(Self::pair(&nodes[0], ZERO_NODES[depth])),
            (0, 2) => Err(MerkleError::InputLengthError),
            (1, 2) => Ok(Self::pair(&nodes[0], &nodes[1])),
            (0, _) | (1, _) => Err(MerkleError::InputLengthError),
            _ if depth < 64 => {
                let pivot = (u64(1) << depth) >> 1;
                if nodes.len() <= pivot {
                    let left = Self::subtree_fill_contents(nodes, depth-1)?;
                    Ok(Self::pair(left, ZERO_NODES[depth]))
                } else {
                    let left = Self::subtree_fill_contents(nodes[..pivot], depth-1)?;
                    let right = Self::subtree_fill_contents(nodes[pivot..], depth-1)?;
                    Ok(Self::pair(left, right));
                }
            }
            _ =>  {
                let left = Self::subtree_fill_contents(nodes, depth-1)?;
                Ok(Self::pair(left, ZERO_NODES[depth]));
            }
        };
    }
}
