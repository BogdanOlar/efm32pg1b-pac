#[repr(C)]
#[derive(Debug)]
///Port A Registers
#[doc(alias = "PORT_A")]
pub struct PortA {
    ctrl: Ctrl,
    model: Model,
    modeh: Modeh,
    dout: Dout,
    _reserved4: [u8; 0x08],
    dout_tgl: DoutTgl,
    din: Din,
    pinlockn: Pinlockn,
    _reserved7: [u8; 0x04],
    ovt_dis: OvtDis,
}
impl PortA {
    ///0x00 - Port Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Port Pin Mode Low Register
    #[inline(always)]
    pub const fn model(&self) -> &Model {
        &self.model
    }
    ///0x08 - Port Pin Mode High Register
    #[inline(always)]
    pub const fn modeh(&self) -> &Modeh {
        &self.modeh
    }
    ///0x0c - Port Data Out Register
    #[inline(always)]
    pub const fn dout(&self) -> &Dout {
        &self.dout
    }
    ///0x18 - Port Data Out Toggle Register. Write bits to 1 to toggle corresponding bits in GPIO_Px_DOUT. Bits written to 0 will have no effect.
    #[inline(always)]
    pub const fn dout_tgl(&self) -> &DoutTgl {
        &self.dout_tgl
    }
    ///0x1c - Port Data in Register
    #[inline(always)]
    pub const fn din(&self) -> &Din {
        &self.din
    }
    ///0x20 - Port Unlocked Pins Register. Shows unlocked pins in the port. To lock pin n, clear bit n. The pin is then locked until reset.
    #[inline(always)]
    pub const fn pinlockn(&self) -> &Pinlockn {
        &self.pinlockn
    }
    ///0x28 - Over Voltage Disable for All Modes
    #[inline(always)]
    pub const fn ovt_dis(&self) -> &OvtDis {
        &self.ovt_dis
    }
}
///CTRL (rw) register accessor: Port Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrl`]
///module
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
///Port Control Register
pub mod ctrl;
///MODEL (rw) register accessor: Port Pin Mode Low Register
///
///You can [`read`](crate::Reg::read) this register and get [`model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@model`]
///module
#[doc(alias = "MODEL")]
pub type Model = crate::Reg<model::MODELrs>;
///Port Pin Mode Low Register
pub mod model;
///MODEH (rw) register accessor: Port Pin Mode High Register
///
///You can [`read`](crate::Reg::read) this register and get [`modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@modeh`]
///module
#[doc(alias = "MODEH")]
pub type Modeh = crate::Reg<modeh::MODEHrs>;
///Port Pin Mode High Register
pub mod modeh;
///DOUT (rw) register accessor: Port Data Out Register
///
///You can [`read`](crate::Reg::read) this register and get [`dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dout`]
///module
#[doc(alias = "DOUT")]
pub type Dout = crate::Reg<dout::DOUTrs>;
///Port Data Out Register
pub mod dout;
///DOUT_TGL (w) register accessor: Port Data Out Toggle Register. Write bits to 1 to toggle corresponding bits in GPIO_Px_DOUT. Bits written to 0 will have no effect.
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_tgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dout_tgl`]
///module
#[doc(alias = "DOUT_TGL")]
pub type DoutTgl = crate::Reg<dout_tgl::DOUT_TGLrs>;
///Port Data Out Toggle Register. Write bits to 1 to toggle corresponding bits in GPIO_Px_DOUT. Bits written to 0 will have no effect.
pub mod dout_tgl;
///DIN (r) register accessor: Port Data in Register
///
///You can [`read`](crate::Reg::read) this register and get [`din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@din`]
///module
#[doc(alias = "DIN")]
pub type Din = crate::Reg<din::DINrs>;
///Port Data in Register
pub mod din;
///PINLOCKN (rw) register accessor: Port Unlocked Pins Register. Shows unlocked pins in the port. To lock pin n, clear bit n. The pin is then locked until reset.
///
///You can [`read`](crate::Reg::read) this register and get [`pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pinlockn`]
///module
#[doc(alias = "PINLOCKN")]
pub type Pinlockn = crate::Reg<pinlockn::PINLOCKNrs>;
///Port Unlocked Pins Register. Shows unlocked pins in the port. To lock pin n, clear bit n. The pin is then locked until reset.
pub mod pinlockn;
///OVT_DIS (rw) register accessor: Over Voltage Disable for All Modes
///
///You can [`read`](crate::Reg::read) this register and get [`ovt_dis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovt_dis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ovt_dis`]
///module
#[doc(alias = "OVT_DIS")]
pub type OvtDis = crate::Reg<ovt_dis::OVT_DISrs>;
///Over Voltage Disable for All Modes
pub mod ovt_dis;
