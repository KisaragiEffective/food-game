use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub mod private {
    pub(crate) trait Sealed {}
}

trait DowncastUnchecked : private::Sealed {
    type P<T: Any + ?Sized>;

    unsafe fn downcast<R: Any>(this: Self::P<dyn Any>) -> Self::P<R>;
}

impl<'a> private::Sealed for &'a dyn Any {}

impl<'a> DowncastUnchecked for &'a dyn Any {
    type P<T: Any + ?Sized> = &'a T;

    unsafe fn downcast<R: Any>(this: Self::P<dyn Any>) -> Self::P<R> {
        debug_assert!(this.is::<R>());
        unsafe { &*(this as *const dyn Any as *const R) }
    }
}

impl private::Sealed for Box<dyn Any> {}

impl DowncastUnchecked for Box<dyn Any> {
    type P<T: Any + ?Sized> = Box<T>;

    unsafe fn downcast<R: Any>(this: Self::P<dyn Any>) -> Self::P<R> {
        debug_assert!(this.is::<R>());
        let raw: *mut dyn Any = Box::into_raw(this);

        unsafe {
            Box::from_raw(raw as *mut R)
        }
    }
}

// this function is port of stdlib: MIT OR Apache-2.0
pub unsafe fn downcast_ref_unchecked<T: Any>(this: &dyn Any) -> &T {
    <&dyn Any>::downcast(this)
}

unsafe fn downcast_boxed<T: Any>(this: Box<dyn Any>) -> Box<T> {
    <Box<dyn Any> as DowncastUnchecked>::downcast(this)
}
