pub fn get_number_of_namespaces() -> Result<usize, SysNvmeError> {
	let mut number_of_namespaces: usize = 0;
	let exit_code = unsafe { sys_nvme_get_number_of_namespaces(&mut number_of_namespaces) };
	match exit_code {
		0 => Ok(number_of_namespaces),
		n => Err(SysNvmeError::from(n)),
	}
}

pub fn get_max_buffer_size() -> Result<usize, SysNvmeError> {
	let mut max_buffer_size: usize = 0;
	let exit_code =
		unsafe { sys_nvme_get_max_buffer_size(&mut max_buffer_size) };
	match exit_code {
		0 => Ok(max_buffer_size),
		n => Err(SysNvmeError::from(n)),
	}
}

pub fn get_max_number_of_queue_entries() -> Result<u16, SysNvmeError> {
	let mut max_number_of_queue_entries: u16 = 0;
	let exit_code =
		unsafe { sys_nvme_get_max_number_of_queue_entries(&mut max_number_of_queue_entries) };
	match exit_code {
		0 => Ok(max_number_of_queue_entries),
		n => Err(SysNvmeError::from(n)),
	}
}

pub fn get_size_of_namespace(namespace_index: usize) -> Result<u64, SysNvmeError> {
	let mut result: u64 = 0;
	let exit_code = unsafe { sys_nvme_get_size_of_namespace(namespace_index, &mut result) };
	match exit_code {
		0 => Ok(result),
		n => Err(SysNvmeError::from(n)),
	}
}

pub fn create_io_queue_pair(
	namespace_index: usize,
	number_of_entries: u16,
) -> Result<IoQueuePairId, SysNvmeError> {
	let mut result: usize = 0;
	let exit_code =
		unsafe { sys_nvme_create_io_queue_pair(namespace_index, number_of_entries, &mut result) };
	match exit_code {
		0 => Ok(IoQueuePairId(result)),
		n => Err(SysNvmeError::from(n)),
	}
}

pub fn delete_io_queue_pair(io_queue_pair_id: IoQueuePairId) -> Result<(), SysNvmeError> {
	let exit_code = unsafe { sys_nvme_delete_io_queue_pair(io_queue_pair_id.0) };
	match exit_code {
		0 => Ok(()),
		n => Err(SysNvmeError::from(n)),
	}
}

pub fn read_from_io_queue_pair(
	io_queue_pair_id: &IoQueuePairId,
	buffer: &mut [u8],
	logical_block_address: u64,
) -> Result<(), SysNvmeError> {
	let exit_code = unsafe {
		sys_nvme_read_from_io_queue_pair(
			io_queue_pair_id.0,
			buffer.as_mut_ptr(),
			buffer.len(),
			logical_block_address,
		)
	};
	match exit_code {
		0 => Ok(()),
		n => Err(SysNvmeError::from(n)),
	}
}

pub fn write_to_io_queue_pair(
	io_queue_pair_id: &IoQueuePairId,
	buffer: &[u8],
	logical_block_address: u64,
) -> Result<(), SysNvmeError> {
	let exit_code = unsafe {
		sys_nvme_write_to_io_queue_pair(
			io_queue_pair_id.0,
			buffer.as_ptr(),
			buffer.len(),
			logical_block_address,
		)
	};
	match exit_code {
		0 => Ok(()),
		n => Err(SysNvmeError::from(n)),
	}
}

unsafe extern "C" {
	#[link_name = "sys_nvme_get_number_of_namespaces"]
	fn sys_nvme_get_number_of_namespaces(result: *mut usize) -> usize;

	#[link_name = "sys_nvme_get_max_buffer_size"]
	fn sys_nvme_get_max_buffer_size(result: *mut usize) -> usize;

	#[link_name = "sys_nvme_get_max_number_of_queue_entries"]
	fn sys_nvme_get_max_number_of_queue_entries(result: *mut u16) -> usize;

	#[link_name = "sys_nvme_get_size_of_namespace"]
	fn sys_nvme_get_size_of_namespace(namespace_index: usize, result: *mut u64) -> usize;

	#[link_name = "sys_nvme_create_io_queue_pair"]
	fn sys_nvme_create_io_queue_pair(
		namespace_index: usize,
		number_of_entries: u16,
		resulting_io_queue_pair_id: *mut usize,
	) -> usize;

	#[link_name = "sys_nvme_delete_io_queue_pair"]
	fn sys_nvme_delete_io_queue_pair(io_queue_pair_id: usize) -> usize;

	#[link_name = "sys_nvme_read_from_io_queue_pair"]
	fn sys_nvme_read_from_io_queue_pair(
		io_queue_pair_id: usize,
		buffer_pointer: *mut u8,
		buffer_size: usize,
		logical_block_address: u64,
	) -> usize;

	#[link_name = "sys_nvme_write_to_io_queue_pair"]
	fn sys_nvme_write_to_io_queue_pair(
		io_queue_pair_id: usize,
		buffer_pointer: *const u8,
		buffer_size: usize,
		logical_block_address: u64,
	) -> usize;
}

#[derive(Debug)]
pub struct IoQueuePairId(usize);

#[derive(Debug, Clone, Copy)]
pub enum SysNvmeError {
	UnknownError = 0,
	ZeroPointerParameter = 1,
	DeviceDoesNotExist = 2,
	CouldNotIdentifyNamespaces = 3,
	NamespaceDoesNotExist = 4,
	CouldNotCreateIoQueuePair = 5,
	CouldNotDeleteIoQueuePair = 6,
	CouldNotFindIoQueuePair = 7,
	BufferTooBig = 8,
	CouldNotAllocateMemory = 9,
	CouldNotReadFromIoQueuePair = 10,
	CouldNotWriteToIoQueuePair = 11,
}

impl From<usize> for SysNvmeError {
	fn from(value: usize) -> Self {
		match value {
			1 => SysNvmeError::ZeroPointerParameter,
			2 => SysNvmeError::DeviceDoesNotExist,
			3 => SysNvmeError::CouldNotIdentifyNamespaces,
			4 => SysNvmeError::NamespaceDoesNotExist,
			5 => SysNvmeError::CouldNotCreateIoQueuePair,
			6 => SysNvmeError::CouldNotDeleteIoQueuePair,
			7 => SysNvmeError::CouldNotFindIoQueuePair,
			8 => SysNvmeError::BufferTooBig,
			9 => SysNvmeError::CouldNotAllocateMemory,
			10 => SysNvmeError::CouldNotReadFromIoQueuePair,
			11 => SysNvmeError::CouldNotWriteToIoQueuePair,
			_ => SysNvmeError::UnknownError,
		}
	}
}
