use nix::libc::{
    c_char, c_void, ifconf, ifreq, ioctl, socket, AF_INET, IFNAMSIZ, SIOCGIFCONF, SOCK_DGRAM,
};
use std::ffi::CStr;
use std::io;
use std::mem;

const MAX_IFACES: usize = 128;

fn main() -> io::Result<()> {
    // Step 1: Create a socket
    let sock_fd = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
    if sock_fd < 0 {
        return Err(io::Error::last_os_error());
    }

    // Step 2: Prepare the buffer for interfaces
    let mut ifreqs: [ifreq; MAX_IFACES] = unsafe { mem::zeroed() };
    let mut ifc = ifconf {
        ifc_len: (mem::size_of::<ifreq>() * MAX_IFACES) as i32,
        ifc_buf: ifreqs.as_mut_ptr() as *mut c_void,
    };

    // Step 3: Perform ioctl to get the list of interfaces
    let res = unsafe { ioctl(sock_fd, SIOCGIFCONF, &mut ifc) };
    if res < 0 {
        return Err(io::Error::last_os_error());
    }

    // Step 4: Calculate the number of interfaces
    let num_ifaces = (ifc.ifc_len as usize) / mem::size_of::<ifreq>();

    // Step 5: Iterate through the interfaces and print their names
    println!("Available network interfaces:");
    for i in 0..num_ifaces {
        let iface_name = unsafe {
            CStr::from_ptr(ifreqs[i].ifr_name.as_ptr() as *const c_char)
                .to_str()
                .unwrap_or("Invalid UTF-8")
        };
        println!("{}", iface_name);
    }

    Ok(())
}
