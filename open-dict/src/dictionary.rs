use crate::{data::DomainWrapper, index::*};
use super::object::Object;
use heapless::LinearMap;

/// Structure representing the Dictionary
/// 
pub struct Dictionary<const SIZE_U32: usize, const SIZE_I32: usize, const SIZE_DOM: usize> {
    member_u32: LinearMap<AccessIndex<MarkerU32>, Object<u32>, SIZE_U32>,
    member_i32: LinearMap<AccessIndex<MarkerI32>, Object<i32>, SIZE_I32>,
    member_domain: LinearMap<AccessIndex<MarkerDomain>, Object<DomainWrapper>, SIZE_U32>,
}

impl<const SIZE_U32: usize, const SIZE_I32: usize, const SIZE_DOM: usize>
    Dictionary<SIZE_U32, SIZE_I32, SIZE_DOM>
{
    pub const fn new() -> Self {
        Self {
            member_u32: LinearMap::new(),
            member_i32: LinearMap::new(),
            member_domain: LinearMap::new(),
        }
    }
}

pub mod domain {

    use crate::data::Data;
    use crate::object::ObjectAccess;

    use super::*;
    impl<const SIZE_U32: usize, const SIZE_I32: usize, const SIZE_DOM: usize>
        Dictionary<SIZE_U32, SIZE_I32, SIZE_DOM>
    {
        pub fn add_domain(
            &mut self,
            index: AccessIndex<MarkerDomain>,
            access: ObjectAccess,
            data: Data<DomainWrapper>,
        ) -> Result<Option<Object<DomainWrapper>>, (AccessIndex<MarkerDomain>, Object<DomainWrapper>)>
        {
            let obj = Object::<DomainWrapper>::new(access, data);
            self.member_domain.insert(index, obj)
        }

        pub fn borrow_domain(&self, index: AccessIndex<MarkerDomain>) -> Option<&DomainWrapper> {
            self.member_domain.get(&index)?.get_data().borrow()
        }
        pub fn borrow_mut_domain(
            &mut self,
            index: AccessIndex<MarkerDomain>,
        ) -> Option<&mut DomainWrapper> {
            self.member_domain
                .get_mut(&index)?
                .get_data_mut()
                .borrow_mut()
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        const DOMAIN_INDEX: AccessIndex<MarkerDomain> =
            AccessIndex::<MarkerDomain>::new(0x2500, 0x02);
        static mut BUFFER: [u8; 1024] = [0; 1024];
        #[test]
        fn basic_domain_api_test() {
            let mut dict = Dictionary::<5, 5, 5>::new();

            dict.add_domain(DOMAIN_INDEX, ObjectAccess::ReadWrite, unsafe {
                Data::<DomainWrapper>::new(&mut BUFFER)
            })
            .unwrap();

            {
                let dom_writer = dict.borrow_mut_domain(DOMAIN_INDEX).unwrap();
                dom_writer.inner[0] = 0xAF;
                dom_writer.inner[1] = 0xFE;
                dom_writer.inner[2] = 0xF0;
                dom_writer.inner[3] = 0x0D;
            }
            {
                let dom = dict.borrow_domain(DOMAIN_INDEX).unwrap();
                assert_eq!(0xAF, dom.inner[0]);
                assert_eq!(0xFE, dom.inner[1]);
                assert_eq!(0xF0, dom.inner[2]);
                assert_eq!(0x0D, dom.inner[3]);
            }
        }
    }
}

macro_rules! ImplApi {
    (($ty:ident,$marker:ident, $member:ident) => $name:ident {$add:ident, $get:ident, $set:ident}) => {
        pub mod $name {
            use crate::data::Data;
            use crate::object::ObjectAccess;

            use super::*;
            impl<const SIZE_U32: usize, const SIZE_I32: usize, const SIZE_DOM: usize>
                super::Dictionary<SIZE_U32, SIZE_I32, SIZE_DOM>
            {
                pub fn $add(
                    &mut self,
                    index: AccessIndex<$marker>,
                    access: ObjectAccess,
                    data: Data<$ty>,
                ) -> Result<Option<Object<$ty>>, (AccessIndex<$marker>, Object<$ty>)> {
                    let obj = Object::<$ty>::new(access, data);
                    self.$member.insert(index, obj)
                }

                pub fn $get(&self, idx: AccessIndex<$marker>) -> Option<$ty> {
                    self.$member.get(&idx)?.get_data().get_value()
                }
                pub fn $set(&mut self, idx: AccessIndex<$marker>, value: $ty) -> Result<(), $ty> {
                    if let Some(obj) = self.$member.get_mut(&idx) {
                        obj.get_data_mut().set_value(value)
                    } else {
                        Err(value)
                    }
                }
            }
            #[cfg(test)]
            mod tests {
                use super::*;
                use crate::data::Data;
                use crate::index::{$marker, AccessIndex};
                use crate::object::ObjectAccess;

                // Globally defined Index Access Token
                const DEVICE_INDEX: AccessIndex<$marker> =
                    AccessIndex::<$marker>::new(0x1000, 0x00);
                static mut DICT: Dictionary<4, 4, 4> = Dictionary::<4, 4, 4>::new();
                #[test]
                fn basic_api_test() {
                    // Provide once unsafe access
                    let dict = unsafe { &mut DICT };

                    dict.$add(
                        DEVICE_INDEX,
                        ObjectAccess::ReadOnly,
                        Data::<$ty>::new(Some(0), Some((0, 255))),
                    )
                    .unwrap();

                    dict.$set(DEVICE_INDEX, 0xFE).unwrap();
                    assert_eq!(dict.$get(DEVICE_INDEX), Some(0xFE));
                }
            }
        }
    };
}

ImplApi!((u32, MarkerU32, member_u32) => unsigned32{add_u32, get_u32, set_u32});
ImplApi!((i32, MarkerI32, member_i32) => interger32{add_i32, get_i32, set_i32});
