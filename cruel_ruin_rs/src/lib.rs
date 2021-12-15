use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;

pub struct CruelRuntime {
	resolver: TokioAsyncResolver,
	rt: tokio::runtime::Runtime,
}

// There are probably some inefficiences in how this is implemented.
// It may leak, too.
pub struct CruelString (String);

#[repr (C)]
pub struct CruelStr {
	data: *const u8,
	len: usize,
}

#[no_mangle]
pub extern "C" fn cruel_rt_init () -> Box <CruelRuntime> {
	let rt = tokio::runtime::Runtime::new ().unwrap ();
	
	let resolver = rt.block_on (async {
		TokioAsyncResolver::tokio (
			ResolverConfig::default (),
			ResolverOpts::default ()
		)
	}).expect ("failed to connect resolver");
	
	Box::new (CruelRuntime {
		resolver,
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
pub extern "C" fn cruel_rt_resolve (
	rt: *const CruelRuntime,
	host: *const CruelStr,
) -> Box <CruelString> 
{
	// I promise regular Rust code is not this ugly.
	// This is because I'm writing the FFI stuff by hand, which is stupid,
	// but I know how to do it, and I don't feel like learning about cxxbind
	// or whatever the new stuff is.
	
	if 
		rt.is_null () ||
		host.is_null ()
	{
		return Box::new (CruelString ("".to_string ()));
	}
	
	let rt = unsafe {
		&*rt
	};
	
	let host = unsafe {
		&*host
	};
	
	if host.data.is_null () {
		return Box::new (CruelString ("".to_string ()));
	}
	
	let host = unsafe {
		std::slice::from_raw_parts (host.data, host.len)
	};
	
	// Normally Rust code would have error handling, too.
	let host = match std::str::from_utf8 (host) {
		Ok (x) => x,
		_ => return Box::new (CruelString ("".to_string ())),
	};
	
	let response = rt.rt.block_on (async {
		rt.resolver.ipv4_lookup (host).await.unwrap ()
	});
	
	let ip_str = response.iter ().next ()
	.map (|x| format! ("{:?}", x))
	.unwrap_or_else (|| "".to_string ());
	
	println! ("Rust resolved the IP as {}", ip_str);
	
	Box::new (CruelString (ip_str))
}

#[no_mangle]
pub extern "C" fn cruel_string_ptr (s: *const CruelString) -> *const u8 {
	if s.is_null () {
		panic! ("cruel_string_ptr got nullptr");
	}
	
	let s = unsafe {
		&*s
	};
	
	s.0.as_ptr ()
}

#[no_mangle]
pub extern "C" fn cruel_string_len (s: *const CruelString) -> usize {
	if s.is_null () {
		panic! ("cruel_string_len got nullptr");
	}
	
	let s = unsafe {
		&*s
	};
	
	s.0.len ()
}

#[no_mangle]
pub extern "C" fn cruel_string_free (s: *mut CruelString) {
	if s.is_null () {
		return;
	}
	
	let s = unsafe {
		Box::from_raw (s)
	};
	
	std::mem::drop (s);
}
