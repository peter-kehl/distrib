use core::marker::PhantomData;

use distrib::{
    DataHolder, Plan, PlanHolder, PlanRealData, PlanRealDataHolders, PrdInner, PrdTypes, Real,
    RealHolder,
};

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
/*impl<P: Plan, R: Real, D: Send + Sized> TrackT<P, R, D> for Tct<P, D> {
    fn process_t(&self) -> D {
        self.p()
        // OR (if we need to disambiguate):
        //
        // <Self as TrackChildT<T>>::p(self)
    }
}*/

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
//impl<P: Plan, R: Real, PRDH: PlanRealDataHolders<P, R>> Processing<P, R, PRDH> {}

pub fn to_uppercase_f<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>>(
    prd: PrdInner<P, R, PRDHS, &str>,
) -> PrdInner<P, R, PRDHS, String> {
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

distrib::generate_prd_struct!(pub, , pub); // OK
// distrib::generate_prd_struct!(pub, , ); -- NOT OK!
//distrib::generate_prd_struct!(pub, , , Prd); // OK

pub fn to_uppercase_g<PTS: PrdTypes>(prd: Prd<PTS, &str>) -> Prd<PTS, String> {
    if prd.being_planned() {
        // plan -> update/add to Plan
        let (plan, data) = prd.plan_data_moved();
        // Copy of the original code, OR adjust manually
    } else {
        // execute & collect Real costs
        let (real, data) = prd.real_data_moved();
        // Copy of the original code
    }
    loop {}
}

impl<PTS: PrdTypes> Prd<PTS, &str> {
    pub fn to_uppercase(self) -> Prd<PTS, String> {
        if self.being_planned() {
            // plan
            let (plan, data) = self.plan_data_moved();
            // Copy of the original code
        } else {
            let (real, data) = self.real_data_moved();
            // Copy of the original code
        }
        loop {}
    }
}

impl<PTS: PrdTypes> Prd<PTS, String> {
    pub fn to_uppercase(self) -> Prd<PTS, String> {
        if self.being_planned() {
            // plan
            let (plan, data) = self.plan_data_moved();
            // Copy of the original code
        } else {
            let (real, data) = self.real_data_moved();
            // Copy of the original code
        }
        loop {}
    }
}

impl<PTS: PrdTypes> Prd<PTS, Vec<u8>> {
    pub fn to_word(self) -> Prd<PTS, String> {
        if self.being_planned() {
            // plan
            let (plan, data) = self.plan_data_moved();
            // Copy of the original code
        } else {
            let (real, data) = self.real_data_moved();
            // Copy of the original code
        }
        loop {}
    }
}

impl<PTS: PrdTypes> Prd<PTS, Vec<u8>> {
    pub fn to_word_uppercase(self) -> Prd<PTS, String> {
        self.to_word().to_uppercase()
    }

    pub async fn f() {}
}

pub fn instantiate_outside_struct<PTS: PrdTypes>() {
    #[allow(unreachable_code)]
    let _prd = Prd::<PTS, u8>::new(loop {});
}
distrib::generate_prd_struct!(pub, , pub, Prd2);
pub fn move_between_prds<PTS: PrdTypes>() {
    #[allow(unreachable_code)]
    let prd = Prd::<PTS, u8>::new(loop {});
    let prd2: Prd2<PTS, u8> = prd.inner.into();
    // OR the same the other way, but using .inner() method:
    let _prd_again: Prd<PTS, u8> = prd2.inner().into();
}

/// To avoid adding `<PTS: PrdTypes>` generic parameter to every function, we can workaround with a
/// trait and have that `PTS` as an associated type of that trait.
///
/// Any functions are implemented in the trait (as default implementations). Of course, you can add
/// generic parameters to the trait, as needed.
///
/// But, this doesn't save much. Plus, invoking these functions is *less* ergonomic.
pub trait T {
    type PTS: PrdTypes;

    fn f(prd: Prd<Self::PTS, &str>) -> Prd<Self::PTS, String> {
        if prd.being_planned() {
            // plan
            let (plan, data) = prd.plan_data_moved();
            // Copy of the original code
        } else {
            let (real, data) = prd.real_data_moved();
            // Copy of the original code
        }
        loop {}
    }
}

//impl <P: Plan, R: Real, PRDH: PlanRealDataHolders<P, R>> Prd<P, R, PRDH, S> {}
