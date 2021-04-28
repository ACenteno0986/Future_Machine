#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
extern "C" {
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn send(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn sendfile(__out_fd: libc::c_int, __in_fd: libc::c_int,
                __offset: *mut off_t, __count: size_t) -> ssize_t;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut server: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut obj: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut sock: libc::c_int = 0;
    let mut choice: libc::c_int = 0;
    let mut buf: [libc::c_char; 100] = [0; 100];
    let mut command: [libc::c_char; 5] = [0; 5];
    let mut filename: [libc::c_char; 20] = [0; 20];
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut k: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut filehandle: libc::c_int = 0;
    sock =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if sock == -(1 as libc::c_int) {
        printf(b"socket creation failed\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    server.sin_family = 2 as libc::c_int as sa_family_t;
    server.sin_port =
        atoi(*argv.offset(1 as libc::c_int as isize)) as in_port_t;
    server.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    k =
        connect(sock, &mut server as *mut sockaddr_in as *mut sockaddr,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    socklen_t);
    if k == -(1 as libc::c_int) {
        printf(b"Connect Error\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut limite: libc::c_int = 2 as libc::c_int;
    while limite < argc {
        if strcmp(*argv.offset(limite as isize),
                  b"get\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            printf(b"Enter filename to get: \x00" as *const u8 as
                       *const libc::c_char);
            scanf(b"%s\x00" as *const u8 as *const libc::c_char,
                  filename.as_mut_ptr());
            strcpy(buf.as_mut_ptr(),
                   b"get \x00" as *const u8 as *const libc::c_char);
            strcat(buf.as_mut_ptr(), filename.as_mut_ptr());
            send(sock, buf.as_mut_ptr() as *const libc::c_void,
                 100 as libc::c_int as size_t, 0 as libc::c_int);
            recv(sock, &mut size as *mut libc::c_int as *mut libc::c_void,
                 ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                 0 as libc::c_int);
            if size == 0 {
                printf(b"No such file on the remote directory\n\n\x00" as
                           *const u8 as *const libc::c_char);
                break ;
            } else {
                f = malloc(size as libc::c_ulong) as *mut libc::c_char;
                recv(sock, f as *mut libc::c_void, size as size_t,
                     0 as libc::c_int);
                loop  {
                    filehandle =
                        open(filename.as_mut_ptr(),
                             0o100 as libc::c_int | 0o200 as libc::c_int |
                                 0o1 as libc::c_int, 0o666 as libc::c_int);
                    if !(filehandle == -(1 as libc::c_int)) { break ; }
                    sprintf(filename.as_mut_ptr().offset(strlen(filename.as_mut_ptr())
                                                             as isize),
                            b"%d\x00" as *const u8 as *const libc::c_char, i);
                    //needed only if same directory is used for both server and client
                }
                write(filehandle, f, size, 0 as libc::c_int);
                close(filehandle);
                strcpy(buf.as_mut_ptr(),
                       b"cat \x00" as *const u8 as *const libc::c_char);
                strcat(buf.as_mut_ptr(), filename.as_mut_ptr());
                system(buf.as_mut_ptr());
                break ;
            }
        } else if strcmp(*argv.offset(limite as isize),
                         b"put\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            printf(b"Enter filename to put to server: \x00" as *const u8 as
                       *const libc::c_char);
            scanf(b"%s\x00" as *const u8 as *const libc::c_char,
                  filename.as_mut_ptr());
            filehandle = open(filename.as_mut_ptr(), 0 as libc::c_int);
            if filehandle == -(1 as libc::c_int) {
                printf(b"No such file on the local directory\n\n\x00" as
                           *const u8 as *const libc::c_char);
                break ;
            } else {
                strcpy(buf.as_mut_ptr(),
                       b"put \x00" as *const u8 as *const libc::c_char);
                strcat(buf.as_mut_ptr(), filename.as_mut_ptr());
                send(sock, buf.as_mut_ptr() as *const libc::c_void,
                     100 as libc::c_int as size_t, 0 as libc::c_int);
                stat(filename.as_mut_ptr(), &mut obj);
                size = obj.st_size as libc::c_int;
                send(sock,
                     &mut size as *mut libc::c_int as *const libc::c_void,
                     ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                     0 as libc::c_int);
                sendfile(sock, filehandle, 0 as *mut off_t, size as size_t);
                recv(sock,
                     &mut status as *mut libc::c_int as *mut libc::c_void,
                     ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                     0 as libc::c_int);
                if status != 0 {
                    printf(b"File stored successfully\n\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    printf(b"File failed to be stored to remote machine\n\x00"
                               as *const u8 as *const libc::c_char);
                }
                break ;
            }
        } else if strcmp(*argv.offset(limite as isize),
                         b"pwd\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            strcpy(buf.as_mut_ptr(),
                   b"pwd\x00" as *const u8 as *const libc::c_char);
            send(sock, buf.as_mut_ptr() as *const libc::c_void,
                 100 as libc::c_int as size_t, 0 as libc::c_int);
            recv(sock, buf.as_mut_ptr() as *mut libc::c_void,
                 100 as libc::c_int as size_t, 0 as libc::c_int);
            printf(b"The path of the remote directory is: %s\n\x00" as
                       *const u8 as *const libc::c_char, buf.as_mut_ptr());
            break ;
        } else if strcmp(*argv.offset(limite as isize),
                         b"ls\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            strcpy(buf.as_mut_ptr(),
                   b"ls\x00" as *const u8 as *const libc::c_char);
            send(sock, buf.as_mut_ptr() as *const libc::c_void,
                 100 as libc::c_int as size_t, 0 as libc::c_int);
            recv(sock, &mut size as *mut libc::c_int as *mut libc::c_void,
                 ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                 0 as libc::c_int);
            f = malloc(size as libc::c_ulong) as *mut libc::c_char;
            recv(sock, f as *mut libc::c_void, size as size_t,
                 0 as libc::c_int);
            filehandle =
                creat(b"temp.txt\x00" as *const u8 as *const libc::c_char,
                      0o1 as libc::c_int as mode_t);
            write(filehandle, f, size, 0 as libc::c_int);
            close(filehandle);
            printf(b"The remote directory listing is as follows:\n\x00" as
                       *const u8 as *const libc::c_char);
            system(b"cat temp.txt\x00" as *const u8 as *const libc::c_char);
            break ;
        } else if strcmp(*argv.offset(limite as isize),
                         b"cd\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            strcpy(buf.as_mut_ptr(),
                   b"cd \x00" as *const u8 as *const libc::c_char);
            printf(b"Enter the path to change the remote directory: \x00" as
                       *const u8 as *const libc::c_char);
            scanf(b"%s\x00" as *const u8 as *const libc::c_char,
                  buf.as_mut_ptr().offset(3 as libc::c_int as isize));
            send(sock, buf.as_mut_ptr() as *const libc::c_void,
                 100 as libc::c_int as size_t, 0 as libc::c_int);
            recv(sock, &mut status as *mut libc::c_int as *mut libc::c_void,
                 ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                 0 as libc::c_int);
            if status != 0 {
                printf(b"Remote directory successfully changed\n\x00" as
                           *const u8 as *const libc::c_char);
            } else {
                printf(b"Remote directory failed to change\n\x00" as *const u8
                           as *const libc::c_char);
            }
            break ;
        } else {
            if strcmp(*argv.offset(limite as isize),
                      b"quit\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                strcpy(buf.as_mut_ptr(),
                       b"quit\x00" as *const u8 as *const libc::c_char);
                send(sock, buf.as_mut_ptr() as *const libc::c_void,
                     100 as libc::c_int as size_t, 0 as libc::c_int);
                recv(sock,
                     &mut status as *mut libc::c_int as *mut libc::c_void,
                     100 as libc::c_int as size_t, 0 as libc::c_int);
                if status != 0 {
                    printf(b"Server closed\nQuitting..\n\x00" as *const u8 as
                               *const libc::c_char);
                    exit(0 as libc::c_int);
                }
                printf(b"Server failed to close connection\n\x00" as *const u8
                           as *const libc::c_char);
            }
            limite += 1 as libc::c_int
        }
    }
    return 0;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}

    for (syscall, &number) in map.iter() {
        println!("{}: {}", syscall, number);
    }
}
