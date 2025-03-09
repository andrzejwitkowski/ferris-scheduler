use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn task_definition(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        #input

        impl Default for #name {
            fn default() -> Self {
                Self {}
            }
        }
        
        // Add a function to register this task handler with a scheduler
        impl #name {
            pub fn register(scheduler: &mut impl crate::domain::port::task_scheduler::TaskScheduler) -> Result<(), crate::domain::port::task_scheduler::TaskSchedulerError> {
                let handler = Self::default();
                scheduler.register_handler(Box::new(handler))
            }
        }
    };

    TokenStream::from(expanded)
}