use std::{mem, ptr};

use libc::{read, write, timeval};

fn main() {
    let mut pipe_fds = [0i32; 2];

    let res = unsafe { libc::pipe(pipe_fds.as_mut_ptr() as *mut libc::c_int) };

    if res == -1 {
        println!("Failed create channel. Close.");
        return;
    }

    let read_fd = pipe_fds[0];
    let write_fd = pipe_fds[1];

    println!(
        "Channel was created successfully. There are {} and {} fds.",
        read_fd, write_fd
    );

    let msg = b"First message";
    let bytes = unsafe { write(write_fd, msg.as_ptr() as *const libc::c_void, msg.len()) };
    println!("Wrote {} bytes", bytes);

    let mut readfds: libc::fd_set = unsafe { mem::zeroed() };
    let mut writefds: libc::fd_set = unsafe { mem::zeroed() };

    for _ in 0..3 {
        match unsafe {
            libc::FD_ZERO(&mut readfds);
            libc::FD_ZERO(&mut writefds);

            libc::FD_SET(read_fd, &mut readfds);
            libc::FD_SET(write_fd, &mut writefds);

            let mut tm = timeval{
                tv_sec: 5,
                tv_usec: 0,
            };

            libc::select(
                write_fd + 1,
                &mut readfds,
                &mut writefds,
                ptr::null_mut(),
                &mut tm,
            )
        } {
            -1 => {
                eprintln!("ret -1");
                return;
            }
            0 => {
                eprintln!("Timeout");
                return;
            }
            count_ready_fds => {
                println!("Ready fds: {}", count_ready_fds);

                if unsafe { libc::FD_ISSET(read_fd, &readfds) } {
                    println!("Found read");
                    
                    const MSG_LEN: usize = 128;

                    let mut buf = [0u8; MSG_LEN];
                    let bytes = unsafe { read(read_fd, buf.as_mut_ptr() as *mut libc::c_void, MSG_LEN) };

                    println!("read {} bytes", bytes);

                    let result = String::from_utf8_lossy(&buf[..bytes as usize]).to_string();
                    println!("result buf: {:?}", result);
                };

                if unsafe { libc::FD_ISSET(write_fd, &writefds) } {
                    println!("Found write");
                    let msg = b"Yet another message";
                    let bytes =
                        unsafe { write(write_fd, msg.as_ptr() as *const libc::c_void, msg.len()) };
                    println!("Wrote {} bytes", bytes);
                }
            }
        }
    }

    pipe_fds.iter().for_each(|fd| unsafe {
        libc::close(*fd);
    });
}
