use std::result::Result;
use std::boxed::Box;

use crate::tree::{Node};
use crate::concurrent::{Commit};

type DynCommit = Commit<Box<dyn Node>>;
