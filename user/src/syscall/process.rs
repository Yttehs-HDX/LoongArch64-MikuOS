use super::syscall;

const SYSCALL_EXIT: usize = 93;

pub fn sys_exit(code: i32) -> isize {
    let args = [code as usize, 0, 0];
    syscall(SYSCALL_EXIT, args)
}