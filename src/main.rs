extern crate nix;
extern crate libc;
extern crate core;

use core::ffi;
use nix::sys::wait::*;
use nix::sys::signal::Signal; 
use nix::sys::ptrace;
use std::mem;
use std::str::FromStr;
//fn attach(pid: i64) -> Result<

struct Registers {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbp: u64,
    rbx: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rax: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    orig_rax: u64,
    rip: u64,
    cs: u64,
    eflags: u64,
    rsp: u64,
    ss: u64,
    fs_base: u64,
    gs_base: u64,
    ds: u64,
    es: u64,
    fs: u64,
    gs: u64
}

const buffer_size: usize = 27;
const offset: usize = 15;
fn main() {

    let args: Vec<String> = std::env::args().collect();
    let pid_str: &String = args.get(1).unwrap();

    let pid_id = i32::from_str(pid_str).unwrap();
    println!("Attaching PID {}", pid_id);

    let pid = nix::unistd::Pid::from_raw(pid_id);

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

    let m = unsafe { libc::malloc(buffer_size*mem::size_of::<u64>()) as *mut libc::c_void };
    let addr: ptrace::AddressType = m as *mut ffi::c_void;


    loop{
        match waitpid(pid, Some(<WaitPidFlag>::WSTOPPED)) {
            Ok(res) => {
                println!("Res {:?}", res);
                let a = unsafe {ptrace::ptrace(ptrace::Request::PTRACE_GETREGS, pid, addr, m)};
                let mem: &mut [u64; buffer_size]  = unsafe { &mut *(m as *mut [u64; buffer_size]) };
                println!("reg: {} ", (mem[offset]));
//                for x in mem.iter() { 
//                    println!("state: {} | {}", (*x) as u64, x);
//                }

//                unsafe{println!("Address {:?} {:?}", addr, *m)};

//                let read_content = ptrace::read(pid, addr);
//                match read_content{
//                    Ok(c) => println!("MEMORY {:?}", c),
//                    _ => println!("NO MEMORY"),
//                }
                 let syscall = nix::sys::ptrace::syscall(pid);
//                println!("Syscall {:?}", syscall);


//                let r = ptrace::cont(pid, None);
//                println!("Cont {:?}", r)
            }
            _ => {},
        }
    }
}
