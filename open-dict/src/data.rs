pub trait DictType {}

impl DictType for u8 {}
impl DictType for u16 {}
impl DictType for u32 {}
impl DictType for u64 {}
impl DictType for i8 {}
impl DictType for i16 {}
impl DictType for i32 {}
impl DictType for i64 {}
#[derive(Debug, Default)]
pub struct DomainWrapper {
    pub inner: &'static mut [u8],
}
impl DomainWrapper {
    pub fn new(inner: &'static mut [u8]) -> Self {
        Self { inner }
    }
}
impl DictType for DomainWrapper {}

// Make sense to add limit check and default to data
// Check it in data scope - types always align
// DataType also here?
#[derive(Debug, Default)]
pub struct Data<T: DictType> {
    limit: Option<(T, T)>,
    default: Option<T>,
    value: Option<T>,
}
macro_rules! DataNew {
    ($ty:ident) => {
        impl Data<$ty> {
            pub const fn new(default: Option<$ty>, limit: Option<($ty, $ty)>) -> Self {
                // Check that default is within limit
                let checked_default = Data::<$ty>::restore_default_internal(limit, default);
                Self {
                    limit: limit,
                    default,
                    value: checked_default,
                }
            }
            const fn restore_default_internal(
                limit: Option<($ty, $ty)>,
                default: Option<$ty>,
            ) -> Option<$ty> {
                if let Some((lower, upper)) = limit {
                    if let Some(value) = default {
                        // Limit is Some and Default is Some
                        // Check bounds
                        if value < lower || value > upper {
                            None
                        } else {
                            default
                        }
                    } else {
                        // No default given
                        None
                    }
                } else {
                    // No limit given
                    // fine to use default
                    default
                }
            }
            pub fn restore_default(&mut self) {
                self.value = Data::<$ty>::restore_default_internal(self.limit, self.default);
            }
            pub fn get_value(&self) -> Option<$ty> {
                self.value
            }
            pub fn set_value(&mut self, val: $ty) -> Result<(), $ty> {
                if let Some((lower, upper)) = self.limit {
                    if val < lower || val > upper {
                        return Err(val);
                    }
                }
                self.value = Some(val);
                Ok(())
            }
            pub fn is_value_set(&self) -> bool {
                self.value.is_some()
            }
        }
        impl Clone for Data<$ty> {
            fn clone(&self) -> Self {
                Self {
                    default: self.default.clone(),
                    limit: self.limit.clone(),
                    value: self.value.clone(),
                }
            }
        }
    };
}
DataNew!(u32);
DataNew!(u16);
DataNew!(u8);
DataNew!(u64);
DataNew!(i32);
DataNew!(i16);
DataNew!(i8);
DataNew!(i64);

impl Data<DomainWrapper> {
    pub fn new(buffer: &'static mut [u8]) -> Self {
        Self {
            limit: None,
            default: None,
            value: Some(DomainWrapper::new(buffer)),
        }
    }

    pub fn borrow(&self) -> Option<&DomainWrapper> {
        self.value.as_ref()
    }

    pub fn borrow_mut(&mut self) -> Option<&mut DomainWrapper> {
        self.value.as_mut()
    }
}

#[cfg(test)]
mod expteriments {
    use super::*;
    #[test]
    fn happy_paths_copytypes() {
        let mut data = Data::<i32>::new(Some(-5), Some((-10, 10)));
        // Default within limits
        assert!(data.is_value_set());
        assert_eq!(-5, data.get_value().unwrap());
        // change value
        assert_eq!(Ok(()), data.set_value(-9));

        assert!(data.is_value_set());
        assert_eq!(-9, data.get_value().unwrap());

        data.restore_default();

        assert!(data.is_value_set());
        assert_eq!(-5, data.get_value().unwrap());
    }

    #[test]
    fn default_without_limits() {
        // Construct data with Default but no Limit (None)
        let mut data = Data::<u8>::new(Some(50), None);

        // Should correctly set default on constructio
        assert!(data.is_value_set());
        assert_eq!(50, data.get_value().unwrap());

        // Change value
        assert_eq!(Ok(()), data.set_value(80));

        // Works okay
        assert_eq!(80, data.get_value().unwrap());

        // Check that restore works
        data.restore_default();
        assert!(data.is_value_set());
        assert_eq!(50, data.get_value().unwrap());
    }

    #[test]
    fn default_outside_limits() {
        // Construct data with Default but no Limit (None)
        let mut data = Data::<u16>::new(Some(50), Some((10, 20)));

        assert_eq!(false, data.is_value_set());
        assert_eq!(None, data.get_value());

        data.restore_default();

        assert_eq!(false, data.is_value_set());
        assert_eq!(None, data.get_value());

        assert_eq!(Err(100), data.set_value(100));
        assert_eq!(Ok(()), data.set_value(15));

        assert_eq!(true, data.is_value_set());
        assert_eq!(Some(15), data.get_value());
    }

    #[test]
    fn domain_happy_path() {
        static mut BUFFER: [u8; 1024] = [0; 1024];
        let mut domain = unsafe { Data::<DomainWrapper>::new(&mut BUFFER) };
        let buffer = domain.borrow_mut().unwrap();
        buffer.inner[0] = 0xAF;
        buffer.inner[1] = 0xFE;

        let reader = domain.borrow().unwrap();
        assert_eq!(0xAF, reader.inner[0]);
        assert_eq!(0xFE, reader.inner[1]);
    }
}
