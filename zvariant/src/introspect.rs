use std::iter::{Iterator, IntoIterator};

pub trait Introspectable {
    fn introspection_info() -> IntrospectionHandle;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimaryType {
    Enum,
    Struct,
    Option,
    Array,
    Byte,
    Boolean,
    Int16,
    Int32,
    Int64,
    UInt16,
    UInt32,
    UInt64,
    Double,
    String,
    Unit,
}

pub type IntrospectionHandle = Box<dyn IntrospectionInfo>;

impl IntoIterator for &IntrospectionHandle {
    type Item = (&'static str, IntrospectionHandle);
    type IntoIter = IntrospectionIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        IntrospectionIntoIterator::new(self.copy())
    }
}

impl IntoIterator for IntrospectionHandle {
    type Item = (&'static str, IntrospectionHandle);
    type IntoIter = IntrospectionIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        IntrospectionIntoIterator::new(self)
    }
}

pub struct IntrospectionIntoIterator {
    which: usize,
    handle: IntrospectionHandle,
}

impl IntrospectionIntoIterator {
    fn new(handle: IntrospectionHandle) -> Self {
        IntrospectionIntoIterator {
            which: 0,
            handle,
        }
    }
}

impl Iterator for IntrospectionIntoIterator {
    type Item = (&'static str, IntrospectionHandle);

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.handle.member_by_index(self.which);
        self.which += 1;
        res
    }
}

pub trait IntrospectionInfoImpl {
    fn member_by_index_impl(which: usize) -> Option<(&'static str, IntrospectionHandle)>;
    fn name_impl() -> Option<&'static str>;
    fn primary_type_impl() -> PrimaryType;
    fn new() -> Self;
}

pub trait IntrospectionInfo {
    fn member_by_index(&self, which: usize) -> Option<(&'static str, IntrospectionHandle)>;
    fn name(&self) -> Option<&'static str>;
    fn primary_type(&self) -> PrimaryType;
    fn copy(&self) -> IntrospectionHandle;
}

impl<T: 'static + IntrospectionInfoImpl> IntrospectionInfo for T {
    fn member_by_index(&self, which: usize) -> Option<(&'static str, IntrospectionHandle)> {
        T::member_by_index_impl(which)
    }

    fn name(&self) -> Option<&'static str> {
        T::name_impl()
    }

    fn primary_type(&self) -> PrimaryType {
        T::primary_type_impl()
    }

    fn copy(&self) -> IntrospectionHandle {
        Box::new(T::new())
    }
}

impl<T> Introspectable for Option<T> where T: Introspectable {
    fn introspection_info() -> IntrospectionHandle {
        let val = T::introspection_info();
        struct IntrospectionInfoStruct(IntrospectionHandle);

        impl IntrospectionInfo for IntrospectionInfoStruct {
            fn member_by_index(&self, w: usize) -> Option<(&'static str, IntrospectionHandle)> {
                match w {
                    0 => {
                        let vtable = match self {
                            IntrospectionInfoStruct(vtable) => vtable
                        };
                        Some(("value", vtable.copy()))
                    },
                    _ => None,
                }
            }

            fn name(&self) -> Option<&'static str> {
                None
            }

            fn primary_type(&self) -> PrimaryType {
                PrimaryType::Option
            }

            fn copy(&self) -> IntrospectionHandle {
                let vtable = match self {
                    IntrospectionInfoStruct(vtable) => vtable
                };
                Box::new(IntrospectionInfoStruct(vtable.copy()))
            }
        }

        Box::new(IntrospectionInfoStruct(val))
    }
}

impl<T, const N: usize> Introspectable for [T; N] where T: Introspectable {
    fn introspection_info() -> IntrospectionHandle {
        let val = T::introspection_info();
        struct IntrospectionInfoStruct(IntrospectionHandle);

        impl IntrospectionInfo for IntrospectionInfoStruct {
            fn member_by_index(&self, w: usize) -> Option<(&'static str, IntrospectionHandle)> {
                match w {
                    0 => {
                        let vtable = match self {
                            IntrospectionInfoStruct(vtable) => vtable
                        };
                        Some(("item", vtable.copy()))
                    },
                    _ => None,
                }
            }

            fn name(&self) -> Option<&'static str> {
                None
            }

            fn primary_type(&self) -> PrimaryType {
                PrimaryType::Array
            }

            fn copy(&self) -> IntrospectionHandle {
                let vtable = match self {
                    IntrospectionInfoStruct(vtable) => vtable
                };
                Box::new(IntrospectionInfoStruct(vtable.copy()))
            }
        }

