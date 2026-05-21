use crate::{node, syncable::Syncable};

pub struct Children {
	ids: Vec<node::ID>
}

pub enum ChildrenEvent {
	SetTo(Children)
}

pub enum ChildrenMerge {
	SetTo(Children)
}

pub enum ChildrenAutoMerge {
	SameChildren
}

impl Syncable for Children {
	type Event = ChildrenEvent;
	type Merge = ChildrenMerge;
	type AutoMerge = ChildrenAutoMerge;
}
