// TODO: Add process struct with functions

fn main() {
    let process_names = get_processes();
    for name in process_names {
        println!("{}", name);
    }
    pause();
}

fn pause() {
    use std::io;
    use std::io::prelude::*;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn get_processes() -> Vec<String> {
    use regex::Regex;
    use std::ffi::CString;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::psapi;
    let re = Regex::new(r"[^\\]*$").unwrap();

    let mut processes = Vec::new();
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
                    let str_cap = re.captures(&str_buf).unwrap();
                    processes.push(str_cap.get(0).unwrap().as_str().to_owned());
                }
            }
        }
    }
    processes
}
