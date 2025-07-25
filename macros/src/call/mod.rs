pub mod expand;
pub mod parse;

pub fn call(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let mut finished = item.clone();
	let item_mod = syn::parse_macro_input!(item as syn::Item);

	// First we parse the call functions implemented for the pallet...
	let generated: proc_macro::TokenStream = match parse::CallDef::try_from(item_mod.clone()) {
		// ..then we generate our new code.
		Ok(def) => expand::expand_call(def).into(),
		Err(e) => e.to_compile_error().into(),
	};

	// Add our generated code to the end, and return the final result.
	finished.extend(generated);
	return finished;
}