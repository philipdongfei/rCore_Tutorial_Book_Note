pub mod heap_allocator;


/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    /*
     * frame_allocator::init_frame_allocator();
     * KERNEL_SPACE.exclusive_access().activate();
     */
}


