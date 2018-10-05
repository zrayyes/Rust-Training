fn main() {
    get_processes();
}

fn get_processes() {
    use std::ffi::CString;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::psapi;

    let mut bytes_returned: DWORD = 0;
    // Create a large array to contain all process ids
    let mut pids: [DWORD; 1024] = [0; 1024];
    unsafe {
        psapi::EnumProcesses(pids.as_mut_ptr(), pids.len() as DWORD, &mut bytes_returned);
    }

    println!("Number of processes running: {}", bytes_returned / 4);

    for pid in pids.iter() {
        if *pid != 0 {
            unsafe {
                let handle = winapi::um::processthreadsapi::OpenProcess(4096, 0, *pid);
                let mut filename: [i8; 255] = [0; 255];
                psapi::GetProcessImageFileNameA(handle, filename.as_mut_ptr(), 255);

                let c_str = CString::from_raw(filename.as_mut_ptr());
                let str_buf: String = c_str.to_str().unwrap().to_owned();
                if str_buf != "" {
                    println!("{}", str_buf);
                }
            }
        }
    }
}
