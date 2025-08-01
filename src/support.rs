pub struct Block<Header, Extrinsic> {
	///contains metadata about the block.
	pub header: Header,
	///represent the state transitions to be executed in this block.
	pub extrinsics: Vec<Extrinsic>,
}

pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

/// The Result type for our runtime.
pub type DispatchResult = Result<(), &'static str>;

/// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition
/// function call.
pub trait Dispatch {
	type Caller;
	type Call;

	
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}