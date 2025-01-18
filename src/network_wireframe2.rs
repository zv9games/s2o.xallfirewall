use winapi::um::winsock2::{WSACleanup, SOCKET, WSAGetLastError, SOCKET_ERROR, recv, select, timeval, fd_set};
use std::net::Ipv4Addr;
use std::time::Duration;
use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use std::ptr;
use std::error::Error;

pub fn start_packet_capture(socket: SOCKET, interface_name: &str, stop_signal: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
    println!("Starting packet capture...");
    let capture_result = local_capture_packets(socket, |packet| {
        println!("Packet received with length: {}", packet.len());
        if packet.len() >= 20 {
            let src_ip = Ipv4Addr::new(packet[12], packet[13], packet[14], packet[15]);
            let dest_ip = Ipv4Addr::new(packet[16], packet[17], packet[18], packet[19]);
            println!("Src: {}, Dest: {}", src_ip, dest_ip);
        }
    }, stop_signal);

    if capture_result.is_err() {
        eprintln!("Packet capture failed on interface {}: {}", interface_name, capture_result.err().unwrap());
    } else {
        println!("Packet capture successful on interface: {}", interface_name);
    }

    // Cleanup Winsock
    unsafe { WSACleanup() };
    println!("Winsock cleanup complete.");

    Ok(())
}

fn local_capture_packets<F>(socket: SOCKET, handle_packet: F, stop_signal: Arc<AtomicBool>) -> Result<(), Box<dyn Error>>
where
    F: Fn(&[u8]) + Send + Sync + 'static,
{
    let mut buffer = [0u8; 65535];
    let extended_timeout = Duration::from_secs(1);
    let mut readfds = unsafe { std::mem::zeroed::<fd_set>() };
    let mut timeout = timeval {
        tv_sec: 1,
        tv_usec: 0,
    };

    println!("Starting packet capture...");

    while !stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
        unsafe {
            for fd in &mut readfds.fd_array {
                *fd = 0;
            }
            readfds.fd_count = 0;
            if readfds.fd_count < 64 {
                readfds.fd_array[readfds.fd_count as usize] = socket;
                readfds.fd_count += 1;
            }
        }

        println!("Checking for packets...");
        let select_result = unsafe { select(0, &mut readfds, ptr::null_mut(), ptr::null_mut(), &mut timeout) };
        if select_result > 0 {
            println!("Socket is ready for reading...");
            let packet_size = unsafe { recv(socket, buffer.as_mut_ptr() as *mut _, buffer.len() as i32, 0) };
            if packet_size == -1 {
                let error_code = unsafe { WSAGetLastError() };
                eprintln!("Error: Failed to capture packet. Error code: {}", error_code);
                return Err("Failed to capture packet".into());
            }

            if packet_size > 0 {
                println!("Captured packet of size: {}", packet_size);
                handle_packet(&buffer[..packet_size as usize]);
                return Ok(());
            } else {
                println!("Packet size is zero.");
            }
        } else if select_result == 0 {
            println!("No packets available to read (timeout).");
        } else {
            let error_code = unsafe { WSAGetLastError() };
            eprintln!("Error in select. Error code: {}", error_code);
            return Err("Error in select".into());
        }

        std::thread::sleep(extended_timeout);
    }

    println!("Packet capture ended.");
    Ok(())
}
