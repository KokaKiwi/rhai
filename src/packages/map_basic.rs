#![cfg(not(feature = "no_object"))]

use crate::def_package;
use crate::dynamic::Dynamic;
use crate::engine::Map;
use crate::parser::{ImmutableString, INT};
use crate::plugin::*;

#[cfg(not(feature = "no_index"))]
use crate::engine::Array;

def_package!(crate:BasicMapPackage:"Basic object map utilities.", lib, {
    combine_with_exported_module!(lib, "map", map_functions);
});

#[export_module]
mod map_functions {
    pub fn has(map: &mut Map, prop: ImmutableString) -> bool {
        map.contains_key(&prop)
    }
    pub fn len(map: &mut Map) -> INT {
        map.len() as INT
    }
    pub fn clear(map: &mut Map) {
        map.clear();
    }
    pub fn remove(x: &mut Map, name: ImmutableString) -> Dynamic {
        x.remove(&name).unwrap_or_else(|| ().into())
    }
    #[rhai_fn(name = "mixin", name = "+=")]
    pub fn mixin(map1: &mut Map, map2: Map) {
        map2.into_iter().for_each(|(key, value)| {
            map1.insert(key, value);
        });
    }
    #[rhai_fn(name = "+")]
    pub fn merge(mut map1: Map, map2: Map) -> Map {
        map2.into_iter().for_each(|(key, value)| {
            map1.insert(key, value);
        });
        map1
    }
    pub fn fill_with(map1: &mut Map, map2: Map) {
        map2.into_iter().for_each(|(key, value)| {
            map1.entry(key).or_insert(value);
        });
    }

    #[cfg(not(feature = "no_index"))]
    pub mod indexing {
        pub fn keys(map: &mut Map) -> Array {
            map.iter().map(|(k, _)| k.clone().into()).collect()
        }
        pub fn values(map: &mut Map) -> Array {
            map.iter().map(|(_, v)| v.clone()).collect()
        }
    }
}
