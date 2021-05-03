use std::{ffi::OsStr, path::Path};
use winapi::um::{minwinbase, synchapi, winnt};

fn is_roblox_open() -> bool {
	let roblox_process = Some(OsStr::new("RobloxPlayerBeta"));
	let mut is_open = false;

	process_list::for_each_process(|_id: u32, name: &Path| {
		if name.file_stem() == roblox_process {
			is_open = true;
		}
	})
	.unwrap();

	is_open
}

fn enable_multiple_roblox() -> winnt::HANDLE {
	println!("Holding ROBLOX_singletonMutex");

	let mut default_security = minwinbase::SECURITY_ATTRIBUTES::default();
	let mutex_name = "ROBLOX_singletonMutex"
		.encode_utf16()
		.collect::<Vec<u16>>()
		.as_mut_ptr();

	unsafe { synchapi::CreateMutexW(&mut default_security, 1, mutex_name) }
}

fn disable_multiple_roblox(handle: winnt::HANDLE) {
	unsafe {
		synchapi::ReleaseMutex(handle);
	}
}

pub fn start() -> Box<dyn FnOnce()> {
	if is_roblox_open() {
		println!("A Roblox process is open. Multiple Roblox will not be enabled.");

		return Box::new(|| {});
	}

	let rblx_handle = enable_multiple_roblox();
	Box::new(move || disable_multiple_roblox(rblx_handle))
}
