// https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
// https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
#![feature(async_fn_in_trait)]
extern crate alloc;
use core::marker::PhantomData;

/// Estimated plan. It collects expected storage, bandwidth + latency, and computation costs and
/// constraints.
pub trait Plan: Send + Sized {}
/// Executed plan. It collects actual storage, bandwidth + latency, and computation costs.
pub trait Real: Send + Sized {}

#[derive(Clone, Copy)]
pub struct Cost {
    pub stack: f32,
    pub heap: f32,
    pub cpu: f32,
    pub gpu: f32,
    // We could have a field for SSL (since some Intel chipsets accelerate it). But this is likely
    // to be processed after decrypted/before encrypted, anyway.
    pub storage: f32,
    pub bandwidth: f32,
    pub latency: f32,
    pub fluctuation: f32,
    pub reliability: f32,
}
impl Cost {
    pub const fn new(
        stack: f32,
        heap: f32,
        cpu: f32,
        gpu: f32,
        storage: f32,
        bandwidth: f32,
        latency: f32,
        fluctuation: f32,
        reliability: f32,
    ) -> Self {
        Self {
            stack,
            heap,
            cpu,
            gpu,
            storage,
            bandwidth,
            latency,
            fluctuation,
            reliability,
        }
    }
}
#[inline]
pub const fn default_cost() -> Cost {
    Cost::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
}
impl Default for Cost {
    fn default() -> Self {
        default_cost()
    }
}
unsafe impl Send for Cost {}

macro_rules! unsupported {
    () => {
        unimplemented!("Not supposed to be used.")
    };
}

/// [Send] is required, so that async runtimes can move the result across threads.
///
/// [PlanRealData] is implemented by this crate's "runtime".
pub trait PlanRealData<P: Plan, R: Real, D: Send + Sized>: Send + Sized {
    fn plan_mut(&mut self) -> &mut P {
        unsupported!()
    }
    fn real_mut(&mut self) -> &mut R {
        unsupported!()
    }
    fn data_mut(&mut self) -> &mut D {
        unsupported!()
    }
    fn moved(self) -> (P, R, D) {
        unsupported!()
    }
}

pub trait PlanHolder<P: Plan>: Send + Sized {
    fn plan_mut(&mut self) -> &mut P {
        unsupported!()
    }
    /// Whether we are in the `Plan` mode (rather than `Process` mode).
    fn being_planned(&self) -> bool {
        unsupported!()
    }
}
pub trait RealHolder<R: Real>: Send + Sized {
    fn real_mut(&mut self) -> &mut R {
        unsupported!()
    }
}
pub trait DataHolder<D: Send + Sized>: Send + Sized {
    fn data_mut(&mut self) -> &mut D {
        unsupported!()
    }
}
pub trait CostTable: Send + Sized {
    fn cost_basic_mut(&mut self) -> &mut Cost {
        unsupported!()
    }
    /// Only if supported.
    fn cost_second_mut(&mut self) -> &mut Cost {
        unsupported!()
    }
    /// Only if supported.
    fn cost_third_mut(&mut self) -> &mut Cost {
        unsupported!()
    }
}
pub trait CostHolder<CT: CostTable>: Send + Sized {
    fn cost_table_mut(&mut self) -> &mut CT {
        unsupported!()
    }
    fn empty() -> Self {
        unsupported!()
    }
    fn from_cost(cost: Cost) -> Self {
        unsupported!()
    }
    fn from_cost_table(cost_table: CT) -> Self {
        unsupported!()
    }
}
//impl<CT: CostTable, T: CostHolder<CT>> From<Cost> for T {}

// @TODO relax Send + Sized?
pub trait PlanRealDataHolders<P: Plan, R: Real, CT: CostTable>: Send + Sized {
    type PLAN: PlanHolder<P>;
    type REAL: RealHolder<R>;
    type COST: CostHolder<CT>;

    type DATA<D: Send + Sized>: DataHolder<D>;
    /// For Plan mode only. Prefer [PlanRealDataHolders::plan_cost_holder_data_moved] instead.
    fn plan_cost_table_data_moved<D: Send + Sized>(
        _plan: Self::PLAN,
        _cost: Self::COST,
        _data: Self::DATA<D>,
    ) -> (P, CT, D) {
        unsupported!()
    }
    /// For Plan mode only. Preferred (instead of [PlanRealDataHolders::plan_cost_table_data_moved]).
    fn plan_cost_holder_data_moved<D: Send + Sized>(
        _plan: Self::PLAN,
        _cost: Self::COST,
        _data: Self::DATA<D>,
    ) -> (P, Self::COST, D) {
        unsupported!()
    }

