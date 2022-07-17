use crate::batch::{get_current_app, validate_app_addr_range_access};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let cur_app = get_current_app();
    let buf_usize = buf as usize;
    validate_app_addr_range_access(cur_app, buf_usize..buf_usize+len);
    
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
