use super::utils::errors::ExecutionError;
use std::ptr;

/// Represents the memory of the EVM.
#[derive(Debug)]
pub struct Memory {
    pub heap: Vec<u8>,
}

impl Memory {
    /// Creates a new instance of `Memory`.
    ///
    /// # Arguments
    ///
    /// * `data` - An optional vector of bytes to initialize the memory with.
    ///
    /// # Returns
    ///
    /// A new instance of `Memory`.
    pub fn new(data: Option<Vec<u8>>) -> Self {
        Self {
            heap: if data.is_some() {
                data.unwrap()
            } else {
                vec![0; 0]
            },
        }
    }

    /// Extends the memory by the specified size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size to extend the memory by.
    pub fn extend(&mut self, size: usize) {
        self.heap.extend(vec![0; size]);
    }

    /// Reads bytes from memory starting at the specified address.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to start reading from.
    /// * `size` - The number of bytes to read.
    ///
    /// # Returns
    ///
    /// A `Result` containing the bytes read or an `ExecutionError` if the read operation failed.
    pub unsafe fn read(&mut self, offset: usize, size: usize) -> Result<Vec<u8>, ExecutionError> {
        // Increase memory heap to the nearest multiple of 32 if address is out of bounds
        if offset + size > self.heap.len() {
            // Calculate the nearest multiple of 32
            let nearest_multiple = if offset % 32 == 0 {
                offset + 32
            } else {
                (offset + 32) + (32 - (offset + 32) % 32)
            };

            // Extend memory heap
            self.extend(nearest_multiple - self.heap.len());
        }

        let ptr = self.heap.as_ptr().add(offset);
        let mut data = vec![0; size];
        ptr::copy(ptr, data.as_mut_ptr(), size);

        Ok(data)
    }

    /// Writes bytes to memory starting at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to start writing to.
    /// * `data` - The bytes to write.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the write operation was successful or an `ExecutionError` if it failed.
    pub unsafe fn write(&mut self, offset: usize, data: Vec<u8>) -> Result<(), ExecutionError> {
        // check if memory should be extended
        if offset + data.len() > self.heap.len() {
            // Calculate the nearest multiple of 32
            let nearest_multiple = if offset % 32 == 0 {
                offset + data.len() + 32
            } else {
                (offset + data.len() + 32) + (32 - (offset + data.len() + 32) % 32)
            };

            // Extend memory heap
            self.extend(nearest_multiple - self.heap.len());
        }

        let ptr = self.heap.as_mut_ptr().add(offset);
        ptr::copy(data.as_ptr(), ptr, data.len());

        Ok(())
    }

    /// Reads 32 bytes from memory starting at the specified address.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to start reading from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the 32 bytes read or an `ExecutionError` if the read operation failed.
    pub unsafe fn mload(&mut self, offset: usize) -> Result<[u8; 32], ExecutionError> {
        // Increase memory heap to the nearest multiple of 32 if address is out of bounds
        if offset + 32 > self.heap.len() {
            // Calculate the nearest multiple of 32
            let nearest_multiple = if offset % 32 == 0 {
                offset + 32
            } else {
                (offset + 32) + (32 - (offset + 32) % 32)
            };

            // Extend memory heap
            self.extend(nearest_multiple - self.heap.len());
        }

        let ptr = self.heap.as_ptr().add(offset);
        Ok(ptr::read(ptr as *const [u8; 32]))
    }

    /// Writes 32 bytes to memory starting at the specified address.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to start writing to.
    /// * `data` - The 32 bytes to write.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the write operation was successful or an `ExecutionError` if it failed.
    pub unsafe fn mstore(&mut self, offset: usize, data: [u8; 32]) -> Result<(), ExecutionError> {
        // Check if memory should be extended
        if offset + 32 > self.heap.len() {
            self.extend(offset + 32 - self.heap.len());
        }

        let ptr = self.heap.as_mut_ptr().add(offset);
        ptr::write(ptr as *mut [u8; 32], data);

        Ok(())
    }

    /// Gets the size of the memory heap.
    ///
    /// # Returns
    ///
    /// The size of the memory heap.
    pub fn msize(&self) -> usize {
        self.heap.len()
    }
}

impl Clone for Memory {
    fn clone(&self) -> Self {
        Memory {
            heap: self.heap.clone(),
        }
    }
}
