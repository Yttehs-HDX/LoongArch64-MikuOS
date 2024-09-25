use crate::syscall::*;

pub fn exit(code: i32) -> isize {
    sys_exit(code)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}