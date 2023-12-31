// https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
// https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
//#![feature(async_fn_in_trait)]
#![allow(unused_variables)]
extern crate alloc;
use core::marker::PhantomData;

/// Estimated plan. It collects expected storage, bandwidth + latency, and computation costs and
/// constraints.
pub trait Plan: SendSized {}
/// Executed plan. It collects actual storage, bandwidth + latency, and computation costs.
pub trait Real: SendSized {}

#[derive(Clone, Copy)]
pub struct Cost {
    //pub stack: f32,
    pub heap: f32,
    pub cpu: f32,
    //pub gpu: f32,
    // We could have a field for SSL (since some Intel chipsets accelerate it). But this is likely
    // to be processed after decrypted/before encrypted, anyway.
    pub storage: f32,
    pub bandwidth: f32,
    //pub latency: f32,
    //pub fluctuation: f32,
    //pub reliability: f32,
}
impl Cost {
    pub const fn new(
        //stack: f32,
        heap: f32,
        cpu: f32,
        //gpu: f32,
        storage: f32,
        bandwidth: f32,
        //latency: f32,
        //fluctuation: f32,
        //reliability: f32,
    ) -> Self {
        Self {
            //stack,
            heap,
            cpu,
            //gpu,
            storage,
            bandwidth,
            //latency,
            //fluctuation,
            //reliability,
        }
    }
    pub fn heap(mut self, heap: f32) -> Self {
        self.heap = heap;
        self
    }
    pub fn cpu(mut self, cpu: f32) -> Self {
        self.cpu = cpu;
        self
    }
    pub fn storage(mut self, storage: f32) -> Self {
        self.storage = storage;
        self
    }
    pub fn bandwidth(&mut self, bandwidth: f32) -> &mut Self {
        self.bandwidth = bandwidth;
        self
    }
}
#[inline]
pub const fn default_cost() -> Cost {
    Cost::new(0.0, 0.0, 0.0, 0.0)
}
impl Default for Cost {
    fn default() -> Self {
        default_cost()
    }
}
unsafe impl Send for Cost {}

pub const REAL: bool = true;
pub const PLAN: bool = false;

// @TODO Consider
//#[repr(transparent)]
pub struct CostOf<const IS_REAL: bool> {
    cost: Cost,
}

#[macro_export]
macro_rules! unsupported {
    () => {
        unimplemented!("Not supposed to be used.")
    };
}

/// [Send] is required, so that async runtimes can move the result across threads.
///
/// [PlanRealData] is implemented by this crate's "runtime".
pub trait PlanRealData<P: Plan, R: Real, D: SendSized, const IS_REAL: bool>: SendSized {
    fn plan_mut(&mut self) -> &mut P {
        unsupported!()
    }
    fn real_mut(&mut self) -> &mut R {
        unsupported!()
    }
    fn data(&self) -> &D {
        unsupported!()
    }
    fn data_mut(&mut self) -> &mut D {
        unsupported!()
    }
    fn moved(self) -> (P, R, D) {
        unsupported!()
    }
}

pub trait PlanHolder<P: Plan, const IS_REAL: bool>: SendSized {
    fn plan_mut(&mut self) -> &mut P {
        unsupported!()
    }
    /// Whether we are in the `Plan` mode (rather than `Process` mode).
    fn being_planned(&self) -> bool {
        unsupported!()
    }
}
pub trait RealHolder<R: Real, const IS_REAL: bool>: SendSized {
    fn real_mut(&mut self) -> &mut R {
        unsupported!()
    }
}
pub trait DataHolder<D: Send + Sized>: Send + Sized {
    fn data(&self) -> &D {
        unsupported!()
    }
    fn data_mut(&mut self) -> &mut D {
        unsupported!()
    }
}
pub trait CostTable<const IS_REAL: bool>: Send + Sized {
    /// Indicates that we need only basic [Cost] object.
    fn using_basic_mut(&mut self) {
        unsupported!()
    }
    fn cost_basic_mut(&mut self) -> &mut Cost {
        unsupported!()
    }

