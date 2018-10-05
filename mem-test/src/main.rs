fn main() {
    get_processes();
}

fn get_processes() {
    use winapi;
    use winapi::shared::minwindef::DWORD;
    
    let mut bytes_returned: DWORD = 0;
    // Create a large array to contain all process ids
    let mut pids: [DWORD; 1024] = [0; 1024];
    unsafe {
        winapi::um::psapi::EnumProcesses(pids.as_mut_ptr(), pids.len() as DWORD, &mut bytes_returned);
    }

    println!("Number of processes running: {}", bytes_returned / 4);
    
    for pid in pids.iter() {
        if *pid != 0 {
            println!("{}",*pid);
        } 
    }
}
