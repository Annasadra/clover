pub use sc_executor::NativeExecutor;
use sc_executor::native_executor_instance;

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
native_executor_instance!(
	pub Executor,
	clover_runtime::api::dispatch,
	clover_runtime::native_version,
	frame_benchmarking::benchmarking::HostFunctions,
);