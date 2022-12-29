mod client;
mod request;
pub use crate::client::{
    examples::{examplenewtrans, exampletranslate},
    maintenance::{ready, Ready},
    newtrans::newtrans,
    translate::translate,
};
