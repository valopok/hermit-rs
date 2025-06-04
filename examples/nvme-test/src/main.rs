#[cfg(target_os = "hermit")]
use hermit as _;

mod syscalls;
mod tests;
use syscalls::*;
use tests::run_tests;

fn main() {
	println!("Hello, NVMe!");
	match example() {
		Err(error) => eprintln!("{error:?}"),
		Ok(()) => println!("Success!"),
	}
	run_tests();
}

fn example() -> Result<(), SysNvmeError> {
	let number_of_namespaces = get_number_of_namespaces()?;
	println!("Number of namespaces: {number_of_namespaces}.");

	let max_buffer_size = get_max_buffer_size()?;
	println!("Max buffer size: {max_buffer_size:#X}.");

	let max_number_of_queue_entries = get_max_number_of_queue_entries()?;
	println!("Max number of queue entries: {max_number_of_queue_entries}.");

	let namespace_index: usize = 0;
	let size_of_namespace = get_size_of_namespace(namespace_index)?;
	println!("Size of namespace with index {namespace_index}: {size_of_namespace:#X}.");

	let number_of_entries = 2;
	let io_queue_pair_id = create_io_queue_pair(namespace_index, number_of_entries)?;
	println!("Created IO queue pair with ID {io_queue_pair_id:?} and {number_of_entries} queue entries for namespace {namespace_index}.");

	let buffer_1: [u8; 16] = [
		0, 1, 2, 3, 4, 5, 6, 7, 8, 9, b'A', b'B', b'C', b'D', b'E', b'F',
	];
	let logical_block_address = 0;
	write_to_io_queue_pair(&io_queue_pair_id, &buffer_1, logical_block_address)?;
	println!("Wrote to IO queue pair with ID {io_queue_pair_id:?}.");

	let mut buffer_2: [u8; 16] = [0; 16];
	read_from_io_queue_pair(&io_queue_pair_id, &mut buffer_2, logical_block_address)?;
	println!("Read from IO queue pair with ID {io_queue_pair_id:?}.");

	println!("buffer_1: {buffer_1:?}");
	println!("buffer_2: {buffer_2:?}");

	delete_io_queue_pair(io_queue_pair_id)?;
	println!("Deleted IO queue pair.");
	Ok(())
}
