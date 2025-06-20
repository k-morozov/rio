use std::mem;

const MAX_EPOLL_EVENTS: usize = 32;

fn main() {
    let epfd = unsafe { libc::epoll_create1(0) };

    let mut ev = libc::epoll_event {
        events: libc::EPOLLIN as u32,
        u64: libc::STDIN_FILENO as u64,
    };

    let ctl_res =
        unsafe { libc::epoll_ctl(epfd, libc::EPOLL_CTL_ADD, libc::STDIN_FILENO, &mut ev) };

    println!("ctl_res={}", ctl_res);

    let mut events: [libc::epoll_event; MAX_EPOLL_EVENTS] = unsafe { mem::zeroed() };

    let nfds = unsafe { libc::epoll_wait(epfd, events.as_mut_ptr(), MAX_EPOLL_EVENTS as i32, -1) };

    println!("Found {} nfds", nfds);

    for idx in 0..nfds {
        let e = &events[idx as usize];
        if e.events == libc::EPOLLIN as u32 {
            let fd = e.u64 as i32;
            println!("Found EPOLLIN for {} fd", fd);

            const MSG_LEN: usize = 128;

            let mut buf = [0u8; MSG_LEN];
            let bytes = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, MSG_LEN) };

            let text = String::from_utf8_lossy(&buf[..bytes as usize]).to_string();
            println!("read {} bytes, text={}", bytes, text);
        }
    }

    unsafe {
        libc::close(epfd);
    }
}