        Box::new(IntrospectionInfoStruct(val))
    }
}

impl<T> Introspectable for &[T] where T: Introspectable {
    fn introspection_info() -> IntrospectionHandle {
        let val = T::introspection_info();
        struct IntrospectionInfoStruct(IntrospectionHandle);

        impl IntrospectionInfo for IntrospectionInfoStruct {
            fn member_by_index(&self, w: usize) -> Option<(&'static str, IntrospectionHandle)> {
                match w {
                    0 => {
                        let vtable = match self {
                            IntrospectionInfoStruct(vtable) => vtable
                        };
                        Some(("item", vtable.copy()))
                    },
                    _ => None,
                }
            }

            fn name(&self) -> Option<&'static str> {
                None
            }

            fn primary_type(&self) -> PrimaryType {
                PrimaryType::Array
            }

            fn copy(&self) -> IntrospectionHandle {
                let vtable = match self {
                    IntrospectionInfoStruct(vtable) => vtable
                };
                Box::new(IntrospectionInfoStruct(vtable.copy()))
            }
        }

        Box::new(IntrospectionInfoStruct(val))
    }
}

impl<T> Introspectable for Vec<T> where T: Introspectable {
    fn introspection_info() -> IntrospectionHandle {
        let val = T::introspection_info();
        struct IntrospectionInfoStruct(IntrospectionHandle);

        impl IntrospectionInfo for IntrospectionInfoStruct {
            fn member_by_index(&self, w: usize) -> Option<(&'static str, IntrospectionHandle)> {
                match w {
                    0 => {
                        let vtable = match self {
                            IntrospectionInfoStruct(vtable) => vtable
                        };
                        Some(("some", vtable.copy()))
                    },
                    _ => None,
                }
            }

            fn name(&self) -> Option<&'static str> {
                None
            }

            fn primary_type(&self) -> PrimaryType {
                PrimaryType::Array
            }

            fn copy(&self) -> IntrospectionHandle {
                let vtable = match self {
                    IntrospectionInfoStruct(vtable) => vtable
                };
                Box::new(IntrospectionInfoStruct(vtable.copy()))
            }
        }

        Box::new(IntrospectionInfoStruct(val))
    }
}

impl<T> Introspectable for &T where T: Introspectable {
    fn introspection_info() -> IntrospectionHandle {
        T::introspection_info()
    }
}

impl<T> Introspectable for &mut T where T: Introspectable {
    fn introspection_info() -> IntrospectionHandle {
        T::introspection_info()
    }
}

impl<T> Introspectable for Box<T> where T: Introspectable {
    fn introspection_info() -> IntrospectionHandle {
        T::introspection_info()
    }
}

macro_rules! primitive_type {
    ($type: ty, $value: expr) => {
        impl Introspectable for $type {
            fn introspection_info() -> IntrospectionHandle {
                struct IntrospectionInfoStruct;

                impl IntrospectionInfoImpl for IntrospectionInfoStruct {
                    fn member_by_index_impl(_: usize) -> Option<(&'static str, IntrospectionHandle)> {
                        None
                    }

                    fn name_impl() -> Option<&'static str> {
                        None
                    }

                    fn primary_type_impl() -> PrimaryType {
                        $value
                    }

                    fn new() -> IntrospectionInfoStruct {
                        IntrospectionInfoStruct
                    }
                }

                Box::new(IntrospectionInfoStruct)
            }
        }
    };
}

primitive_type!{bool, PrimaryType::Boolean}
primitive_type!{i32, PrimaryType::Int32}
primitive_type!{u8, PrimaryType::Byte}
primitive_type!{i16, PrimaryType::Int16}
primitive_type!{i64, PrimaryType::Int64}
primitive_type!{u16, PrimaryType::UInt16}
primitive_type!{u32, PrimaryType::UInt32}
primitive_type!{u64, PrimaryType::UInt64}
primitive_type!{f32, PrimaryType::Double}
primitive_type!{f64, PrimaryType::Double}
primitive_type!{&str, PrimaryType::String}
primitive_type!{String, PrimaryType::String}
primitive_type!{(), PrimaryType::Unit}
