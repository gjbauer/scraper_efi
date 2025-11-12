#![no_main]
#![no_std]

use core::time::Duration;
use uefi::prelude::*;
use uefi::boot;
use uefi::println;
use uefi::mem::memory_map::MemoryMap;
use uefi::mem::memory_map::MemoryType;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{FileMode, FileAttribute, FileType};
use uefi::proto::media::file::File;

#[entry]
fn start() -> Status {
	match main() {
		Ok(()) => Status::SUCCESS,
		Err(err) => err.status(),
	}
}

fn main() -> uefi::Result {
	let memory_map = boot::memory_map(MemoryType::LOADER_DATA)?;  
	
	let last_entry = memory_map.entries().last().unwrap();
	println!("Memory region: {:?}", last_entry);
	println!("  Physical Start: 0x{:x}", last_entry.phys_start);   
	println!("  Number of Pages: {}", last_entry.page_count);
	
	let mut fs_protocol = boot::get_image_file_system(boot::image_handle())?;
	
	let mut root = fs_protocol.open_volume()?;

	// Open a file
	let file_handle = root.open(
		cstr16!("config.txt"),
		FileMode::Read,
		FileAttribute::empty()
	)?;

	// Use the file
	if let FileType::Regular(mut file) = file_handle.into_type()? {
		let mut buffer = [0u8; 4*1024*1024*1024];
		let bytes_read = file.read(&mut buffer)?;
		// Process file contents...
	}
	
	Ok(())
}

