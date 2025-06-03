#[cfg(target_os = "hermit")]
use hermit as _;

fn main() {
	println!("Hello, NVMe!");
	let mut number_of_namespaces: usize = 0;
	let exit_code = unsafe { sys_nvme_get_number_of_namespaces(&mut number_of_namespaces) };
	if exit_code != 0 {
		eprintln!("sys_nvme_get_number_of_namespaces failed with exit code {exit_code}.");
	} else {
		println!("Number of namespaces: {number_of_namespaces}.");
	}

	if exit_code != 0 || number_of_namespaces == 0 {
		return;
	}

	let namespace_index: usize = 0;
	let mut size_of_namespace: u64 = 0;
	let exit_code =
		unsafe { sys_nvme_get_size_of_namespace(namespace_index, &mut size_of_namespace) };
	if exit_code != 0 {
		eprintln!("sys_nvme_get_size_of_namespace failed with exit code {exit_code}.");
	} else {
		println!("Size of namespace with index {namespace_index}: {size_of_namespace:#X}.");
	}

	let number_of_entries = 2;
	let mut io_queue_pair_id: usize = 0;
	let exit_code = unsafe {
		sys_nvme_create_io_queue_pair(namespace_index, number_of_entries, &mut io_queue_pair_id)
	};
	if exit_code != 0 {
		eprintln!("sys_nvme_create_io_queue_pair failed with exit code {exit_code}.");
	} else {
		println!("Created IO queue pair with ID {io_queue_pair_id} and {number_of_entries} queue entries for namespace {namespace_index}.");
	}

	let buffer_1: [u8; 16] = [
		0, 1, 2, 3, 4, 5, 6, 7, 8, 9, b'A', b'B', b'C', b'D', b'E', b'F',
	];
	let logical_block_address = 0;
	let exit_code = unsafe {
		sys_nvme_write_to_io_queue_pair(
			io_queue_pair_id,
			buffer_1.as_ptr(),
			buffer_1.len(),
			logical_block_address,
		)
	};
	if exit_code != 0 {
		eprintln!("sys_nvme_write_to_io_queue_pair failed with exit code {exit_code}.");
	} else {
		println!("Wrote to IO queue pair with ID {io_queue_pair_id}.");
	}

	let mut buffer_2: [u8; 16] = [0;16];
	let exit_code = unsafe {
		sys_nvme_read_from_io_queue_pair(
			io_queue_pair_id,
			buffer_2.as_mut_ptr(),
			buffer_2.len(),
			logical_block_address,
		)
	};
	if exit_code != 0 {
		eprintln!("sys_nvme_read_from_io_queue_pair failed with exit code {exit_code}.");
	} else {
		println!("Read from IO queue pair with ID {io_queue_pair_id}.");
	}

    println!("buffer_1: {buffer_1:?}");
    println!("buffer_2: {buffer_2:?}");

	let exit_code = unsafe { sys_nvme_delete_io_queue_pair(io_queue_pair_id) };
	if exit_code != 0 {
		eprintln!("sys_nvme_delete_io_queue_pair failed with exit code {exit_code}.");
	} else {
		println!("Deleted IO queue pair with ID {io_queue_pair_id}.");
	}
}

unsafe extern "C" {
	#[link_name = "sys_nvme_get_number_of_namespaces"]
	pub fn sys_nvme_get_number_of_namespaces(result: *mut usize) -> usize;
}

unsafe extern "C" {
	#[link_name = "sys_nvme_get_size_of_namespace"]
	pub fn sys_nvme_get_size_of_namespace(namespace_index: usize, result: *mut u64) -> usize;
}

unsafe extern "C" {
	#[link_name = "sys_nvme_create_io_queue_pair"]
	pub fn sys_nvme_create_io_queue_pair(
		namespace_index: usize,
		number_of_entries: usize,
		resulting_io_queue_pair_id: *mut usize,
	) -> usize;
}

unsafe extern "C" {
	#[link_name = "sys_nvme_delete_io_queue_pair"]
	pub fn sys_nvme_delete_io_queue_pair(io_queue_pair_id: usize) -> usize;
}

unsafe extern "C" {
	#[link_name = "sys_nvme_read_from_io_queue_pair"]
	pub fn sys_nvme_read_from_io_queue_pair(
		io_queue_pair_id: usize,
		buffer_pointer: *mut u8,
		buffer_size: usize,
		logical_block_address: u64,
	) -> usize;
}

unsafe extern "C" {
	#[link_name = "sys_nvme_write_to_io_queue_pair"]
	pub fn sys_nvme_write_to_io_queue_pair(
		io_queue_pair_id: usize,
		buffer_pointer: *const u8,
		buffer_size: usize,
		logical_block_address: u64,
	) -> usize;
}
