use winapi::um::winsock2::{
    WSADATA, WSAStartup, WSACleanup, SOCKET, INVALID_SOCKET, SOCK_RAW, socket, WSAGetLastError,
    SOCKET_ERROR, bind, setsockopt, fd_set, IPPROTO_RAW, timeval,
};
use winapi::shared::ws2def::{AF_INET, SOCKADDR_IN, SOCKADDR, IPPROTO_IP};
use winapi::shared::ws2ipdef::IP_HDRINCL;
use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use std::ptr;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Initializing Winsock...");
    let mut wsa_data: WSADATA = unsafe { std::mem::zeroed() };
    if unsafe { WSAStartup(0x0202, &mut wsa_data) } != 0 {
        eprintln!("WSAStartup failed with error: {}", unsafe { WSAGetLastError() });
        return Err("WSAStartup failed".into());
    }
    println!("Winsock initialized successfully.");

    println!("Creating socket...");
    let socket: SOCKET = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_RAW as i32) };
    if socket == INVALID_SOCKET {
        eprintln!("Failed to create socket with error: {}", unsafe { WSAGetLastError() });
        unsafe { WSACleanup() };
        return Err("Failed to create socket".into());
    }
    println!("Socket created successfully.");

    println!("Setting socket options...");
    let opt_val: i32 = 1;
    if unsafe { setsockopt(socket, IPPROTO_IP as i32, IP_HDRINCL as i32, &opt_val as *const _ as *const i8, std::mem::size_of::<i32>() as i32) } == SOCKET_ERROR {
        eprintln!("Failed to set socket options with error: {}", unsafe { WSAGetLastError() });
        unsafe { WSACleanup() };
        return Err("Failed to set socket options".into());
    }
    println!("Socket options set successfully.");

    println!("Binding socket...");
    let addr = SOCKADDR_IN {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: unsafe { std::mem::zeroed() },
        sin_zero: [0; 8],
    };
    let result = unsafe {
        bind(socket, &addr as *const _ as *const SOCKADDR, std::mem::size_of::<SOCKADDR_IN>() as i32)
    };
    if result == SOCKET_ERROR {
        eprintln!("Failed to bind socket with error: {}", unsafe { WSAGetLastError() });
        unsafe { WSACleanup() };
        return Err("Failed to bind socket".into());
    }
    println!("Socket bound successfully.");

    println!("All basic operations succeeded.");
    unsafe { WSACleanup() };
    Ok(())
}
