use std::io::{self, Read};
use std::fs::File;
use axhal::paging::MappingFlags;
use axhal::mem::{PAGE_SIZE_4K, phys_to_virt};
use axmm::AddrSpace;
use crate::VM_ENTRY;

pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    let mut buf = [0u8; 64];
    let n_bytes_read = load_file(fname, &mut buf)?;
    warn!("Load file read {} bytes.", n_bytes_read);
    warn!("File contents: {:?}", buf);
    // let mut hex_string = std::string::String::new();

    // for &b in &buf {
    //     // The {:02x} format specifier formats 'b' as a two-digit,
    //     // lowercase hexadecimal number, with leading zeros as needed.
    //     write!(&mut hex_string, "{:02x} ", b).expect("Unable to write");
    // }

    // ax_println!("Hex representation: {}", hex_string);

    uspace.map_alloc(VM_ENTRY.into(), PAGE_SIZE_4K, MappingFlags::READ|MappingFlags::WRITE|MappingFlags::EXECUTE|MappingFlags::USER, true).unwrap();

    let (paddr, _, _) = uspace
        .page_table()
        .query(VM_ENTRY.into())
        .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", VM_ENTRY));

    ax_println!("paddr: {:#x}", paddr);

    unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            PAGE_SIZE_4K,
        );
    }
    warn!("Copied file {} to paddr: {:#x}", fname, paddr);
    Ok(())
}

fn load_file(fname: &str, buf: &mut [u8]) -> io::Result<usize> {
    ax_println!("app: {}", fname);
    let mut file = File::open(fname)?;
    let n = file.read(buf)?;
    Ok(n)
}
