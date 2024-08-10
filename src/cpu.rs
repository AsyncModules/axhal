//! CPU-related operations.

use core::panic;

#[percpu::def_percpu]
static CPU_ID: usize = 0;

#[percpu::def_percpu]
static IS_BSP: bool = false;

#[percpu::def_percpu]
static CURRENT_TASK_PTR: usize = 0;

/// Returns the ID of the current CPU.
#[inline]
pub fn this_cpu_id() -> usize {
    CPU_ID.read_current()
}

/// Returns whether the current CPU is the primary CPU (aka the bootstrap
/// processor or BSP)
#[inline]
pub fn this_cpu_is_bsp() -> bool {
    IS_BSP.read_current()
}

#[allow(dead_code)]
/// Initializes the primary CPU for its pointer.
pub fn init_primary(cpu_id: usize) {
    percpu::init(axconfig::SMP);
    percpu::set_local_thread_pointer(cpu_id);
    unsafe {
        CPU_ID.write_current_raw(cpu_id);
        IS_BSP.write_current_raw(true);
    }
}

#[allow(dead_code)]
/// Initializes the secondary CPU for its pointer.
pub fn init_secondary(cpu_id: usize) {
    percpu::set_local_thread_pointer(cpu_id);
    unsafe {
        CPU_ID.write_current_raw(cpu_id);
        IS_BSP.write_current_raw(false);
    }
}

extern "C" {
    fn boot_stack();
}

/// Get boot_stack_bottom
#[inline]
pub fn get_boot_stack_bottom() -> usize {
    let sp = crate::arch::get_sp();
    for i in 0..axconfig::SMP {
        let stack_bottom = boot_stack as usize + i * axconfig::TASK_STACK_SIZE;
        let stack_top = boot_stack as usize + (i + 1) * axconfig::TASK_STACK_SIZE;
        if stack_bottom <= sp && sp < stack_top {
            return stack_bottom;
        }
    }
    panic!("Failed to find boot_stack_bottom");
}