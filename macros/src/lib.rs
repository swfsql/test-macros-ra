#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn my_macro_attr(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        #[macro_use]
        mod exported_macro {
            #[macro_export]
            macro_rules! my_macro {
                () => {
                    pub struct CreatedByMyMacro;
                };
            }
        }
    })
}