    fn real_data_moved<D: Send + Sized>(_real: Self::REAL, _data: Self::DATA<D>) -> (R, D) {
        unsupported!()
    }

    fn from_plan_cost_table_data<D: Send + Sized>(_plan: P, _cost_table: CT, _data: D) -> Self {
        unsupported!()
    }
    fn from_plan_cost_holder_data<D: Send + Sized>(
        _plan: P,
        _cost_holder: Self::COST,
        _data: D,
    ) -> Self {
        unsupported!()
    }

    fn from_real_data<D: Send + Sized>(_real: R, _data: D) -> Self {
        unsupported!()
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

    struct CostH<CT: CostTable> {
        _ct: PhantomData<CT>,
    }
    impl<CT: CostTable> CostHolder<CT> for CostH<CT> {}

    struct PRDHS {}
    impl<P: Plan, R: Real, CT: CostTable> PlanRealDataHolders<P, R, CT> for PRDHS {
        type PLAN = PlanH<P>;
        type REAL = RealH<R>;
        type COST = CostH<CT>;
        type DATA<D: Send + Sized> = DataH<D>;
    }
    assert!(core::mem::size_of::<PRDHS>() == 0);
};
pub struct PrdInner<
    P: Plan,
    R: Real,
    CT: CostTable,
    PRDHS: PlanRealDataHolders<P, R, CT>,
    D: Send + Sized,
> {
    _p: PhantomData<P>,
    _r: PhantomData<R>,
    _ct: PhantomData<CT>,
    _prdh: PhantomData<PRDHS>,
    _d: PhantomData<D>,

    pub plan_holder: PRDHS::PLAN,
    pub real_holder: PRDHS::REAL,
    pub data_holder: PRDHS::DATA<D>,
    pub cost_holder: PRDHS::COST,
}
unsafe impl<P: Plan, R: Real, CT: CostTable, PRDHS: PlanRealDataHolders<P, R, CT>, D: Send + Sized>
    Send for PrdInner<P, R, CT, PRDHS, D>
{
}

impl<P: Plan, R: Real, CT: CostTable, PRDHS: PlanRealDataHolders<P, R, CT>, D: Send + Sized>
    PrdInner<P, R, CT, PRDHS, D>
{
    pub fn plan_cost_table_data_moved(self) -> (P, CT, D) {
        PRDHS::plan_cost_table_data_moved(self.plan_holder, self.cost_holder, self.data_holder)
    }
    pub fn plan_cost_holder_data_moved(
        self,
    ) -> (P, <PRDHS as PlanRealDataHolders<P, R, CT>>::COST, D) {
        PRDHS::plan_cost_holder_data_moved(self.plan_holder, self.cost_holder, self.data_holder)
    }

    pub fn real_data_moved(self) -> (R, D) {
        PRDHS::real_data_moved(self.real_holder, self.data_holder)
    }

    pub fn from_plan_cost_table_data(_plan: P, _cost_table: CT, _data: D) -> Self {
        unsupported!()
    }
    pub fn from_plan_cost_holder_data(
        _plan: P,
        _cost_holder: <PRDHS as PlanRealDataHolders<P, R, CT>>::COST,
        _data: D,
    ) -> Self {
        unsupported!()
    }

    pub fn from_real_data(_real: R, _data: D) -> Self {
        unsupported!()
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
    pub fn cost_table_mut(&mut self) -> &mut CT {
        self.cost_holder.cost_table_mut()
    }
    /// Whether we are in the `Plan` mode (rather than `Process` mode).
    pub fn is_plan(&self) -> bool {
        self.plan_holder.being_planned()
    }
}

pub trait PrdTypes: Send + Sized {
    type P: Plan;
    type R: Real;
    type CT: CostTable;
    type PRDHS: PlanRealDataHolders<Self::P, Self::R, Self::CT>;
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
            $inner_vis inner: $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D>,
        }
        unsafe impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> ::core::marker::Send for $struct_name<PTS, D> {}

        impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> $struct_name<PTS, D> {
            $method_vis fn new(inner: $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D>) -> Self {
                Self { inner }
            }
            $method_vis fn from_plan_cost_table_data(plan: PTS::P, cost_table: PTS::CT, data: D) -> Self {
                // @TODO The following (listing generic params) fails!
                // Self::new(PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>::from_plan_data(plan, data))
                Self::new($crate::PrdInner::from_plan_cost_table_data(plan, cost_table, data))
            }
            $method_vis fn from_plan_cost_holder_data(plan: PTS::P, cost_holder: <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST, data: D) -> Self {
                // @TODO The following (listing generic params) fails!
                // Self::new(PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>::from_plan_data(plan, data))
                Self::new($crate::PrdInner::from_plan_cost_holder_data(plan, cost_holder, data))
            }

            $method_vis fn from_real_data(real: PTS::R, data: D) -> Self {
                Self::new($crate::PrdInner::from_real_data(real, data))
            }

            $method_vis fn inner(self) -> $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D> {
                self.inner
            }

            $method_vis fn plan_cost_table_data_moved(self) -> (PTS::P, PTS::CT, D) {
                PTS::PRDHS::plan_cost_table_data_moved(self.inner.plan_holder, self.inner.cost_holder, self.inner.data_holder)
            }
            $method_vis fn plan_cost_holder_data_moved(self) -> (PTS::P, <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST, D) {
                PTS::PRDHS::plan_cost_holder_data_moved(self.inner.plan_holder, self.inner.cost_holder, self.inner.data_holder)
            }

            $method_vis fn real_data_moved(self) -> (PTS::R, D) {
                PTS::PRDHS::real_data_moved(self.inner.real_holder, self.inner.data_holder)
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
        impl<PTS: $crate::PrdTypes, D: ::core::marker::Send + ::core::marker::Sized> ::core::convert::From<$crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D>> for $struct_name<PTS, D> {
            fn from(value: $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D>) -> Self {
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

fn empty_vec<T>() -> Vec<T> {
    Vec::with_capacity(0)
}
impl<PTS: PrdTypes, T: Send + Sized> PrdBaseVec<PTS, T> {
    pub fn map_leaf_uniform_cost_obj<R: Send + Sized, F: Fn(T) -> R>(
        self,
        each: F,
        cost_each: Cost,
    ) -> PrdBaseVec<PTS, R> {
        if self.being_planned() {
            let (plan, cost_table, data) = self.plan_cost_table_data_moved();
            // @TODO Storage ops: If continuous input & from a sequential source, amortize access
            // time.

            // RAM cost: We sum both the output AND *input* data (since input may be larger).

            PrdBaseVec::from_plan_cost_table_data(plan, cost_table, empty_vec())
        } else {
            let (real, data) = self.real_data_moved();

            let mut result = Vec::with_capacity(data.len());
            result.extend(data.into_iter().map(each));
            PrdBaseVec::from_real_data(real, result)
        }
    }

    pub fn map_leaf_uniform_cost_holder<R: Send + Sized, F: Fn(T) -> R>(
        self,
        each: F,
        cost_holder_each: <<PTS as PrdTypes>::PRDHS as PlanRealDataHolders<
            PTS::P,
            PTS::R,
            PTS::CT,
        >>::COST,
    ) -> PrdBaseVec<PTS, R> {
        if self.being_planned() {
            let (plan, cost_holder, data) = self.plan_cost_holder_data_moved();
            // @TODO Storage ops: If continuous input & from a sequential source, amortize access
            // time.

            // RAM cost: We sum both the output AND *input* data (since input may be larger).

            PrdBaseVec::from_plan_cost_holder_data(plan, cost_holder, empty_vec())
        } else {
            let (real, data) = self.real_data_moved();

            let mut result = Vec::with_capacity(data.len());
            result.extend(data.into_iter().map(each));
            PrdBaseVec::from_real_data(real, result)
        }
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
        impl<PTS: $crate::PrdTypes, T: ::core::marker::Send + ::core::marker::Sized> $struct_name<PTS, ::alloc::vec::Vec<T>> {
            $vis fn map_leaf_uniform_cost_obj<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R>(
                self, each: F, cost_each: Cost
            ) -> $struct_name<PTS, ::alloc::vec::Vec<R>> {
                $crate::PrdBaseVec::<PTS, T>::from(self.inner())
                    .map_leaf_uniform_cost_obj(each, cost_each)
                    .inner()
                    .into()
            }

            $vis fn map_leaf_uniform_cost_holder<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R>(
                self, each: F,
                cost_holder_each: <<PTS as $crate::PrdTypes>::PRDHS as $crate::PlanRealDataHolders<
                PTS::P,
                PTS::R,
                PTS::CT,
            >>::COST
            ) -> $struct_name<PTS, ::alloc::vec::Vec<R>> {
                $crate::PrdBaseVec::<PTS, T>::from(self.inner())
                    .map_leaf_uniform_cost_holder(each, cost_holder_each)
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