    /// Indicates that we need the second [Cost] object, too.
    fn using_second_mut(&mut self) {
        unsupported!()
    }
    /// Only if supported.
    fn cost_second_mut(&mut self) -> &mut Cost {
        unsupported!()
    }

    /// Indicates that we need the third [Cost] object, too.
    fn using_third_mut(&mut self) {
        unsupported!()
    }
    /// Only if supported.
    fn cost_third_mut(&mut self) -> &mut Cost {
        unsupported!()
    }
}
pub trait CostHolder<CT: CostTable<IS_REAL>, const IS_REAL: bool>: Send + Sized {
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
// Can't have a blanket impl of From, nor of Into:
/*impl<U: CostHolder<CT, IS_REAL>, CT: CostTable<IS_REAL>, const IS_REAL: bool> From<Cost> for U {
    fn from(value: Cost) -> Self {
        todo!()
    }
}
impl<U: CostHolder<CT, IS_REAL>, CT: CostTable<IS_REAL>, const IS_REAL: bool> Into<U> for Cost {
    fn into(from: Cost) -> Self {
        todo!()
    }
}*/

// @TODO relax Send + Sized?
pub trait PlanRealDataHolders<P: Plan, R: Real, CT: CostTable<IS_REAL>, const IS_REAL: bool>:
    Send + Sized
{
    type PLAN: PlanHolder<P, IS_REAL>;
    type REAL: RealHolder<R, IS_REAL>;
    type COST: CostHolder<CT, IS_REAL>;

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
    impl<P: Plan, const IS_REAL: bool> PlanHolder<P, IS_REAL> for PlanH<P> {}
    struct RealH<R: Real> {
        _r: PhantomData<R>,
    }
    impl<R: Real, const IS_REAL: bool> RealHolder<R, IS_REAL> for RealH<R> {}
    struct DataH<D: Send + Sized> {
        _d: PhantomData<D>,
    }
    impl<D: Send + Sized> DataHolder<D> for DataH<D> {}

    struct CostH<CT: CostTable<IS_REAL>, const IS_REAL: bool> {
        _ct: PhantomData<CT>,
    }
    impl<CT: CostTable<IS_REAL>, const IS_REAL: bool> CostHolder<CT, IS_REAL> for CostH<CT, IS_REAL> {}

    struct PRDHS {}
    impl<P: Plan, R: Real, CT: CostTable<IS_REAL>, const IS_REAL: bool>
        PlanRealDataHolders<P, R, CT, IS_REAL> for PRDHS
    {
        type PLAN = PlanH<P>;
        type REAL = RealH<R>;
        type COST = CostH<CT, IS_REAL>;
        type DATA<D: Send + Sized> = DataH<D>;
    }
    assert!(core::mem::size_of::<PRDHS>() == 0);
};
/// Struct for collecting Prd: Plan + Real + Data. Used as the only inner field in struct(s)
/// generated by [generate_prd_struct] .(Those outer structs are, by default, called [Prd] in user
/// space. In this crate the outer struct is called [PrdBase].)
pub struct PrdInner<
    P: Plan,
    R: Real,
    CT: CostTable<IS_REAL>,
    PRDHS: PlanRealDataHolders<P, R, CT, IS_REAL>,
    D: Send + Sized,
    const IS_REAL: bool,
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
unsafe impl<
        P: Plan,
        R: Real,
        CT: CostTable<IS_REAL>,
        PRDHS: PlanRealDataHolders<P, R, CT, IS_REAL>,
        D: Send + Sized,
        const IS_REAL: bool,
    > Send for PrdInner<P, R, CT, PRDHS, D, IS_REAL>
{
}

impl<
        P: Plan,
        R: Real,
        CT: CostTable<IS_REAL>,
        PRDHS: PlanRealDataHolders<P, R, CT, IS_REAL>,
        D: Send + Sized,
        const IS_REAL: bool,
    > PrdInner<P, R, CT, PRDHS, D, IS_REAL>
{
    pub fn plan_cost_table_data_moved(self) -> (P, CT, D) {
        PRDHS::plan_cost_table_data_moved(self.plan_holder, self.cost_holder, self.data_holder)
    }
    pub fn plan_cost_holder_data_moved(
        self,
    ) -> (
        P,
        <PRDHS as PlanRealDataHolders<P, R, CT, IS_REAL>>::COST,
        D,
    ) {
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
        _cost_holder: <PRDHS as PlanRealDataHolders<P, R, CT, IS_REAL>>::COST,
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
    pub fn data(&self) -> &D {
        self.data_holder.data()
    }
    pub fn data_mut(&mut self) -> &mut D {
        self.data_holder.data_mut()
    }
    pub fn cost_table_mut(&mut self) -> &mut CT {
        self.cost_holder.cost_table_mut()
    }
    /// Whether we are in the `Plan` mode (rather than `Process` mode).
    pub fn is_being_planned(&self) -> bool {
        self.plan_holder.being_planned()
    }
    // @TODO remove
    /*pub fn cost_each<F: Fn(Cost) -> Cost>(&self, f: F) -> Cost {
        todo!()
    }*/
}

pub trait PrdTypes<const IS_REAL: bool>: Send + Sized {
    type P: Plan;
    type R: Real;
    type CT: CostTable<IS_REAL>;
    type PRDHS: PlanRealDataHolders<Self::P, Self::R, Self::CT, IS_REAL>;
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
        $struct_vis struct $struct_name<PTS: $crate::PrdTypes<IS_REAL>, D: ::core::marker::Send + ::core::marker::Sized, const IS_REAL: bool> {
            $inner_vis inner: $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D, IS_REAL>,
        }
        unsafe impl<PTS: $crate::PrdTypes<IS_REAL>, D: ::core::marker::Send + ::core::marker::Sized, const IS_REAL: bool> ::core::marker::Send for $struct_name<PTS, D, IS_REAL> {}

        impl<PTS: $crate::PrdTypes<IS_REAL>, D: ::core::marker::Send + ::core::marker::Sized, const IS_REAL: bool> $struct_name<PTS, D, IS_REAL> {
            $method_vis fn new(inner: $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D, IS_REAL>) -> Self {
                Self { inner }
            }
            $method_vis fn from_plan_cost_table_data(plan: PTS::P, cost_table: PTS::CT, data: D) -> Self {
                // @TODO The following (listing generic params) fails!
                // Self::new(PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>::from_plan_data(plan, data))
                Self::new($crate::PrdInner::from_plan_cost_table_data(plan, cost_table, data))
            }
            $method_vis fn from_plan_cost_holder_data(plan: PTS::P, cost_holder: <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT, IS_REAL>>::COST, data: D) -> Self {
                // @TODO The following (listing generic params) fails!
                // Self::new(PrdInner<PTS::P, PTS::R, PTS::PRDHS, D>::from_plan_data(plan, data))
                Self::new($crate::PrdInner::from_plan_cost_holder_data(plan, cost_holder, data))
            }

            $method_vis fn from_real_data(real: PTS::R, data: D) -> Self {
                Self::new($crate::PrdInner::from_real_data(real, data))
            }

            $method_vis fn inner(self) -> $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D, IS_REAL> {
                self.inner
            }

            $method_vis fn plan_cost_table_data_moved(self) -> (PTS::P, PTS::CT, D) {
                PTS::PRDHS::plan_cost_table_data_moved(self.inner.plan_holder, self.inner.cost_holder, self.inner.data_holder)
            }
            $method_vis fn plan_cost_holder_data_moved(self) -> (PTS::P, <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT, IS_REAL>>::COST, D) {
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
            $method_vis fn data(&self) -> &D {
                self.inner.data_holder.data()
            }
            $method_vis fn data_mut(&mut self) -> &mut D {
                self.inner.data_holder.data_mut()
            }

            /// Whether we are in the `Plan` mode (rather than `Process` mode).
            $method_vis fn being_planned(&self) -> ::core::primitive::bool {
                self.inner.plan_holder.being_planned()
            }
            $method_vis fn advise_data_len(&mut self, len: usize) {
                $crate::unsupported!();
            }
            // @TODO remove
            /*$method_vis fn cost_each<F: ::core::ops::Fn($crate::Cost) -> $crate::Cost>(&self, f: F) -> impl ::core::ops::Fn() -> $crate::Cost {
                || self.inner.cost_each(f)
            }*/
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
        impl<PTS: $crate::PrdTypes<IS_REAL>, D: ::core::marker::Send + ::core::marker::Sized, const IS_REAL: bool> ::core::convert::From<$crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D, IS_REAL>> for $struct_name<PTS, D, IS_REAL> {
            fn from(value: $crate::PrdInner<PTS::P, PTS::R, PTS::CT, PTS::PRDHS, D, IS_REAL>) -> Self {
                Self::new(value)
            }
        }
    };
    // @TODO Can't match the following with empty (private) last visibility ($method_vis)?!
    ($struct_vis:vis, $inner_vis:vis, $method_vis:vis) => {
        /// Prd: Plan + Real + Data
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
            $vis type [<$struct_name Vec>]<PTS, T, const IS_REAL: bool> = $struct_name<PTS, ::alloc::vec::Vec<T>, IS_REAL>;
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
impl<PTS: PrdTypes<IS_REAL>, T: Send + Sized, const IS_REAL: bool> PrdBaseVec<PTS, T, IS_REAL> {
    pub fn map_leaf_uniform_cost_obj<R: Send + Sized, F: Fn(T) -> R>(
        self,
        each: F,
        cost_each: Cost,
    ) -> PrdBaseVec<PTS, R, IS_REAL> {
        if self.being_planned() {
            let (plan, cost_table, data) = self.plan_cost_table_data_moved();
            // @TODO Storage ops: If continuous input & from a sequential source, amortize access
            // time.
            //
            // RAM cost: We sum both the output AND *input* data (since input may be larger).

            PrdBaseVec::from_plan_cost_table_data(plan, cost_table, empty_vec())
        } else {
            let (real, data) = self.real_data_moved();

            let mut result = Vec::with_capacity(data.len());
            result.extend(data.into_iter().map(each));
            PrdBaseVec::from_real_data(real, result)
        }
    }

    pub fn vec_map_leaf_uniform_cost_holder<R: Send + Sized, F: Fn(T) -> R>(
        self,
        each: F,
        cost_holder_each: <<PTS as PrdTypes<IS_REAL>>::PRDHS as PlanRealDataHolders<
            PTS::P,
            PTS::R,
            PTS::CT,
            IS_REAL,
        >>::COST,
    ) -> PrdBaseVec<PTS, R, IS_REAL> {
        if self.being_planned() {
            let (plan, cost_holder, data) = self.plan_cost_holder_data_moved();
            // @TODO Storage ops: If continuous input & from a sequential source, amortize access
            // time.
            //
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

/// Carrying a (given) initial data size. Use with implementation of [ExactSizeIterator]. It can be
/// phantom - that is, if an [ExactSizeIterator] implementation, like [SkippableIterator],
/// implements [HasPhantomInitialSize], then that [HasPhantomInitialSize::phantom_initial_size] may
/// differ to the initial [ExactSizeIterator::len()] of that iterator.
pub trait HasPhantomInitialSize {
    fn phantom_initial_size(&self) -> usize;
}

/// Indicate how [Iterator::next] works for [SkippableIterator].
#[derive(PartialEq, Eq)]
enum SkippableIteratorMode {
    /// Pass through. [Iterator::next] returns whatever the underlying iterator returns.
    PassThrough,
    /// Skip. [Iterator::next] always returns [None].
    Skip,
    /// Whether to allow skip. Otherwise it [panic]s on skip. Good for checking of incorrect usage.
    PanicOnNext,
}
/// Used as a result type from our iteration-processing functions, so a function can return an empty
/// Iterator instead of a given one, yet have the same return type (closure/`impl`) in both cases.
/// Since iterators are lazy, if instantiated with [`SkippableIterator::new_skip`] or
/// [`SkippableIterator::new`] with argument `skip` being `true`, then the underlying Iterator is
/// not advanced.
///
/// If `I` is an [ExactSizeIterator], this implements [ExactSizeIterator], too. But, if skippable
/// (if `skip` is `true`), then [ExactSizeIterator::len] returns the underlying `self.iter.len()`,
/// even though `next()` returns [None].
pub struct SkippableIterator<T, I: Iterator<Item = T>, const IS_REAL: bool> {
    iter: I,
    //#[cfg(debug_assertions)]
    /// If true, then we do NOT access `iter`, but act as empty.
    mode: SkippableIteratorMode,
    /// Used only if this instance was instantiated with [SkippableIterator::new_panic_on_next_with_phantom_initial_size].
    phantom_initial_size: usize,
}
impl<T, I: Iterator<Item = T>, const IS_REAL: bool> Iterator for SkippableIterator<T, I, IS_REAL> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if IS_REAL {
            self.iter.next()
        } else {
            match self.mode {
                SkippableIteratorMode::Skip => None,
                SkippableIteratorMode::PanicOnNext => unsupported!(),
                _ => unreachable!(),
            }
        }
    }
}
unsafe impl<T: Send, I: Iterator<Item = T> + Send, const IS_REAL: bool> Send
    for SkippableIterator<T, I, IS_REAL>
{
}

impl<T, I: Iterator<Item = T>, const IS_REAL: bool> SkippableIterator<T, I, IS_REAL> {
    fn new(iter: I, mode: SkippableIteratorMode, phantom_initial_size: usize) -> Self {
        assert_eq!(mode == SkippableIteratorMode::PassThrough, IS_REAL);
        Self {
            iter,
            mode,
            phantom_initial_size,
        }
    }
}
impl<T, I: Iterator<Item = T>, const IS_REAL: bool> SkippableIterator<T, I, IS_REAL> {
    /// Use for const generic `IS_REAL` being `false` only. Otherwise this panics. (We make this
    /// implemented regardless of `IS_REAL`, so that we can use it in the same client code with either
    /// value of `IS_REAL`.)
    pub fn new_pass_through(iter: I) -> Self {
        Self::new(iter, SkippableIteratorMode::PassThrough, 0)
    }
    pub fn new_skip(iter: I) -> Self {
        Self::new(iter, SkippableIteratorMode::Skip, 0)
    }
}
/// We return the underlying size, but only if we are not skipping any items. Otherwise we return
/// `0` (or, if instantiated with [SkippableIterator::new_panic_on_next_with_phantom_initial_size],
/// we panic).
impl<T, I: ExactSizeIterator<Item = T>, const IS_REAL: bool> ExactSizeIterator
    for SkippableIterator<T, I, IS_REAL>
{
    fn len(&self) -> usize {
        match self.mode {
            SkippableIteratorMode::PassThrough => self.iter.len(),
            SkippableIteratorMode::Skip => 0,
            SkippableIteratorMode::PanicOnNext => unsupported!(),
        }
    }
}
impl<T, I: ExactSizeIterator<Item = T>, const IS_REAL: bool> SkippableIterator<T, I, IS_REAL> {
    pub fn new_with_phantom_skip(iter: I, phantom_initial_size: usize) -> Self {
        Self::new(iter, SkippableIteratorMode::Skip, phantom_initial_size)
    }
    pub fn new_with_phantom_panic_on_next(iter: I, phantom_initial_size: usize) -> Self {
        Self::new(
            iter,
            SkippableIteratorMode::PanicOnNext,
            phantom_initial_size,
        )
    }
}
impl<T, I: ExactSizeIterator<Item = T>, const IS_REAL: bool> HasPhantomInitialSize
    for SkippableIterator<T, I, IS_REAL>
{
    fn phantom_initial_size(&self) -> usize {
        match self.mode {
            SkippableIteratorMode::Skip => self.phantom_initial_size,
            SkippableIteratorMode::PassThrough | SkippableIteratorMode::PanicOnNext => {
                unsupported!()
            }
        }
    }
}
pub trait PhantomSizeIterator<const IS_REAL: bool>:
    ExactSizeIterator + HasPhantomInitialSize
{
}
impl<U: ExactSizeIterator + HasPhantomInitialSize, const IS_REAL: bool> PhantomSizeIterator<IS_REAL>
    for U
{
}

pub trait PhantomSizeSendIterator<const IS_REAL: bool>:
    PhantomSizeIterator<IS_REAL> + Send
{
}
impl<U: PhantomSizeIterator<IS_REAL> + Send, const IS_REAL: bool> PhantomSizeSendIterator<IS_REAL>
    for U
{
}

// TODO use
pub trait SendIterator: Iterator + Send {}
impl<U: Iterator + Send> SendIterator for U {}

pub trait SendSized: Send + Sized {}
impl<U: Send + Sized> SendSized for U {}

// Can't have the following:
//
//type PrdBaseIter<PTS: PrdTypes, T: Send + Sized, I: Iterator<Item = T> + Send> = PrdBase<PTS, I>;
//
// We can have the following, but it doesn't help much:
//
// type PrdBaseIter<PTS: PrdTypes, I: Iterator + Send> = PrdBase<PTS, I>;
impl<
        PTS: PrdTypes<IS_REAL>,
        T: Send + Sized,
        I: Iterator<Item = T> + Send,
        const IS_REAL: bool,
    > PrdBase<PTS, I, IS_REAL>
{
    /// Used only if the iterator `I` is not an [ExactSizeIterator]. Otherwise use [PrdBase::iter_exact_size_map_leaf_uniform_cost_holder_exact_size], if possible (or [PrdBase::iter_exact_size_map_leaf_uniform_cost_holder] otherwise).
    pub fn iter_map_leaf_uniform_cost_holder<R: Send + Sized, F: Fn(T) -> R + Send>(
        self,
        each: F,
        cost_holder_each: <<PTS as PrdTypes<IS_REAL>>::PRDHS as PlanRealDataHolders<
            PTS::P,
            PTS::R,
            PTS::CT,
            IS_REAL,
        >>::COST,
    ) -> PrdBase<PTS, impl Iterator<Item = R> + Send, IS_REAL> {
        if self.being_planned() {
            let (plan, cost_holder, data) = self.plan_cost_holder_data_moved();
            // @TODO Storage ops: If continuous input & from a sequential source, amortize access
            // time.
            //
            // @TODO RAM cost: We sum only the output.
            //
            // @TODO At the root level, sum the input data size, too.
            //
            // This is OK - because .map() is LAZY. We only have this to get the correct result
            // iterator type. The below [PhantomSizeSkippableIterator::new_skip] ensures that we
            // don't iterate over the original iterator.
            //
            // BUT, if that were not .map(), but a custom method, it could have had side effects, or
            // side costs.
            //
            // Alternatively, we could create an empty iterator with a settable (phantom) size.
            let result_to_skip = data.map(each);

            PrdBase::from_plan_cost_holder_data(
                plan,
                cost_holder,
                SkippableIterator::<_, _, IS_REAL>::new_skip(result_to_skip),
            )
        } else {
            let (real, data) = self.real_data_moved();

            let result = data.map(each);

            PrdBase::from_real_data(real, SkippableIterator::new_pass_through(result))
        }
    }
}

impl<
        PTS: PrdTypes<IS_REAL>,
        T: Send + Sized,
        I: PhantomSizeSendIterator<IS_REAL, Item = T>,
        const IS_REAL: bool,
    > PrdBase<PTS, I, IS_REAL>
{
    /// For data sources with exact (known) size, but when the transformation generates an iterator
    /// of an unknown/variable size.
    ///
    /// But, if the transformation is 1:1, use
    /// [PrdBase::iter_exact_size_map_leaf_uniform_cost_holder_exact_size] instead.
    pub fn iter_exact_size_map_leaf_uniform_cost_holder<R: Send + Sized, F: Fn(T) -> R + Send>(
        mut self,
        each: F,
        cost_holder_each: <<PTS as PrdTypes<IS_REAL>>::PRDHS as PlanRealDataHolders<
            PTS::P,
            PTS::R,
            PTS::CT,
            IS_REAL,
        >>::COST,
    ) -> PrdBase<PTS, impl Iterator<Item = R> + Send, IS_REAL> {
        let len = self.data().phantom_initial_size();
        self.advise_data_len(len);

        if self.being_planned() {
            let (plan, cost_holder, data) = self.plan_cost_holder_data_moved();
            // @TODO Storage ops: If continuous input & from a sequential source, amortize access
            // time.
            //
            // @TODO RAM cost: We sum only the output.
            //
            // @TODO At the root level, sum the input data size, too.
            let result_to_skip = data.map(each);

            PrdBase::from_plan_cost_holder_data(
                plan,
                cost_holder,
                SkippableIterator::<_, _, IS_REAL>::new_skip(result_to_skip),
            )
        } else {
            let (real, data) = self.real_data_moved();

            let result = data.map(each);

            PrdBase::from_real_data(real, SkippableIterator::new_pass_through(result))
        }
    }

    /// For data sources with exact (known) size, and the transformation generates an iterator of a
    /// known/exact size, too.
    pub fn iter_exact_size_map_leaf_uniform_cost_holder_exact_size<
        R: Send + Sized,
        F: Fn(T) -> R + Send,
    >(
        mut self,
        each: F,
        cost_holder_each: impl Fn() -> <<PTS as PrdTypes<IS_REAL>>::PRDHS as PlanRealDataHolders<
            PTS::P,
            PTS::R,
            PTS::CT,
            IS_REAL,
        >>::COST,
    ) -> PrdBase<PTS, impl PhantomSizeSendIterator<IS_REAL, Item = R>, IS_REAL> {
        let len = self.data().phantom_initial_size();
        self.advise_data_len(len);

        if self.being_planned() {
            let (plan, cost_holder, data) = self.plan_cost_holder_data_moved();
            // @TODO Storage ops: If continuous input & from a sequential source, amortize access
            // time.
            //
            // @TODO RAM cost: We sum only the output.
            //
            // @TODO At the root level, sum the input data size, too.
            //
            // @TODO eliminate this
            //
            // AND
            //
            // keep the `data` iterator at the higher level, so that we can re-run in Process mode
            // (instead of Plan mode)
            //
            // For that (to keep the `data` iterator at the higher level in Plan mode),
            // `iter_exact_size_map_leaf_uniform_cost_holder_exact_size` and similar methods have
            // NOT to consume `self`, but to take `&mut self`. Then we `&mut` borrow `plan`,
            // `cost_holder` and `data` (out of `&mut self`).
            //
            // But, in the `Process/Execute` if-else branch below, we move `real` out of `&mut
            // self`h. So PrdBase will need to store `real` in an `Option<R>`, so it can be moved
            // out.
            //
            // We do NOT need to invoke `each`, except if it's an inner Iterator. But we'll handle
            // that in special functions of `PrdBase` instead.
            let result_to_skip = data.map(each);

            PrdBase::from_plan_cost_holder_data(
                plan,
                cost_holder,
                SkippableIterator::<_, _, IS_REAL>::new_with_phantom_panic_on_next(
                    result_to_skip,
                    len,
                ),
            )
        } else {
            let (real, data) = self.real_data_moved();

            let result = data.map(each);

            PrdBase::from_real_data(real, SkippableIterator::new_pass_through(result))
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
        impl<PTS: $crate::PrdTypes<IS_REAL>, T: ::core::marker::Send + ::core::marker::Sized, const IS_REAL: bool> $struct_name<PTS, ::alloc::vec::Vec<T>, IS_REAL> {
            $vis fn map_leaf_uniform_cost_obj<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R>(
                self, each: F, cost_each: $crate::Cost
            ) -> $struct_name<PTS, ::alloc::vec::Vec<R>, IS_REAL> {
                $crate::PrdBaseVec::<PTS, T, IS_REAL>::from(self.inner())
                    .map_leaf_uniform_cost_obj(each, cost_each)
                    .inner()
                    .into()
            }

            $vis fn vec_map_leaf_uniform_cost_holder<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R>(
                self, each: F,
                cost_holder_each: <<PTS as $crate::PrdTypes<IS_REAL>>::PRDHS as $crate::PlanRealDataHolders<
                PTS::P,
                PTS::R,
                PTS::CT,
                IS_REAL
            >>::COST
            ) -> $struct_name<PTS, ::alloc::vec::Vec<R>, IS_REAL> {
                $crate::PrdBaseVec::<PTS, T, IS_REAL>::from(self.inner())
                    .vec_map_leaf_uniform_cost_holder(each, cost_holder_each)
                    .inner()
                    .into()
            }
        }

        impl<
        PTS: $crate::PrdTypes<IS_REAL>,
        T: ::core::marker::Send + ::core::marker::Sized,
        I: ::core::iter::Iterator<Item = T> + ::core::marker::Send,
        const IS_REAL: bool
        >
        $struct_name<PTS, I, IS_REAL> {
            $vis fn iter_map_leaf_uniform_cost_holder<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R + ::core::marker::Send>(
                self, each: F, cost_holder_each: <<PTS as $crate::PrdTypes<IS_REAL>>::PRDHS as $crate::PlanRealDataHolders<
                PTS::P,
                PTS::R,
                PTS::CT,
                IS_REAL
            >>::COST
            ) -> $struct_name<PTS, impl ::core::iter::Iterator<Item = R> + ::core::marker::Send, IS_REAL> {
                $crate::PrdBase::<PTS, I, IS_REAL>::from(self.inner())
                    .iter_map_leaf_uniform_cost_holder(each, cost_holder_each)
                    .inner()
                    .into()
            }
        }

        impl<
        PTS: $crate::PrdTypes<IS_REAL>,
        T: ::core::marker::Send + ::core::marker::Sized,
        I: $crate::PhantomSizeSendIterator<IS_REAL, Item = T>,
        const IS_REAL: bool
        >
        $struct_name<PTS, I, IS_REAL> {
            $vis fn iter_exact_size_map_leaf_uniform_cost_holder<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R + ::core::marker::Send>(
                self, each: F, cost_holder_each: <<PTS as $crate::PrdTypes<IS_REAL>>::PRDHS as $crate::PlanRealDataHolders<
                PTS::P,
                PTS::R,
                PTS::CT,
                IS_REAL
            >>::COST
            ) -> $struct_name<PTS, impl ::core::iter::Iterator<Item = R> + ::core::marker::Send, IS_REAL> {
                $crate::PrdBase::<PTS, I, IS_REAL>::from(self.inner())
                    .iter_exact_size_map_leaf_uniform_cost_holder(each, cost_holder_each)
                    .inner()
                    .into()
            }

            $vis fn iter_exact_size_map_leaf_uniform_cost_holder_exact_size<R: ::core::marker::Send + ::core::marker::Sized,
            F: ::core::ops::Fn(T) -> R + ::core::marker::Send>(
                self,
                each: F,
                cost_holder_each: impl ::core::ops::Fn() -> <<PTS as $crate::PrdTypes<IS_REAL>>::PRDHS as $crate::PlanRealDataHolders<
                PTS::P,
                PTS::R,
                PTS::CT,
                IS_REAL
            >>::COST
            ) -> $struct_name<PTS, impl $crate::PhantomSizeSendIterator<IS_REAL, Item = R>, IS_REAL> {
                $crate::PrdBase::<PTS, I, IS_REAL>::from(self.inner())
                    .iter_exact_size_map_leaf_uniform_cost_holder_exact_size(each, cost_holder_each)
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
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {}
}
