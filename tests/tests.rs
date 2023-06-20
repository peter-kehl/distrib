#![allow(unused_variables)]
use core::borrow::Borrow;
use std::fmt::Display;

use distrib::{
    Cost, CostHolder, CostTable, DataHolder, PhantomSizeIterator, Plan, PlanHolder,
    PlanRealDataHolders, PrdInner, PrdTypes, Real, RealHolder,
};

extern crate alloc;

pub fn to_uppercase_f<
    P: Plan,
    R: Real,
    CT: CostTable<REAL>,
    PRDHS: PlanRealDataHolders<P, R, CT, REAL>,
    const REAL: bool,
>(
    prd: PrdInner<P, R, CT, PRDHS, &str, REAL>,
) -> PrdInner<P, R, CT, PRDHS, String, REAL> {
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

pub fn to_uppercase_g<PTS: PrdTypes<REAL>, const REAL: bool>(
    prd: Prd<PTS, &str, REAL>,
) -> Prd<PTS, String, REAL> {
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

impl<PTS: PrdTypes<REAL>, const REAL: bool> Prd<PTS, &str, REAL> {
    pub fn to_uppercase(self) -> Prd<PTS, String, REAL> {
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

impl<PTS: PrdTypes<REAL>, const REAL: bool> Prd<PTS, String, REAL> {
    pub fn to_uppercase(self) -> Prd<PTS, String, REAL> {
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

impl<PTS: PrdTypes<REAL>, const REAL: bool> Prd<PTS, Vec<u8>, REAL> {
    pub fn to_word(self) -> Prd<PTS, String, REAL> {
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

impl<PTS: PrdTypes<REAL>, const REAL: bool> Prd<PTS, Vec<u8>, REAL> {
    pub fn to_word_uppercase(self) -> Prd<PTS, String, REAL> {
        self.to_word().to_uppercase()
    }
}

distrib::generate_prd_struct_aliases!(pub, Prd);
distrib::generate_prd_base_proxies!();
const ZERO_COST: Cost = distrib::default_cost();

// @TODO report: Fails:
//
//type PTS_COST<PTS> = <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST;
#[allow(type_alias_bounds)]
type PtsCost<PTS: PrdTypes<REAL>, const REAL: bool> =
    <PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT, REAL>>::COST;

impl<PTS: PrdTypes<REAL>, const REAL: bool> PrdVec<PTS, u8, REAL> {
    pub fn vec_inc_through_single_cost(self) -> PrdVec<PTS, u8, REAL> {
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
    pub fn vec_prefix_cost_holder(self, prefix: &str) -> PrdVec<PTS, String, REAL> {
        let being_planned = self.being_planned();

        self.vec_map_leaf_uniform_cost_holder(
            |v| format!("{prefix}{v}"),
            if being_planned {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::from_cost(
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                //<PTS::PRDHS as PlanRealDataHolders<PTS::P, PTS::R, PTS::CT>>::COST::empty()
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }
}

impl<'s, PTS: PrdTypes<REAL> + 's, I: Iterator<Item = &'s str> + Send + 's, const REAL: bool>
    Prd<PTS, I, REAL>
{
    pub fn iter_map_str_uppercase_cost_holder(
        self,
    ) -> Prd<PTS, impl Iterator<Item = String> + Send + 's, REAL> {
        let being_planned = self.being_planned();

        self.iter_map_leaf_uniform_cost_holder(
            move |v| v.to_uppercase(),
            if being_planned {
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }
}

/// [Into]`<&'s str>` doesn't work well, because we can't have an orphaned slice.
///
/// Instead, we use [Borrow]`<str>`. See [Prd::iter_map_borrow_str_uppercase_cost_holder].
impl<
        's,
        T: Into<&'s str> + Send,
        PTS: PrdTypes<REAL>,
        I: Iterator<Item = T> + Send,
        const REAL: bool,
    > Prd<PTS, I, REAL>
{
    pub fn iter_map_into_str_uppercase_cost_holder(
        self,
    ) -> Prd<PTS, impl Iterator<Item = String> + Send, REAL> {
        if true {
            unimplemented!("Easy to implement, but useless.");
        }
        let being_planned = self.being_planned();
        self.iter_map_leaf_uniform_cost_holder(
            move |v| v.into().to_uppercase(),
            if being_planned {
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }
}

impl<
        's,
        T: Borrow<str> + Send,
        PTS: PrdTypes<REAL>,
        I: Iterator<Item = T> + Send,
        const REAL: bool,
    > Prd<PTS, I, REAL>
{
    pub fn iter_map_borrow_str_uppercase_cost_holder(
        self,
    ) -> Prd<PTS, impl Iterator<Item = String> + Send, REAL> {
        let being_planned = self.being_planned();

        self.iter_map_leaf_uniform_cost_holder(
            move |v| v.borrow().to_uppercase(),
            if being_planned {
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }
}

impl<
        's,
        T: Borrow<str> + Send,
        PTS: PrdTypes<REAL>,
        I: PhantomSizeIterator<REAL, Item = T> + Send,
        const REAL: bool,
    > Prd<PTS, I, REAL>
{
    pub fn iter_exact_size_map_borrow_str_uppercase_cost_holder(
        self,
    ) -> Prd<PTS, impl PhantomSizeIterator<REAL, Item = String> + Send, REAL> {
        let being_planned = self.being_planned();

        self.iter_exact_size_map_leaf_uniform_cost_holder_exact_size(
            move |v| v.borrow().to_uppercase(),
            if being_planned {
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }
}

impl<
        T: Into<String> + Send,
        PTS: PrdTypes<REAL>,
        I: Iterator<Item = T> + Send,
        const REAL: bool,
    > Prd<PTS, I, REAL>
{
    pub fn iter_map_into_string_uppercase_cost_holder(
        self,
    ) -> Prd<PTS, impl Iterator<Item = String> + Send, REAL> {
        let being_planned = self.being_planned();
        self.iter_map_leaf_uniform_cost_holder(
            move |v| v.into().to_uppercase(),
            if being_planned {
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }
}

impl<
        's,
        PTS: PrdTypes<REAL> + 's,
        T: Send + Sized + Display + 's,
        I: Iterator<Item = T> + Send + 's,
        const REAL: bool,
    > Prd<PTS, I, REAL>
{
    pub fn iter_map_prefix_cost_holder(
        self,
        prefix: &'s str,
    ) -> Prd<PTS, impl Iterator<Item = String> + Send + 's, REAL> {
        let being_planned = self.being_planned();
        self.iter_map_leaf_uniform_cost_holder(
            move |v| format!("{prefix}{v}"),
            if being_planned {
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }

    /// Chained operations, with no intermediary storage.
    pub fn iter_map_prefix_and_uppercase_cost_holder(
        self,
        prefix: &'s str,
    ) -> Prd<PTS, impl Iterator<Item = String> + Send + 's, REAL> {
        self.iter_map_prefix_cost_holder(prefix)
            .iter_map_borrow_str_uppercase_cost_holder()
    }
}

impl<
        's,
        PTS: PrdTypes<REAL> + 's,
        T: Send + Sized + Display + 's,
        I: PhantomSizeIterator<REAL, Item = T> + Send + 's,
        const REAL: bool,
    > Prd<PTS, I, REAL>
{
    pub fn iter_exact_size_map_prefix_cost_holder(
        self,
        prefix: &'s str,
    ) -> Prd<PTS, impl PhantomSizeIterator<REAL, Item = String> + Send + 's, REAL> {
        let being_planned = self.being_planned();
        self.iter_exact_size_map_leaf_uniform_cost_holder_exact_size(
            move |v| format!("{prefix}{v}"),
            // @TODO into a closure, move up. The `else` branch is what matters for production -
            // then cost calculation is skipped. Factor out into a closure-accepting funtion.
            //
            // Also, instead of listing Cost {...} constructor, take |c: &mut Cost| and chain-call
            // functions like cpu(&mut self, new_cpu_value: f32) -> Self.
            if being_planned {
                <PtsCost<PTS, REAL>>::from_cost(Cost {
                    cpu: 1.0,
                    ..Cost::default()
                })
            } else {
                <PtsCost<PTS, REAL>>::empty()
            },
        )
    }

    /// Chained operations, with no intermediary storage.
    pub fn iter_exact_size_map_prefix_and_uppercase_cost_holder(
        self,
        prefix: &'s str,
    ) -> Prd<PTS, impl ExactSizeIterator<Item = String> + Send + 's, REAL> {
        self.iter_exact_size_map_prefix_cost_holder(prefix)
            .iter_exact_size_map_borrow_str_uppercase_cost_holder()
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

pub fn instantiate_outside_struct<PTS: PrdTypes<REAL>, const REAL: bool>() {
    #[allow(unreachable_code)]
    let _prd = Prd::<PTS, u8, REAL>::new(loop {});
}

distrib::generate_prd_struct!(pub, , pub, Prd2);

pub fn move_between_prds<PTS: PrdTypes<REAL>, const REAL: bool>() {
    #[allow(unreachable_code)]
    let prd = Prd::<PTS, u8, REAL>::new(loop {});
    let prd2: Prd2<PTS, u8, REAL> = prd.inner.into();
    // OR the same the other way, but using .inner() method:
    let prd_again: Prd<PTS, u8, REAL> = prd2.inner().into();

    let _prd2_again = Prd2::<PTS, u8, REAL>::from(prd_again.inner());
}

/// To avoid adding `<PTS: PrdTypes>` generic parameter to every function, we could workaround with a
/// trait and have that `PTS` as an associated type of that trait.
///
/// Any functions are implemented in the trait (as default implementations). Of course, you can add
/// generic parameters to the trait, as needed.
///
/// But, this doesn't save much. Plus, invoking these functions is *less* ergonomic.
pub trait T<const REAL: bool> {
    type PTS: PrdTypes<REAL>;

    fn f(prd: Prd<Self::PTS, &str, REAL>) -> Prd<Self::PTS, String, REAL> {
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
