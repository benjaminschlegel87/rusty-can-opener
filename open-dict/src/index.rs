use core::marker::PhantomData;

/// Zero sized Marker struct used to mark that a Index corresponds to a object
/// of Type [u32]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkerU32;

/// Zero sized Marker struct used to mark that a Index corresponds to a object
/// of Type [i32]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkerI32;

/// Zero sized Marker struct used to mark that a Index corresponds to a object
/// of Type [crate::data::DomainWrapper]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkerDomain;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Index {
    index: u16,
    subindex: u8,
}
impl Index {
    pub const fn new(index: u16, subindex: u8) -> Self {
        Self { index, subindex }
    }
}
/// The User must use [AccessIndex] in the API of the dictionary. We prevent the user from calling
/// a _u32 method on the [crate::dictionary::Dictionary] with a index that corresponds to a object that is not a _u32 by implementing a zero sized state via
/// [PhantomData]. Now it is not possible to call a API method with a [AccessIndex] that fits not the the type of the Object
/// # Example
/// ```
/// # use crate::open_dict::index::*;
/// # mod dictionary{
/// # use crate::open_dict::index::*;
/// # pub fn get_u32(index: AccessIndex<MarkerU32>) -> u32{
/// # 0 as u32
/// # }
///
/// # fn get_i32(index: AccessIndex<MarkerI32>) -> i32{
/// # 0 as i32
/// # }
/// # }
/// const INDEX_OF_OBJECT: AccessIndex<MarkerU32> = AccessIndex::<MarkerU32>::new(0x2452, 0x01);
/// # use crate::open_dict::dictionary::Dictionary;
/// # let dict = Dictionary::<5,5,5>::new();
/// // This compiles fine as it has MarkerU32
/// // get_u32 API function if Dictionary accepts this type
/// dict.get_u32(INDEX_OF_OBJECT);
/// // This call would not compile as get_i32 API functio nneeds MarkerI32
/// // dictionary::get_i32(INDEX_OF_OBJECT);
///
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessIndex<T> {
    index: Index,
    marker: PhantomData<T>,
}
macro_rules! ImplAccessIndexNew {
    ($ty: ident) => {
        impl AccessIndex<$ty> {
            pub const fn new(index: u16, subindex: u8) -> Self {
                Self {
                    index: Index::new(index, subindex),
                    marker: PhantomData::<$ty>,
                }
            }
        }
    };
}
ImplAccessIndexNew!(MarkerU32);
ImplAccessIndexNew!(MarkerDomain);
ImplAccessIndexNew!(MarkerI32);
