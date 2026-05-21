use crate::syncable::Syncable;

pub enum Content {
	Text(String),
	Image,
	Other
}

pub enum ContentEvent {
	SetTo(Content)
}

pub enum ContentMerge {
	SetTo(Content)
}

pub enum ContentAutoMerge {
	SameContent
}

impl Syncable for Content {
	type Event = ContentEvent;
	type Merge = ContentMerge;
	type AutoMerge = ContentAutoMerge;
}
