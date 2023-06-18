use core::marker::PhantomData;
use std::fmt::Display;

use distrib::{
    Cost, CostHolder, CostTable, DataHolder, Plan, PlanHolder, PlanRealData, PlanRealDataHolders,
    PrdInner, PrdTypes, Real, RealHolder,
};

extern crate alloc;

pub fn to_uppercase_f<P: Plan, R: Real, CT: CostTable, PRDHS: PlanRealDataHolders<P, R, CT>>(
    prd: PrdInner<P, R, CT, PRDHS, &str>,
) -> PrdInner<P, R, CT, PRDHS, String> {
    if prd.is_plan() {
        // plan
        let (plan, cost_table, data) = prd.plan_cost_table_data_moved();
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
        let (plan, cost_table, data) = prd.plan_cost_table_data_moved();
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
            let (plan, cost_table, data) = self.plan_cost_table_data_moved();
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
            let (plan, cost_table, data) = self.plan_cost_table_data_moved();
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
            let (plan, cost_table, data) = self.plan_cost_table_data_moved();
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
const ZERO_COST: Cost = distrib::default_cost();

// @TODO report: Fails:
//
//type PTS_COST<PTS> = <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST;
#[allow(type_alias_bounds)]
type PTS_COST<PTS: PrdTypes> = <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST;

impl<PTS: PrdTypes> PrdVec<PTS, u8> {
    pub fn vec_inc_through_single_cost(self) -> PrdVec<PTS, u8> {
        let being_planned = self.being_planned();
        self.map_leaf_uniform_cost_obj(
            |v| v + 1,
            if being_planned {
                Cost {
                    cpu: 1.0,
                    ..Cost::default()
                }
            } else {
                ZERO_COST
            },
        )
    }
    pub fn vec_prefix_through_holder(self, prefix: &str) -> PrdVec<PTS, String> {
        let being_planned = self.being_planned();
        self.vec_map_leaf_uniform_cost_holder(
            |v| format!("{prefix}{v}"),
            if being_planned {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::from_cost(
                <PTS_COST<PTS>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::empty()
                <PTS_COST<PTS>>::empty()
            },
        )
    }
}

impl<'s, PTS: PrdTypes, T: Send + Sized + Display + 's, I: Iterator<Item = T> + Send + 's>
    Prd<PTS, I>
{
    pub fn iter_map_prefix_through_holder(
        self,
        prefix: &'s str,
    ) -> Prd<PTS, impl Iterator<Item = String> + Send + '_> {
        let being_planned = self.being_planned();
        self.iter_map_leaf_uniform_cost_holder(
            move |v| format!("{prefix}{v}"),
            if being_planned {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::from_cost(
                <PTS_COST<PTS>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::empty()
                <PTS_COST<PTS>>::empty()
            },
        )
    }
}

/*impl <PTS: PrdTypes, T: Send + Sized, I: Iterator<Item = T> + Send> Prd<PTS, I> {
    pub fn iter_prefix_through_holder(self, prefix: &str) -> Prd<PTS, impl Iterator<Item = String>> {
        let being_planned = self.being_planned();
        self.map_leaf_uniform_cost_holder(
            |v| format!("{prefix}{v}"),
            if being_planned {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::from_cost(
                <PTS_COST<PTS>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::empty()
                <PTS_COST<PTS>>::empty()
            },
        )
    }
}*/

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
            let (plan, cost_table, data) = prd.plan_cost_table_data_moved();
            // Copy of the original code
        } else {
            let (real, data) = prd.real_data_moved();
            // Copy of the original code
        }
        loop {}
    }
}
