use std::mem;

use libc::{STDIN_FILENO, read, timeval, write};

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
        "Channel was created successfully. There are read_fd={} and write_fd={}.",
        read_fd, write_fd
    );

    let mut fds: [libc::pollfd; 2] = unsafe { mem::zeroed() };

    fds[0] = libc::pollfd {
        fd: read_fd,
        events: libc::POLLIN | libc::POLLPRI | libc::POLLRDBAND,
        revents: 0,
    };

    fds[1] = libc::pollfd {
        fd: write_fd,
        events: libc::POLLOUT,
        revents: 0,
    };

    for _ in 0..2 {
        let res = unsafe { libc::poll(fds.as_mut_ptr(), 2, -1) };

        println!("res={}", res);

        for pfd in fds {
            match pfd.revents {
                libc::POLLERR => {
                    eprintln!("Error POLLERR");
                }
                libc::POLLHUP => {
                    eprintln!("Error POLLHUP");
                }
                libc::POLLNVAL => {
                    eprintln!("Error POLLNVAL");
                }
                code => {
                    println!("fd {} contains revents with {}", pfd.fd as i16, code);

                    if code & libc::POLLIN == libc::POLLIN {
                        println!("It's POLLIN");

                        const MSG_LEN: usize = 128;

                        let mut buf = [0u8; MSG_LEN];
                        let bytes =
                            unsafe { read(pfd.fd, buf.as_mut_ptr() as *mut libc::c_void, MSG_LEN) };

                        println!("read {} bytes", bytes);

                        let result = String::from_utf8_lossy(&buf[..bytes as usize]).to_string();
                        println!("result buf: {:?}", result);
                    }
                    if code & libc::POLLOUT == libc::POLLOUT {
                        // assert_ne!(code & libc::POLLOUT, 0);
                        println!("It's POLLOUT.");

                        let msg = b"Yet another message";
                        let bytes =
                            unsafe { write(write_fd, msg.as_ptr() as *const libc::c_void, msg.len()) };
                        println!("Wrote {} bytes", bytes);
                    }
                }
            }
        }
    }
    

    pipe_fds.iter().for_each(|fd| unsafe {
        libc::close(*fd);
    });
}
