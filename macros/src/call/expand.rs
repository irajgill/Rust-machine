use super::parse::CallDef;
use quote::quote;

pub fn expand_call(def: CallDef) -> proc_macro2::TokenStream {
	let CallDef { pallet_struct, methods } = def;

	let fn_name = methods.iter().map(|method| &method.name).collect::<Vec<_>>();

	let args_name = methods
		.iter()
		.map(|method| method.args.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let args_type = methods
		.iter()
		.map(|method| method.args.iter().map(|(_, type_)| type_.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let dispatch_impl = quote! {
		//
		#[allow(non_camel_case_types)]
		pub enum Call<T: Config> {
			#(
				#fn_name { #( #args_name: #args_type),* },
			)*
		}

		impl<T: Config> crate::support::Dispatch for #pallet_struct<T> {
			type Caller = T::AccountId;
			type Call = Call<T>;

			fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
				match call {
					#(
						Call::#fn_name { #( #args_name ),* } => {
							self.#fn_name(
								caller,
								#( #args_name ),*
							)?;
						},
					)*
				}
				Ok(())
			}
		}
	};

	// Return the generated code.
	dispatch_impl.into()
}