use super::syscall;

const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    let args = [fd, buffer.as_ptr() as usize, buffer.len()];
    syscall(SYSCALL_WRITE, args)
}