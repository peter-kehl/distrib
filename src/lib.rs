// https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
// https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
#![feature(async_fn_in_trait)]
extern crate alloc;
use core::marker::PhantomData;

/// Type `T` **can** be large. It will NOT be used in planning mode, but in processing mode only.
/*pub struct TrackS<T> {
    _t: PhantomData<T>
}

impl <T> TrackS<T> {
    pub fn process_s(&self) -> T {
        loop {}
    }
}*/
// ---

/// Estimated plan. It collects expected storage, bandwidth + latency, and computation costs and
/// constraints.
pub trait Plan: Send + Sized {}
/// Executed plan. It collects actual storage, bandwidth + latency, and computation costs.
pub trait Real: Send + Sized {}

/*pub trait TrackT<P: Plan, R: Real, D: Send + Sized> {
    fn plan(&self) -> () { // return a closure?
    }

    fn process_t(&self) -> D {
        loop {}
    }
}*/

/// [Send] is required, so that async runtimes can move the result across thread.
///
/// [PlanAndResult] is implemented by the "runtime".
pub trait PlanRealData<P: Plan, R: Real, D: Send + Sized>: Send + Sized {
    fn plan_mut(&mut self) -> &mut P {
        unreachable!()
    }
    fn real_mut(&mut self) -> &mut R {
        unreachable!()
    }
    fn data_mut(&mut self) -> &mut D {
        unreachable!()
    }
    fn moved(self) -> (P, R, D) {
        unreachable!()
    }
}

pub trait PlanHolder<P: Plan>: Send + Sized {
    fn plan_mut(&mut self) -> &mut P {
        unreachable!()
    }
    /// Whether we are in the `Plan` mode (rather than `Process` mode).
    fn being_planned(&self) -> bool {
        unreachable!()
    }
}
pub trait RealHolder<R: Real>: Send + Sized {
    fn real_mut(&mut self) -> &mut R {
        unreachable!()
    }
}
pub trait DataHolder<D: Send + Sized>: Send + Sized {
    fn data_mut(&mut self) -> &mut D {
        unreachable!()
    }
}

// @TODO relax Send + Sized?
pub trait PlanRealDataHolders<P: Plan, R: Real>: Send + Sized {
    type PLAN: PlanHolder<P>;
    type REAL: RealHolder<R>;

    type DATA<D: Send + Sized>: DataHolder<D>;

    fn plan_data_moved<D: Send + Sized>(_plan_data: (Self::PLAN, Self::DATA<D>)) -> (P, D) {
        unreachable!()
    }
    fn real_data_moved<D: Send + Sized>(_real_data: (Self::REAL, Self::DATA<D>)) -> (R, D) {
        unreachable!()
    }
}

// @TODO Move
const _: () = {
    struct PlanH<P: Plan> {
        _p: PhantomData<P>,
    }
    impl<P: Plan> PlanHolder<P> for PlanH<P> {}
    struct RealH<R: Real> {
        _r: PhantomData<R>,
    }
    impl<R: Real> RealHolder<R> for RealH<R> {}
    struct DataH<D: Send + Sized> {
        _d: PhantomData<D>,
    }
    impl<D: Send + Sized> DataHolder<D> for DataH<D> {}
    struct PRDHS {}
    impl<P: Plan, R: Real> PlanRealDataHolders<P, R> for PRDHS {
        type PLAN = PlanH<P>;
        type REAL = RealH<R>;
        type DATA<D: Send + Sized> = DataH<D>;
    }
    assert!(core::mem::size_of::<PRDHS>() == 0);
};
pub struct PrdInner<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>, D: Send + Sized> {
    _p: PhantomData<P>,
    _r: PhantomData<R>,
    _prdh: PhantomData<PRDHS>,
    _d: PhantomData<D>,

    pub plan_holder: PRDHS::PLAN,
    pub real_holder: PRDHS::REAL,
    pub data_holder: PRDHS::DATA<D>,
}
unsafe impl<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>, D: Send + Sized> Send
    for PrdInner<P, R, PRDHS, D>
{
}

