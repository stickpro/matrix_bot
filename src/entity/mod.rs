use crate::{
    error::ResourceType,
};


pub trait AppEntity {
    const RESOURCE: ResourceType;
}

