use std::collections::{BTreeMap, BTreeSet, HashMap};
use serde::{Deserialize, Serialize};
use log::{debug, warn};
use crate::NodeId;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ResolutionMap<T>(HashMap<T, T>)
    where
        T: Eq + Clone + std::hash::Hash + Default;

impl<T: Eq + Clone + std::hash::Hash + Default> ResolutionMap<T> {
    pub fn new() -> Self {
        ResolutionMap(HashMap::new())
    }

    pub fn insert(&mut self, pointer_id: T, pointee_id: T) {
        if self.0.insert(pointer_id, pointee_id).is_some() {
            debug!("Overriding resolution");
        }
    }

    pub fn get(&self, pointer_id: &T) -> Option<T> {
        self.0.get(pointer_id).cloned()
    }

    pub fn get_recur(&self, pointer_id: &T) -> Option<T> {
        self.get(pointer_id).and_then(|pointee_id| {
            if *pointer_id == pointee_id {
                warn!("Resolution loop");

                Some(pointee_id)
            } else {
                self.get_recur(&pointee_id).or(Some(pointee_id))
            }
        })
    }

    pub fn get_map(&self) -> HashMap<T, T> {
        self.0.clone()
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn remove(&mut self, item: &T) {
        self.0.remove(item);
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn inner(&self) -> &HashMap<T, T> {
        &self.0
    }
}



#[derive(Debug, Default, Clone)]
pub struct TraitSolver {
    pub implemented_trait: BTreeMap<String, BTreeSet<String>>, // implementors -> trait
    pub implemented_fns: BTreeMap<String, BTreeMap<NodeId, String>>, // implementor -> (fn_hir_id, fn_name)
    pub trait_methods: BTreeMap<String, BTreeMap<NodeId, String>>, // trait/struct -> (method_hir_id, method_name)
}

impl TraitSolver {
    pub fn new() -> TraitSolver {
        TraitSolver {
            implemented_trait: BTreeMap::new(),
            trait_methods: BTreeMap::new(),
            implemented_fns: BTreeMap::new(),
        }
    }

    // pub fn add_impl(&mut self, tr: &Impl) {
    //     let effective_type = if tr.types.is_empty() {
    //         tr.name.get_name()
    //     } else {
    //         tr.types[0].get_name()
    //     };
    //
    //     self.implemented_fns
    //         .entry(effective_type)
    //         .or_insert(BTreeMap::new())
    //         .extend(
    //             tr.defs
    //                 .iter()
    //                 .map(|fundecl| (fundecl.node_id, fundecl.name.name.clone()))
    //                 .collect::<Vec<_>>(),
    //         );
    //
    //     self.trait_methods
    //         .entry(tr.name.get_name())
    //         .or_insert(BTreeMap::new())
    //         .extend(
    //             tr.defs
    //                 .iter()
    //                 .map(|fundecl| (fundecl.node_id, fundecl.name.to_string()))
    //                 .collect::<Vec<_>>(),
    //         );
    // }
    //
    // pub fn add_implementor(&mut self, implementor_type: Type, trait_type: Type) {
    //     self.implemented_trait
    //         .entry(implementor_type.get_name())
    //         .or_insert_with(BTreeSet::new)
    //         .insert(trait_type.get_name());
    // }
    //
    // pub fn node_id_of_fn_implementor(
    //     &self,
    //     implementor_type: &Type,
    //     fn_name: String,
    // ) -> Option<NodeId> {
    //     self.implemented_fns
    //         .get(&implementor_type.get_name())
    //         .and_then(|set| {
    //             set.iter()
    //                 .find(|(_, name)| **name == fn_name)
    //                 .map(|(id, _)| id.clone())
    //         })
    // }
}
