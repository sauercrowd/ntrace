extern crate nix;
extern crate libc;
extern crate core;

use core::ffi;
use nix::sys::wait::*;
use nix::sys::signal::Signal; 
use nix::sys::ptrace;
use std::mem;
use std::str::FromStr;

struct PidRegisterSession{
    buffer: *mut libc::c_void,
    pid: nix::unistd::Pid,
}


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

impl Registers{
    fn from_pid(pid: nix::unistd::Pid) -> Registers{

        const buffer_size: usize = 27;
        unsafe{
            let buffer_raw = libc::malloc(buffer_size*mem::size_of::<u64>()) as *mut libc::c_void;
            ptrace::ptrace(ptrace::Request::PTRACE_GETREGS, pid, std::ptr::null_mut(), buffer_raw).unwrap();
            let buffer: &mut [u64; buffer_size]  = &mut *(buffer_raw as *mut [u64; buffer_size]);
            let regs = Registers{
                r15: buffer[0],
                r14: buffer[1],
                r13: buffer[2],
                r12: buffer[3],
                rbp: buffer[4],
                rbx: buffer[5],
                r11: buffer[6],
                r10: buffer[7],
                r9: buffer[8],
                r8: buffer[9],
                rax: buffer[10],
                rcx: buffer[11],
                rdx: buffer[12],
                rsi: buffer[13],
                rdi: buffer[14],
                orig_rax: buffer[15],
                rip: buffer[16],
                cs: buffer[17],
                eflags: buffer[18],
                rsp: buffer[19],
                ss: buffer[20],
                fs_base: buffer[21],
                gs_base: buffer[22],
                ds: buffer[23],
                es: buffer[24],
                fs: buffer[25],
                gs: buffer[26]
            };
            libc::free(buffer_raw);
            return regs;
        }
    }
}

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

    loop{
        match waitpid(pid, Some(<WaitPidFlag>::WSTOPPED)) {
            Ok(res) => {
                println!("Res {:?}", res);
                let regs = Registers::from_pid(pid);
                println!("RDI {}", regs.rdi);
                let syscall = nix::sys::ptrace::syscall(pid);
            }
            _ => {},
        }
    }
}
