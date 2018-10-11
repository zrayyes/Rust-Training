// see
// https://www.codeproject.com/Articles/4865/Performing-a-hex-dump-of-another-process-s-memory

use regex::Regex;
use std::ffi::CString;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::DWORD;
use winapi::um::psapi;

pub struct PROCESS {
    pub pid: u32,
    handle: *mut c_void,
    pub name: String,
}

fn stringify_filename(mut name: [i8; 255]) -> String {
    let re = Regex::new(r"[^\\]*$").unwrap();
    unsafe {
        let c_str = CString::from_raw(name.as_mut_ptr());
        let str_buf: String = c_str.to_str().unwrap().to_owned();
        let str_cap = re.captures(&str_buf).unwrap();
        str_cap.get(0).unwrap().as_str().to_owned()
    }
}

pub fn get_processes() -> Vec<PROCESS> {
    let mut processes = Vec::new();
    let mut bytes_returned: DWORD = 0;
    // Create a large array to contain all process ids
    let mut pids: [DWORD; 1024] = [0; 1024];
    unsafe {
        psapi::EnumProcesses(pids.as_mut_ptr(), pids.len() as DWORD, &mut bytes_returned);
    }

    for pid in pids.iter() {
        if *pid != 0 {
            unsafe {
                let handle = winapi::um::processthreadsapi::OpenProcess(4096, 0, *pid);
                let mut filename: [i8; 255] = [0; 255];
                psapi::GetProcessImageFileNameA(handle, filename.as_mut_ptr(), 255);

                let p = PROCESS {
                    pid: *pid,
                    handle: handle,
                    name: stringify_filename(filename),
                };

                if p.name != "" {
                    processes.push(p);
                }
            }
        }
    }
    processes.sort_unstable_by_key(|p| p.pid);
    processes
}

pub fn get_process_from_vector(vector: Vec<PROCESS>, name: &str) -> PROCESS {
    for p in vector {
        if p.name == *name {
            return p;
        }
    }
    panic!("No process found with that name")
}
