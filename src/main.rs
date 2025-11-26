#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::boot;
use uefi::println;
use uefi::print;
use uefi::mem::memory_map::MemoryMap;
use uefi::mem::memory_map::MemoryType;
use uefi::proto::media::file::{FileMode, FileAttribute, FileType};
use uefi::proto::media::file::File;
use uefi::proto::media::file::FileHandle;
use uefi::CString16;
use uefi::Char16;
use uefi::runtime::{self, ResetType};

#[entry]
fn start() -> Status {
	match main() {
		Ok(()) => Status::SUCCESS,
		Err(err) => err.status(),
	}
}

// Convert a number to CStr16
fn number_to_cstr16(num: u64) -> uefi::Result<CString16> {
	// Format the number as a string
	let mut s = CString16::new();

	// Convert number to string manually (since we don't have std::format!)
	if num == 0 {
		s.push(Char16::try_from('0').unwrap());
	} else {
		let mut temp_num = num;
		let mut digits = [0u16; 20]; // Enough for u64 max
		let mut digit_count = 0;

		// Extract digits in reverse order
		while temp_num > 0 {
			digits[digit_count] = (temp_num % 10) as u16 + '0' as u16;
			temp_num /= 10;
			digit_count += 1;
		}

		// Add digits in correct order
		for i in (0..digit_count).rev() {
			s.push(Char16::try_from(digits[i]).unwrap());
		}
	}

	Ok(s)
}

fn copy_memory_via_slice(src_addr: u64, length: usize, buffer: &mut [u8]) -> Result<(), &'static str> {
	if buffer.len() < length {
		return Err("Buffer too small");
	}

	let src_slice = unsafe {
		core::slice::from_raw_parts(src_addr as *const u8, length)
	};

	buffer[..length].copy_from_slice(src_slice);

	Ok(())
}   

fn get_largest_usable_memory_block() -> uefi::Result<Option<(u64, usize)>> {
	let memory_map = boot::memory_map(MemoryType::LOADER_DATA)?;

	let mut largest_block = None;
	let mut largest_size = 0u64;

	for descriptor in memory_map.entries() {
		let is_usable = matches!(descriptor.ty, MemoryType::CONVENTIONAL);

		if is_usable {
			let size_bytes = descriptor.page_count * 4096;
			if size_bytes > largest_size {
				largest_size = size_bytes;
				largest_block = Some((descriptor.phys_start, descriptor.page_count as usize));
			}
		}
	}

	Ok(largest_block)
}

fn main() -> uefi::Result {
	let block = get_largest_usable_memory_block().unwrap().unwrap();
	
	println!("Largest contigous block is {} MB", 4096*block.1 / (1024*1024) );
	
	let mut fs_protocol = boot::get_image_file_system(boot::image_handle())?;
	
	let mut root = fs_protocol.open_volume()?;

	let mut file_handles: [Option<FileHandle>; 16] = [const { None }; 16];
	let mut gb_mul_four = block.1 / (1024*1024);
	if ( block.1 % (1024*1024) ) > 0 { gb_mul_four += 1; }
	println!("Splitting dump across {} file(s)...", gb_mul_four);
	for i in 0..gb_mul_four {
		let mut it = number_to_cstr16(i as u64).unwrap();
		it.push_str(cstr16!(".bin"));
		// Open a file
		let file_handle = root.open(
			&it,
			FileMode::CreateReadWrite,
			FileAttribute::empty()
		)?;
		file_handles[i as usize] = Some(file_handle);
	}

	let mut buffer: [u8; 4096] = [0; 4096];
	for (index, file_handle) in file_handles.into_iter().enumerate() {
		if !file_handle.is_none() {
			if let FileType::Regular(mut file) = file_handle.unwrap().into_type()? {
				let mut end = (index+1)*block.1/gb_mul_four;
				if index == gb_mul_four-1 { end=block.1; }
				let start = index*block.1/gb_mul_four;
				for i in start..end {
					print!("Dumping Memory: {:3.2} %\r", 100.00 * i as f32 / block.1 as f32 );
					let _ = copy_memory_via_slice( block.0 + (i as u64*4096), 4096, &mut buffer);
					file.write(&buffer).discard_errdata()?;
					if i % 8 == 0 { file.flush()?; }
				}
				file.flush()?;
			}
		}
	}
	println!("Shutting down system...");
	runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

