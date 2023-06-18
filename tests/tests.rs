use core::marker::PhantomData;

use distrib::{
    Cost, DataHolder, Plan, PlanHolder, PlanRealData, PlanRealDataHolders, PrdInner, PrdTypes,
    Real, RealHolder,
};

extern crate alloc;

pub struct Processing<P: Plan, R: Real, PRDHS: PlanRealDataHolders<P, R>> {
    _p: PhantomData<P>,
    _r: PhantomData<R>,
    _prdh: PhantomData<PRDHS>,
}

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
//
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

distrib::generate_prd_struct_aliases!(pub, Prd);
distrib::generate_prd_base_proxies!();

impl<PTS: PrdTypes> PrdVec<PTS, u8> {
    pub fn inc(self) -> PrdVec<PTS, u8> {
        self.map_leaf_uniform(
            |v| v + 1,
            Cost {
                cpu: 1.0,
                ..Cost::default()
            },
        )
    }
    pub fn prefix(self, prefix: &str) -> PrdVec<PTS, String> {
        self.map_leaf_uniform(
            |v| format!("{prefix}{v}"),
            Cost {
                cpu: 1.0,
                ..Cost::default()
            },
        )
    }
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
    let prd_again: Prd<PTS, u8> = prd2.inner().into();

    let _prd2_again = Prd2::<PTS, u8>::from(prd_again.inner());
}

/// To avoid adding `<PTS: PrdTypes>` generic parameter to every function, we could workaround with a
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