impl<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>, D: Send + Sized> PrdInner<P, R, PRDHS, D> {
    pub fn plan_data_moved(self) -> (P, D) {
        PRDHS::plan_data_moved((self.plan_holder, self.data_holder))
    }
    pub fn real_data_moved(self) -> (R, D) {
        PRDHS::real_data_moved((self.real_holder, self.data_holder))
    }
    pub fn plan_mut(&mut self) -> &mut P {
        self.plan_holder.plan_mut()
    }
    pub fn real_mut(&mut self) -> &mut R {
        self.real_holder.real_mut()
    }
    pub fn data_mut(&mut self) -> &mut D {
        self.data_holder.data_mut()
    }
    /// Whether we are in the `Plan` mode (rather than `Process` mode).
    pub fn is_plan(&self) -> bool {
        self.plan_holder.being_planned()
    }
}

pub trait PrdTypes: Send + Sized {
    type P: Plan;
    type R: Real;
    type PRDHS: PlanRealDataHolders<Self::P, Self::R>;
}

/*pub trait MoveInnerExact<PTS: PrdTypes, D: Send + Sized> : Sized {
    fn move_inner(self) -> PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>;
}

pub trait MoveInner<INNER: Send + Sized> : Sized {
    fn move_inner(self) -> INNER;
}*/

/// Generate a user space struct that contains one item of type [PrdInner]. This macro, invoked from
/// user space, accepts optional params:
/// - `struct_vis` indicating visibility of the struct (otherwise it's private),
/// - `inner_vis` indicating visibility of `inner` (otherwise it's private),
/// - `method_vis` indicating visibility of methods (otherwise they are private),
/// - `struct_name` (if other than `Prd`).
#[macro_export]
macro_rules! generate_prd_struct {
    // See also https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#vis.
    ($struct_vis:vis, $inner_vis:vis, $method_vis:vis, $struct_name:ident) => {
        /// A struct that carries [distrib::PrdInner].
        ///
        /// This struct exists in user space, so that the user can implement methods on it. That
        /// allows chaining method calls - more ergonomic.
        #[repr(transparent)]
        $struct_vis struct $struct_name<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> {
            $inner_vis inner: $crate::PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>,
        }
        unsafe impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> ::core::marker::Send for $struct_name<PTS, D> {}

        impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> $struct_name<PTS, D> {
            $method_vis fn new(inner: $crate::PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>) -> Self {
                Self { inner }
            }

            $method_vis fn inner(self) -> $crate::PrdInner<PTS::P, PTS::R, PTS::PRDHS, D> {
                self.inner
            }
            $method_vis fn plan_data_moved(self) -> (PTS::P, D) {
                PTS::PRDHS::plan_data_moved((self.inner.plan_holder, self.inner.data_holder))
            }
            $method_vis fn real_data_moved(self) -> (PTS::R, D) {
                PTS::PRDHS::real_data_moved((self.inner.real_holder, self.inner.data_holder))
            }
            $method_vis fn plan_mut(&mut self) -> &mut PTS::P {
                self.inner.plan_holder.plan_mut()
            }
            $method_vis fn real_mut(&mut self) -> &mut PTS::R {
                self.inner.real_holder.real_mut()
            }
            $method_vis fn data_mut(&mut self) -> &mut D {
                self.inner.data_holder.data_mut()
            }
            /// Whether we are in the `Plan` mode (rather than `Process` mode).
            $method_vis fn being_planned(&self) -> ::core::primitive::bool {
                self.inner.plan_holder.being_planned()
            }
        }

        /* // Couldn't compile:
        impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized, FROM: $crate::MoveInnerExact<PTS, D>> ::core::convert::From<FROM> for Prd<PTS, D> {
            fn from(value: FROM) -> Self {
                Self::new(value.move_inner())
            }
        }
        impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized, FROM: $crate::MoveInner<PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>>> ::core::convert::From<FROM> for Prd<PTS, D> {
            fn from(value: FROM) -> Self {
                Self::new(value.move_inner())
            }
        }
        impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> $crate::MoveInner<PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>> for Prd<PTS, D> {
            fn move_inner(self) -> PrdInner<PTS::P, PTS::R, PTS::PRDHS, D> {
                self.inner
            }
        }*/
        /// For interoperability, so that different crates can convert between their implementations
        /// of `Prd` (or the struct named as indicated by `$struct_name` param of
        /// [`distrib::generate_prd_struct`] macro).
        impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> ::core::convert::From<$crate::PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>> for $struct_name<PTS, D> {
            fn from(value: $crate::PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>) -> Self {
                Self::new(value)
            }
        }
    };
    // @TODO Can't match the following with empty (private) last visibility ($method_vis)?!
    ($struct_vis:vis, $inner_vis:vis, $method_vis:vis) => {
        $crate::generate_prd_struct!($struct_vis, $inner_vis, $method_vis, Prd);
    };
    ($struct_name:ident) => {
        $crate::generate_prd_struct!(, , , $struct_name);
    };
    () => {
        $crate::generate_prd_struct!(Prd);
    }
}

