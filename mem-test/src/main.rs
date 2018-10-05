fn main() {
    get_processes();
}

fn get_processes() {
    use winapi;
    
    let mut bytes_returned: u32 = 0;
    // Create a large array to contain all process ids
    let mut pids: [u32; 1024] = [0; 1024];
    unsafe {
        winapi::um::psapi::EnumProcesses(pids.as_mut_ptr(), pids.len() as u32, &mut bytes_returned);
    }

    println!("Number of processes running: {}", bytes_returned / 4);
    println!("{:?}",  &pids[0..]);
}
