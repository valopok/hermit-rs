use crate::syscalls::*;

pub fn run_tests() {
	println!("Start running tests.");
	number_of_namespaces();
	max_buffer_size();
	max_number_of_queue_entries();
	size_of_namespaces();
	io_queue_pairs();
	println!("Tests ran successfully.");
}

fn number_of_namespaces() {
	let result = get_number_of_namespaces();
	assert!(
		result.is_ok(),
		"Could not get number of namespaces. Please verify that an NVMe device is available."
	);
}

fn max_buffer_size() {
	let result = get_max_buffer_size();
	assert!(
		result.is_ok(),
		"Could not get max buffer size. Please verify that an NVMe device is available."
	);
	let max_number_of_queue_entries = result.unwrap();
	assert!(max_number_of_queue_entries >= 2);
}

fn max_number_of_queue_entries() {
	let result = get_max_number_of_queue_entries();
	assert!(
		result.is_ok(),
		"Could not get number of max queue entries. Please verify that an NVMe device is available."
	);
	let max_number_of_queue_entries = result.unwrap();
	assert!(max_number_of_queue_entries >= 2);
}

fn size_of_namespaces() {
	let result = get_number_of_namespaces();
	assert!(
		result.is_ok(),
		"Could not get number of namespaces. Please verify that an NVMe device is available."
	);

	let number_of_namespaces = result.unwrap();
	(0..number_of_namespaces).into_iter().for_each(|i| {
		let size_of_namespace = get_size_of_namespace(i);
		assert!(size_of_namespace.is_ok());
	});

	let invalid = get_size_of_namespace(number_of_namespaces + 1);
	assert!(invalid.is_err());

	let invalid = get_size_of_namespace(number_of_namespaces + 42_000_000);
	assert!(invalid.is_err());
}

fn io_queue_pairs() {
	let number_of_namespaces: usize = get_number_of_namespaces().unwrap();
	let max_entries: u16 = get_max_number_of_queue_entries().unwrap();
	(0..number_of_namespaces).into_iter().for_each(|i| {
		let result = create_io_queue_pair(i, 0);
		assert!(result.is_err());
		let result = create_io_queue_pair(i, 1);
		assert!(result.is_err());

		let result = create_io_queue_pair(i, 2);
		assert!(result.is_ok());
		let result = delete_io_queue_pair(result.unwrap());
		assert!(result.is_ok());

		let result = create_io_queue_pair(i, (max_entries / 2).min(2));
		assert!(result.is_ok());
		let result = delete_io_queue_pair(result.unwrap());
		assert!(result.is_ok());

		let result = create_io_queue_pair(i, max_entries);
		assert!(result.is_ok());
		let result = delete_io_queue_pair(result.unwrap());
		assert!(result.is_ok());

		if max_entries < u16::MAX {
			let result = create_io_queue_pair(i, max_entries + 1);
			assert!(result.is_err());
			let result = create_io_queue_pair(i, u16::MAX);
			assert!(result.is_err());
		}

		let max_number_of_queue_pairs = 2;
		let mut queue_pairs = Vec::new();

		(0..max_number_of_queue_pairs).for_each(|_| {
			let result = create_io_queue_pair(i, 2);
			assert!(result.is_ok());
			queue_pairs.push(result.unwrap())
		});

        let result = create_io_queue_pair(i, 2);
        assert!(result.is_err());

		queue_pairs.into_iter().for_each(|queue_pair| {
			let result = delete_io_queue_pair(queue_pair);
			assert!(result.is_ok());
		});
	});
}
