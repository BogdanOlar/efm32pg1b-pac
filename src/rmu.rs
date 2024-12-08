#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    rstcause: Rstcause,
    cmd: Cmd,
    rst: Rst,
    lock: Lock,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Reset Cause Register
    #[inline(always)]
    pub const fn rstcause(&self) -> &Rstcause {
        &self.rstcause
    }
    ///0x08 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x0c - Reset Control Register
    #[inline(always)]
    pub const fn rst(&self) -> &Rst {
        &self.rst
    }
    ///0x10 - Configuration Lock Register
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
///CTRL (rw) register accessor: Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrl`]
///module
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
///Control Register
pub mod ctrl;
///RSTCAUSE (r) register accessor: Reset Cause Register
///
///You can [`read`](crate::Reg::read) this register and get [`rstcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rstcause`]
///module
#[doc(alias = "RSTCAUSE")]
pub type Rstcause = crate::Reg<rstcause::RSTCAUSErs>;
///Reset Cause Register
pub mod rstcause;
///CMD (w) register accessor: Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cmd`]
///module
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CMDrs>;
///Command Register
pub mod cmd;
///RST (rw) register accessor: Reset Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rst`]
///module
#[doc(alias = "RST")]
pub type Rst = crate::Reg<rst::RSTrs>;
///Reset Control Register
pub mod rst;
///LOCK (rw) register accessor: Configuration Lock Register
///
///You can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lock`]
///module
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LOCKrs>;
///Configuration Lock Register
pub mod lock;
