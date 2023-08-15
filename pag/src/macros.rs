macro_rules! define_pag_class (
($name:ident) => {
    pub struct $name {
        pub ptr: cxx::SharedPtr<ffi::pag::$name>,
    }

    impl $name {
        pub fn from_ptr(ptr: cxx::SharedPtr<ffi::pag::$name>) -> Self {
            Self {
                ptr
            }
        }
        #[allow(dead_code)]
        fn pin_mut<'a>(&self) -> std::pin::Pin<&'a mut ffi::pag::$name> {
            let raw = std::ops::Deref::deref(&self.ptr)
                as *const ffi::pag::$name as *mut ffi::pag::$name;
            unsafe { std::pin::Pin::new_unchecked(&mut *raw) }
        }
    }

    impl std::clone::Clone for crate::$name {
        fn clone(&self) -> Self {
            Self {
                ptr: self.ptr.clone()
            }
        }
    }

    impl std::fmt::Debug for crate::$name {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!($name))
                .finish()
        }
    }
    impl std::fmt::Display for crate::$name {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
);

macro_rules! define_pag_sub_class (
($name:ident, $parent:ident) => {

    pub struct $name {
        parent: crate::$parent,
        pub ptr: cxx::SharedPtr<ffi::pag::$name>,
    }

    impl $name {
        pub fn from_ptr(ptr: cxx::SharedPtr<ffi::pag::$name>) -> Self {
            let parent_ptr: cxx::SharedPtr<ffi::pag::$parent> = unsafe {
                std::mem::transmute(ptr.clone())
            };
            Self {
                parent: crate::$parent::from_ptr(parent_ptr),
                ptr
            }
        }
        #[allow(dead_code)]
        fn pin_mut<'a>(&self) -> std::pin::Pin<&'a mut ffi::pag::$name> {
            let raw = std::ops::Deref::deref(&self.ptr)
                as *const ffi::pag::$name as *mut ffi::pag::$name;
            unsafe { std::pin::Pin::new_unchecked(&mut *raw) }
        }
    }

    impl std::fmt::Debug for crate::$name {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!($name))
                .field("parent", &stringify!($parent))
                .finish()
        }
    }
    impl std::fmt::Display for crate::$name {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl std::clone::Clone for crate::$name {
        fn clone(&self) -> Self {
            Self {
                parent: self.parent.clone(),
                ptr: self.ptr.clone()
            }
        }
    }

    impl std::ops::Deref for $name {
        type Target = $crate::$parent;
        fn deref(&self) -> &Self::Target {
            &self.parent
        }
    }

    impl std::ops::DerefMut for $name {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.parent
        }
    }
}
);

macro_rules! define_pag_type_from(
($into:ident, $name:ident) => {
    impl std::convert::From<crate::$name> for crate::$into {
        fn from(item: crate::$name) -> Self {
            let ptr: cxx::SharedPtr<ffi::pag::$into> = unsafe {
                std::mem::transmute(item.ptr.clone())
            };
            Self::from_ptr(ptr)
        }
    }
}
);
