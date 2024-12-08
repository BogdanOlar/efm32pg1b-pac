#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    hfrcocal0: Hfrcocal0,
    _reserved1: [u8; 0x08],
    hfrcocal3: Hfrcocal3,
    _reserved2: [u8; 0x08],
    hfrcocal6: Hfrcocal6,
    hfrcocal7: Hfrcocal7,
    hfrcocal8: Hfrcocal8,
    _reserved5: [u8; 0x04],
    hfrcocal10: Hfrcocal10,
    hfrcocal11: Hfrcocal11,
    hfrcocal12: Hfrcocal12,
    _reserved8: [u8; 0x2c],
    auxhfrcocal0: Auxhfrcocal0,
    _reserved9: [u8; 0x08],
    auxhfrcocal3: Auxhfrcocal3,
    _reserved10: [u8; 0x08],
    auxhfrcocal6: Auxhfrcocal6,
    auxhfrcocal7: Auxhfrcocal7,
    auxhfrcocal8: Auxhfrcocal8,
    _reserved13: [u8; 0x04],
    auxhfrcocal10: Auxhfrcocal10,
    auxhfrcocal11: Auxhfrcocal11,
    auxhfrcocal12: Auxhfrcocal12,
}
impl RegisterBlock {
    ///0x80 - HFRCO Calibration Register (4 MHz)
    #[inline(always)]
    pub const fn hfrcocal0(&self) -> &Hfrcocal0 {
        &self.hfrcocal0
    }
    ///0x8c - HFRCO Calibration Register (7 MHz)
    #[inline(always)]
    pub const fn hfrcocal3(&self) -> &Hfrcocal3 {
        &self.hfrcocal3
    }
    ///0x98 - HFRCO Calibration Register (13 MHz)
    #[inline(always)]
    pub const fn hfrcocal6(&self) -> &Hfrcocal6 {
        &self.hfrcocal6
    }
    ///0x9c - HFRCO Calibration Register (16 MHz)
    #[inline(always)]
    pub const fn hfrcocal7(&self) -> &Hfrcocal7 {
        &self.hfrcocal7
    }
    ///0xa0 - HFRCO Calibration Register (19 MHz)
    #[inline(always)]
    pub const fn hfrcocal8(&self) -> &Hfrcocal8 {
        &self.hfrcocal8
    }
    ///0xa8 - HFRCO Calibration Register (26 MHz)
    #[inline(always)]
    pub const fn hfrcocal10(&self) -> &Hfrcocal10 {
        &self.hfrcocal10
    }
    ///0xac - HFRCO Calibration Register (32 MHz)
    #[inline(always)]
    pub const fn hfrcocal11(&self) -> &Hfrcocal11 {
        &self.hfrcocal11
    }
    ///0xb0 - HFRCO Calibration Register (38 MHz)
    #[inline(always)]
    pub const fn hfrcocal12(&self) -> &Hfrcocal12 {
        &self.hfrcocal12
    }
    ///0xe0 - AUXHFRCO Calibration Register (4 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal0(&self) -> &Auxhfrcocal0 {
        &self.auxhfrcocal0
    }
    ///0xec - AUXHFRCO Calibration Register (7 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal3(&self) -> &Auxhfrcocal3 {
        &self.auxhfrcocal3
    }
    ///0xf8 - AUXHFRCO Calibration Register (13 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal6(&self) -> &Auxhfrcocal6 {
        &self.auxhfrcocal6
    }
    ///0xfc - AUXHFRCO Calibration Register (16 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal7(&self) -> &Auxhfrcocal7 {
        &self.auxhfrcocal7
    }
    ///0x100 - AUXHFRCO Calibration Register (19 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal8(&self) -> &Auxhfrcocal8 {
        &self.auxhfrcocal8
    }
    ///0x108 - AUXHFRCO Calibration Register (26 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal10(&self) -> &Auxhfrcocal10 {
        &self.auxhfrcocal10
    }
    ///0x10c - AUXHFRCO Calibration Register (32 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal11(&self) -> &Auxhfrcocal11 {
        &self.auxhfrcocal11
    }
    ///0x110 - AUXHFRCO Calibration Register (38 MHz)
    #[inline(always)]
    pub const fn auxhfrcocal12(&self) -> &Auxhfrcocal12 {
        &self.auxhfrcocal12
    }
}
pub use crate::cmu::hfrcoctrl as hfrcocal0;
pub use crate::cmu::hfrcoctrl as hfrcocal3;
pub use crate::cmu::hfrcoctrl as hfrcocal6;
pub use crate::cmu::hfrcoctrl as hfrcocal7;
pub use crate::cmu::hfrcoctrl as hfrcocal8;
pub use crate::cmu::hfrcoctrl as hfrcocal10;
pub use crate::cmu::hfrcoctrl as hfrcocal11;
pub use crate::cmu::hfrcoctrl as hfrcocal12;
pub use crate::cmu::hfrcoctrl as auxhfrcocal0;
pub use crate::cmu::hfrcoctrl as auxhfrcocal3;
pub use crate::cmu::hfrcoctrl as auxhfrcocal6;
pub use crate::cmu::hfrcoctrl as auxhfrcocal7;
pub use crate::cmu::hfrcoctrl as auxhfrcocal8;
pub use crate::cmu::hfrcoctrl as auxhfrcocal10;
pub use crate::cmu::hfrcoctrl as auxhfrcocal11;
pub use crate::cmu::hfrcoctrl as auxhfrcocal12;
pub use crate::cmu::Hfrcoctrl as Hfrcocal0;
pub use crate::cmu::Hfrcoctrl as Hfrcocal3;
pub use crate::cmu::Hfrcoctrl as Hfrcocal6;
pub use crate::cmu::Hfrcoctrl as Hfrcocal7;
pub use crate::cmu::Hfrcoctrl as Hfrcocal8;
pub use crate::cmu::Hfrcoctrl as Hfrcocal10;
pub use crate::cmu::Hfrcoctrl as Hfrcocal11;
pub use crate::cmu::Hfrcoctrl as Hfrcocal12;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal0;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal3;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal6;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal7;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal8;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal10;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal11;
pub use crate::cmu::Hfrcoctrl as Auxhfrcocal12;
