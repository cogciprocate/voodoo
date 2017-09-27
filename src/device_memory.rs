
use std::sync::Arc;
use std::ptr;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;
use std::marker::PhantomData;
use libc::c_void;
use vks;
use ::{util, VooResult, Device};


/// A slice of mapped memory.
///
/// Use `DeviceMemory::unmap` to unmap.
pub struct MemoryMapping<'m, T> {
    ptr: *mut T,
    len: usize,
    mem_handle: vks::VkDeviceMemory,
    _p: PhantomData<&'m ()>,
}

impl<'m, T> MemoryMapping<'m, T> {
    /// Returns a new `MemoryMapping`
    fn new(ptr: *mut T, len: usize, mem_handle: vks::VkDeviceMemory) -> MemoryMapping<'m, T> {
        MemoryMapping {ptr, len, mem_handle, _p: PhantomData}
    }
}

impl<'m, T> Deref for MemoryMapping<'m, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'m, T> DerefMut for MemoryMapping<'m, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}



#[derive(Debug)]
struct Inner {
    handle: vks::VkDeviceMemory,
    device: Device,
    allocation_size: u64,
    memory_type_index: u32,
}

#[derive(Debug, Clone)]
pub struct DeviceMemory {
    inner: Arc<Inner>,
}

impl DeviceMemory {
    /// Returns a new `DeviceMemoryBuilder`.
    pub fn builder() -> DeviceMemoryBuilder {
        DeviceMemoryBuilder::new()
    }

    pub fn new(device: Device, allocation_size: u64, memory_type_index: u32) -> VooResult<DeviceMemory> {
        DeviceMemoryBuilder::new()
            .allocation_size(allocation_size)
            .memory_type_index(memory_type_index)
            .build(device)
    }

    /// Maps a region of this memory object to a pointer.
    ///
    /// Use `::unmap_ptr` to unmap this memory.
    ///
    /// `flags` is reserved for future use.
    pub unsafe fn map_to_ptr<T>(&self, offset_bytes: u64, size_bytes: u64, flags: vks::VkMemoryMapFlags)
            -> VooResult<*mut T> {
        let mut data = ptr::null_mut();
        ::check(self.inner.device.proc_addr_loader().vkMapMemory(self.inner.device.handle(),
            self.inner.handle, offset_bytes, size_bytes, flags, &mut data));
        Ok(data as *mut T)
    }

    /// Unmaps memory.
    ///
    /// Do not use this unless memory was mapped using `::map_to_ptr`.
    ///
    /// Use `::unmap` to unmap memory mapped by `::map`.
    pub unsafe fn unmap_ptr(&self) {
        self.inner.device.proc_addr_loader().core.vkUnmapMemory(self.inner.device.handle(),
            self.inner.handle);
    }

    /// Maps a region of memory and returns a mutable reference to it.
    ///
    /// `flags` is reserved for future use.
    pub fn map<'m, T>(&'m self, offset_bytes: u64, size_bytes: u64, flags: vks::VkMemoryMapFlags)
            -> VooResult<MemoryMapping<'m, T>> {
        let ptr = unsafe { self.map_to_ptr(offset_bytes, size_bytes, flags)? };
        let len = size_bytes as usize / mem::size_of::<T>();
        Ok(MemoryMapping::new(ptr, len, self.inner.handle))
    }

    /// Unmaps memory.
    pub fn unmap<'m, T>(&self, mapping: MemoryMapping<'m, T>) {
        assert!(mapping.mem_handle == self.inner.handle,
            "cannot unmap memory: memory mapping is from a different memory object");
        unsafe { self.unmap_ptr() }
    }

    /// Returns a handle.
    pub fn handle(&self) -> vks::VkDeviceMemory {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkFreeMemory(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `DeviceMemory`.
//
// typedef struct VkMemoryAllocateInfo {
//     VkStructureType    sType;
//     const void*        pNext;
//     VkDeviceSize       allocationSize;
//     uint32_t           memoryTypeIndex;
// } VkMemoryAllocateInfo;
//
#[derive(Debug, Clone)]
pub struct DeviceMemoryBuilder {
    allocate_info: vks::VkMemoryAllocateInfo,
}

impl DeviceMemoryBuilder {
    /// Returns a new render pass builder.
    pub fn new() -> DeviceMemoryBuilder {
        DeviceMemoryBuilder {
            allocate_info: vks::VkMemoryAllocateInfo::default(),
        }
    }

    /// Specifies the size of the allocation in bytes
    pub fn allocation_size<'s>(&'s mut self, allocation_size: vks::VkDeviceSize)
            -> &'s mut DeviceMemoryBuilder {
        self.allocate_info.allocationSize = allocation_size;
        self
    }

    /// Specifies the memory type index, which selects the properties of the
    /// memory to be allocated, as well as the heap the memory will come from.
    pub fn memory_type_index<'s>(&'s mut self, memory_type_index: u32)
            -> &'s mut DeviceMemoryBuilder {
        self.allocate_info.memoryTypeIndex = memory_type_index;
        self
    }

    /// Creates and returns a new `DeviceMemory`
    pub fn build(&self, device: Device) -> VooResult<DeviceMemory> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkAllocateMemory(device.handle(),
                &self.allocate_info, ptr::null(), &mut handle));
        }

        Ok(DeviceMemory {
            inner: Arc::new(Inner {
                handle,
                device,
                allocation_size: self.allocate_info.allocationSize,
                memory_type_index: self.allocate_info.memoryTypeIndex,
            })
        })
    }
}