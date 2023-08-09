use crypto_hash::{digest, Algorithm};
use std::{
    fs::File,
    io::{Read, Result},
    thread, time,
    ptr,
};

fn calculate_hash(data: &[u8]) -> [u8; 32] {
    digest(Algorithm::SHA256, data)
}

fn read_self() -> Result<Vec<u8>> {
    let executable_path = std::env::current_exe()?;
    let mut file = File::open(&executable_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
fn takeaction(){
  //ADD YOUR ACTION
}
fn check_memory_changes(base_address: *const u8, size: usize, original_hash: &[u8; 32]) -> bool {
    let mut buffer = vec![0u8; size];
    unsafe {
        ptr::copy(base_address, buffer.as_mut_ptr(), size);
    }
    let current_hash = calculate_hash(&buffer);
    current_hash != *original_hash
}

fn main() -> Result<()> {
    let original_hash = calculate_hash(&read_self()?);

    let base_address = main as *const u8;
    let code_size = 0;//CODE SIZE

    loop {
        thread::sleep(time::Duration::from_secs(5));

        if read_self()? != read_self()? {
          takeaction();
            break;
        }

        if check_memory_changes(base_address, code_size, &original_hash) {
          takeaction();
            break;
        }
    }

    Ok(())
}
