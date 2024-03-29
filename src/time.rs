use crate::register::{tcfg, cpucfg::Cpucfg};

pub unsafe fn init_trigger(ticks_per_sec: usize) {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    let trigger_freq = stable_counter_freq / ticks_per_sec;
    let mut tcfg = tcfg::read();
    tcfg.init_trigger(trigger_freq);
}

pub unsafe fn get_time_s() -> usize {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    get_time() / stable_counter_freq
}

pub unsafe fn get_time_ms() -> usize {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    get_time() * 1_000 / stable_counter_freq
}

pub unsafe fn get_time_us() -> usize {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    get_time() * 1_000_000 / stable_counter_freq
}

pub unsafe fn get_time() -> usize {
    _rdtime().0
}

unsafe fn _rdtime() -> (usize, usize) {
    // RDTIME{L/H}.W 和 RDTIME.D 指令用于读取恒定频率计时器信息，Stable Counter 值写入通用寄存器 rd 中，Counter ID 号信息写入通用寄存器 rj 中。
    match () {
        #[cfg(all(target_arch = "loongarch64", feature = "inline-asm"))]
        () => {
            let rd: usize; let rj: usize;
            core::arch::asm!("rdtime.d {0}, {1}", out(reg) rd, out(reg) rj);
            (rd, rj)
        }

        #[cfg(all(not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __read_time_rd() -> usize;
                fn __read_time_rj() -> usize;
            }

            (__read_time_rd(), __read_time_rj())
        }
    }
}