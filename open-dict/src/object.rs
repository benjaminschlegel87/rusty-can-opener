use super::data::{Data, DictType};

/// Object Access for external requests
/// If the Object is accssest internally this parameter is not evaluated
#[derive(Debug, Default, Clone, Copy)]
pub enum ObjectAccess {
    #[default]
    Const,
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug)]
pub struct Object<T: DictType> {
    data: Data<T>,
    access: ObjectAccess,
}

impl<T: DictType> Object<T> {
    pub const fn new(access: ObjectAccess, data: Data<T>) -> Self {
        Self { data, access }
    }
    pub fn get_data(&self) -> &Data<T> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut Data<T> {
        &mut self.data
    }
    pub fn get_access(&self) -> ObjectAccess {
        self.access
    }
}
