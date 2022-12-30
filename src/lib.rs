mod client;
mod request;
pub use crate::client::{
    examples::{examplenewtrans, exampletranslate},
    maintenance::ready,
    newtrans::newtrans,
    translate::translate,
};
