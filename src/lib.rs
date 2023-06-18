// https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
// https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
#![feature(async_fn_in_trait)]
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

pub trait Plan: Send + Sized {}
pub trait Real: Send + Sized {}
//pub trait Data: Send + Sized {}
//impl Data for u8 {}
//...

pub trait TrackT<P: Plan, R: Real, D: Send + Sized> {
    fn plan(&self) -> () { // return a closure?
    }

    fn process_t(&self) -> D {
        loop {}
    }
}

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
    fn is_plan(&self) -> bool {
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
pub struct Prd<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>, D: Send + Sized> {
    _p: PhantomData<P>,
    _r: PhantomData<R>,
    _prdh: PhantomData<PRDHS>,
    _d: PhantomData<D>,

    plan_holder: PRDHS::PLAN,
    real_holder: PRDHS::REAL,
    data_holder: PRDHS::DATA<D>,
}
unsafe impl<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>, D: Send + Sized> Send
    for Prd<P, R, PRDHS, D>
{
}

impl<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>, D: Send + Sized> Prd<P, R, PRDHS, D> {
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
        self.plan_holder.is_plan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
