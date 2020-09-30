#![feature(link_llvm_intrinsics)]
#![feature(allocator_api)]
#![feature(stdsimd)]
#![feature(panic_info_message)]

mod lock;

use lock::Mutex;

extern "C" {
    fn _logs(ptr: usize, len: usize);
    fn _logi(ptr: usize, len: usize, n: usize);
}

fn logs(msg: &str) {
    unsafe { _logs(msg.as_ptr() as usize, msg.len()) }
}

fn logi(msg: &str, n: usize) {
    unsafe { _logi(msg.as_ptr() as usize, msg.len(), n) }
}

static ISINIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static mut GLOBLOCK: Mutex<[u64; 4]> = Mutex::new([1, 2, 3, 4]);

#[export_name = "initlock"]
pub extern "C" fn initlock() -> bool {
    ISINIT.compare_and_swap(false, true, std::sync::atomic::Ordering::SeqCst)
}

#[export_name = "start"]
pub extern "C" fn start(n: usize) {
    match n {
        0 => f1(),
        1 => f2(),
        _ => (),
    }
}

fn sleep(s: u64) {
    for j in 0..(1024 * s) {
        let mut n = 1024.0f64;
        for i in 0..1024 {
            n = n.log(i as f64 / 1024.0);
        }
    }
}

fn f1() {
    logs("1: started");
    logs("1: locking mutex");
    {
        let mut lock = unsafe { GLOBLOCK.lock() };
        lock[3] += 1;
        logs("1: and now sleepin");
        sleep(16);
    }
    logs("1: unlocking mutex");
}

fn f2() {
    logs("2: started");
    sleep(3);
    logs("2: try locking mutex");
    {
        let mut lock = unsafe { GLOBLOCK.lock() };
        logs("2: locking mutex");
        lock[3] += 1;
    }
    logs("2: unlocking mutex");
}