generate_prd_struct!(pub, pub, pub, PrdBase);

/// Generate `type` aliases, like `type XyzVec<PTS, T> = Xyz<PTS, Vec<T>>`, where `Xyz` is the name
/// of the struct generated with [`generate_prd_struct`].
///
/// Params:
/// - `vis` - visibility (otherwise private),
/// - `struct_name` - the (existing) struct name (otherwise `Prd`).
///
/// This requires that you have `extern crate alloc;` at the top of your crate's `lib.rs`. That
/// makes it `no_std`-friendly.
#[macro_export]
macro_rules! generate_prd_struct_aliases {
    ($vis: vis, $struct_name:ident) => {
        ::paste::paste! {
            $vis type [<$struct_name Vec>]<PTS, T> = $struct_name<PTS, ::alloc::vec::Vec<T>>;
        }
    };
    ($struct_name:ident) => {
        $crate::generate_prd_struct_aliases!(, $struct_name);
    };
    () => {
        $crate::generate_prd_struct_aliases!(, Prd);
    }
}

generate_prd_struct_aliases!(pub, PrdBase);
impl<PTS: PrdTypes, T: Send + Sized> PrdBaseVec<PTS, T> {
    pub fn map<R: Send + Sized, F: Fn(T) -> R>(self, f: F) -> PrdBaseVec<PTS, R> {
        loop {}
    }
}

/// Generates "proxy" `impl` for the given (user space) struct (which was generated with
/// [`generate_prd_struct`]). These `impl` define functions that proxy to [`PrdBase`] (under its
/// variations/type aliases, such as [`PrdBaseVec`]).
///
/// Invoke [`generate_prd_struct_aliases`] first.
///
/// Params:
/// - `vis` - visibility of the generated (proxy) methods (otherwise private),
/// - `struct_name` - the (existing) struct name (otherwise `Prd`).
///
/// This requires that you have `extern crate alloc;` at the top of your crate's `lib.rs`. That
/// makes it `no_std`-friendly.
#[macro_export]
macro_rules! generate_prd_base_proxies {
    ($vis:vis, $struct_name:ident) => {
        // @TODO consider `paste` crate to generate `PrdVec`
        impl<PTS: $crate::PrdTypes, T: ::core::marker::Send + ::core::marker::Sized> $struct_name<PTS, ::alloc::vec::Vec<T>> {
            $vis fn map<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R>(
                self, f: F
            ) -> $struct_name<PTS, ::alloc::vec::Vec<R>> {
                $crate::PrdBaseVec::<PTS, T>::from(self.inner())
                    .map(f)
                    .inner()
                    .into()
            }
        }
    };
    ($struct_name:ident) => {
        $crate::generate_prd_base_proxies!(, $struct_name);
    };
    () => {
        $crate::generate_prd_base_proxies!(, Prd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
