extern crate nix;
extern crate libc;
extern crate core;

use core::ffi;
use nix::sys::wait::*;
use nix::sys::signal::Signal; 
use nix::sys::ptrace;
use std::mem;

//fn attach(pid: i64) -> Result<



fn main() {
    let pid = nix::unistd::Pid::from_raw(1579);

    let attach = ptrace::attach(pid);
    println!("Attach {:?}", attach);

    match waitpid(pid, Some(<WaitPidFlag>::WSTOPPED)){
        Ok(res) =>{
            println!("First stop {:?}", res);
            let scall = ptrace::syscall(pid);
            println!("First scall {:?}", scall);
        },
        Err(res) => println!("{:?}", res)
    }

    //buffer
    let m: *mut libc::c_void;

    let m = unsafe { libc::malloc(1024*mem::size_of::<i64>()) as *mut libc::c_void };
    let addr: ptrace::AddressType = m as *mut ffi::c_void;


    loop{
        match waitpid(pid, Some(<WaitPidFlag>::WSTOPPED)) {
            Ok(res) => {
                println!("Res {:?}", res);
                let a = unsafe {ptrace::ptrace(ptrace::Request::PTRACE_GETREGS, pid, addr, std::ptr::null_mut())};
                println!("Address {:?}", addr);

                let read_content = ptrace::read(pid, addr);
                match read_content{
                    Ok(c) => println!("MEMORY {:?}", c),
                    _ => println!("NO MEMORY"),
                }
                let syscall = nix::sys::ptrace::syscall(pid);
                println!("Syscall {:?}", syscall);


//                let r = ptrace::cont(pid, None);
//                println!("Cont {:?}", r)
            }
            _ => {},
        }
    }
}
