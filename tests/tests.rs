use core::marker::PhantomData;

use distrib::{Plan, PlanRealData, PlanRealDataHolders, Prd, Real, TrackT};

// #[derive(Track)]
// #[track_data]
//struct MyResult {}
//impl Tracked for MyResult {}

/*impl Track<MyResult> {
    fn f() {}
}*/

pub trait TrackChildT<T> {
    fn p(&self) -> T {
        loop {}
    }
}

// USER + generated?
/// - `P`: Plan
/// - `R`: incoming main parameter & outcoming result (in Process mode).
pub struct Tct<P: Plan, D: Send + Sized> {
    _p: PhantomData<P>,
    _r: PhantomData<D>,
}
// USER:
impl<P: Plan, D: Send + Sized> TrackChildT<D> for Tct<P, D> {}

// generated:
impl<P: Plan, R: Real, D: Send + Sized> TrackT<P, R, D> for Tct<P, D> {
    fn process_t(&self) -> D {
        self.p()
        // OR (if we need to disambiguate):
        //
        // <Self as TrackChildT<T>>::p(self)
    }
}

impl<P: Plan, D: Send + Sized> Tct<P, D> {
    // How do I eliminate `T` data/size from being used in Plan mode, yet have the client code use
    // `T` without extra typecasting and extra cost?
    pub fn plan_or_process(self) -> Self {
        self
    }
}
//-----------------

pub struct Processing<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>> {
    _p: PhantomData<P>,
    _r: PhantomData<R>,
    _prdh: PhantomData<PRDHS>,
}

//impl<P: Plan, R: Real, PRDH: PlanRealDataHolders<P, R>> Processing<P, R, PRDH> {
pub fn f<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>>(
    prd: Prd<P, R, PRDHS, &str>,
) -> Prd<P, R, PRDHS, String> {
    if prd.is_plan() {
        // plan
        let (plan, data) = prd.plan_data_moved();
        // Copy of the original code
    } else {
        let (real, data) = prd.real_data_moved();
        // Copy of the original code
    }
    loop {}
}

pub struct S {}

//impl <P: Plan, R: Real, PRDH: PlanRealDataHolders<P, R>> Prd<P, R, PRDH, S> {}
