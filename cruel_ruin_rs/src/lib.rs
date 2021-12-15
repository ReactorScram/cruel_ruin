use std::{
	time::Duration,
};

pub struct CruelRuntime {
	rt: tokio::runtime::Runtime,
}

#[no_mangle]
pub extern "C" fn cruel_rt_init () -> Box <CruelRuntime> {
	let rt = tokio::runtime::Runtime::new ().unwrap ();
	
	Box::new (CruelRuntime {
		rt,
	})
}

#[no_mangle]
pub extern "C" fn cruel_rt_cleanup (rt: *mut CruelRuntime) {
	if rt.is_null () {
		return;
	}
	
	let rt = unsafe {
		Box::from_raw (rt)
	};
	
	std::mem::drop (rt);
}

#[no_mangle]
pub extern "C" fn cruel_rt_hello (rt: *const CruelRuntime) {
	if rt.is_null () {
		return;
	}
	
	let rt = unsafe {
		&*rt
	};
	
	rt.rt.block_on (async {
		use tokio::time::sleep;
		
		println! ("Hello 1 from Rust");
		sleep (Duration::from_secs (1)).await;
		println! ("Hello 2");
		sleep (Duration::from_secs (1)).await;
		println! ("Hello 3");
	});
}
