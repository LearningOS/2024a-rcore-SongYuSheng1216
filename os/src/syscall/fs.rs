//! File and filesystem-related syscalls


use super::SYSCALL_WRITE;
use super::super::task::get_current_task_info;

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");
    // (add)
    let current_mut_task_info = get_current_task_info();
    unsafe {
        (*current_mut_task_info).syscall_times[SYSCALL_WRITE] += 1;
    }

    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
