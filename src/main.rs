extern crate nix;
extern crate libc;
extern crate core;

use core::ffi;
use nix::sys::wait::*;
use nix::sys::signal::Signal; 
use nix::sys::ptrace;
use std::mem;

fn main() {
    let pid = nix::unistd::Pid::from_raw(28718);

    let attach = ptrace::attach(pid);
    println!("Attach {:?}", attach);
    let m: *mut libc::c_void;

    unsafe{
        let m = libc::malloc(mem::size_of::<i64>()) as *mut libc::c_void;
        let addr: ptrace::AddressType = m as *mut ffi::c_void;
    }


    for x in 0..10{
        match waitpid(pid, Some(<WaitPidFlag>::WSTOPPED)) {
            Ok(WaitStatus::Stopped(pid, Signal::SIGSTOP)) => {
            let syscall = nix::sys::ptrace::syscall(pid);
            println!("Syscall {:?}", syscall);

            let info = ptrace::getsiginfo(pid);
            let r = ptrace::cont(pid, None);
            println!("Cont {:?}", r)
            }
            _ => {},
        }
    }
}
