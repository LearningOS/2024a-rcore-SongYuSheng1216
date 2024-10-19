//! Process management syscalls
#[allow(unused)]
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_current_task_info},
    timer::{get_time_us, get_time_ms},
    syscall::{SYSCALL_YIELD, SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TASK_INFO},
};

#[repr(C)]
#[derive(Debug)]
#[allow(missing_docs)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Clone,Copy,Debug)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
    /// the start time of running
    pub start_time: usize,
}

// 各个数据的更改呢？
// 应该是在各个函数调用的时候改变其调用的syscall_times和任务的time
// 放在mod模块下的syscall函数中

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    // (add)
    let current_mut_task_info = get_current_task_info();
    unsafe {
        (*current_mut_task_info).syscall_times[SYSCALL_EXIT] += 1;
    }

    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    // (add)
    let current_mut_task_info = get_current_task_info();
    unsafe {
        (*current_mut_task_info).syscall_times[SYSCALL_YIELD] += 1;
    }

    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time"); // 这个到时候去查一下，可能是用来debug的框架代码
    // (add)
    let current_mut_task_info = get_current_task_info();
    unsafe {
        (*current_mut_task_info).syscall_times[SYSCALL_GET_TIME] += 1;
    }
    
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}


// 查看task相关的代码，
// 1.理清楚task的数据结构
// 2.理清楚task的初始化
// 3.理清楚task在os中的调用
// 4.理清楚user的应用程序，是如何一步步调用到我们这里定义的sys_task_info的？

// 需要在在各函数调用的时候增加syscall_times
// syscall_times是属于哪个TaskInfo，其信息应该来源于内核栈
// TaskInfo在内核的某个任务结构体内，一同保存

// 将内核态中的此刻任务的TaskInfo数据，拷贝在_ti指向的地址上

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    //println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    trace!("kernel: sys_task_info");
    // (add)
    let current_mut_task_info = get_current_task_info();
    //println!("aaaaaaaaaaaaaaaaaaaaaa : {:?}", unsafe{(*current_mut_task_info).status});
    unsafe {
        // 调用自身也要计数
        (*current_mut_task_info).syscall_times[SYSCALL_TASK_INFO] += 1;
        // 除了task切换时更新一次运行时间，这里调用了info时也更新一次
        (*current_mut_task_info).time = get_time_ms() - (*current_mut_task_info).start_time;
        *_ti = *current_mut_task_info;
    }
    0
}
