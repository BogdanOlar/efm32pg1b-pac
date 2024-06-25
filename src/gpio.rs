#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    port_a: PortA,
    _reserved1: [u8; 0x04],
    port_b: PortB,
    _reserved2: [u8; 0x04],
    port_c: PortC,
    _reserved3: [u8; 0x04],
    port_d: PortD,
    _reserved4: [u8; 0x04],
    port_e: PortE,
    _reserved5: [u8; 0x04],
    port_f: PortF,
    _reserved6: [u8; 0x02e4],
    extipsell: Extipsell,
    extipselh: Extipselh,
    extipinsell: Extipinsell,
    extipinselh: Extipinselh,
    extirise: Extirise,
    extifall: Extifall,
    extilevel: Extilevel,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    em4wuen: Em4wuen,
    _reserved18: [u8; 0x10],
    routepen: Routepen,
    routeloc0: Routeloc0,
    _reserved20: [u8; 0x08],
    insense: Insense,
    lock: Lock,
}
impl RegisterBlock {
    #[doc = "0x00..0x2c - Port A Registers"]
    #[inline(always)]
    pub const fn port_a(&self) -> &PortA {
        &self.port_a
    }
    #[doc = "0x30..0x5c - Port B Registers"]
    #[inline(always)]
    pub const fn port_b(&self) -> &PortB {
        &self.port_b
    }
    #[doc = "0x60..0x8c - Port C Registers"]
    #[inline(always)]
    pub const fn port_c(&self) -> &PortC {
        &self.port_c
    }
    #[doc = "0x90..0xbc - Port D Registers"]
    #[inline(always)]
    pub const fn port_d(&self) -> &PortD {
        &self.port_d
    }
    #[doc = "0xc0..0xec - Port E Registers"]
    #[inline(always)]
    pub const fn port_e(&self) -> &PortE {
        &self.port_e
    }
    #[doc = "0xf0..0x11c - Port F Registers"]
    #[inline(always)]
    pub const fn port_f(&self) -> &PortF {
        &self.port_f
    }
    #[doc = "0x400 - External Interrupt Port Select Low Register"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &Extipsell {
        &self.extipsell
    }
    #[doc = "0x404 - External Interrupt Port Select High Register"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &Extipselh {
        &self.extipselh
    }
    #[doc = "0x408 - External Interrupt Pin Select Low Register"]
    #[inline(always)]
    pub const fn extipinsell(&self) -> &Extipinsell {
        &self.extipinsell
    }
    #[doc = "0x40c - External Interrupt Pin Select High Register"]
    #[inline(always)]
    pub const fn extipinselh(&self) -> &Extipinselh {
        &self.extipinselh
    }
    #[doc = "0x410 - External Interrupt Rising Edge Trigger Register"]
    #[inline(always)]
    pub const fn extirise(&self) -> &Extirise {
        &self.extirise
    }
    #[doc = "0x414 - External Interrupt Falling Edge Trigger Register"]
    #[inline(always)]
    pub const fn extifall(&self) -> &Extifall {
        &self.extifall
    }
    #[doc = "0x418 - External Interrupt Level Register"]
    #[inline(always)]
    pub const fn extilevel(&self) -> &Extilevel {
        &self.extilevel
    }
    #[doc = "0x41c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x420 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x424 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x428 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x42c - EM4 Wake Up Enable Register"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x440 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x444 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0x450 - Input Sense Register"]
    #[inline(always)]
    pub const fn insense(&self) -> &Insense {
        &self.insense
    }
    #[doc = "0x454 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "Port A Registers"]
pub use self::port_a::PortA;
#[doc = r"Cluster"]
#[doc = "Port A Registers"]
pub mod port_a;
pub use self::port_a as port_b;
pub use self::port_a as port_c;
pub use self::port_a as port_d;
pub use self::port_a as port_e;
pub use self::port_a as port_f;
#[doc = "Port B Registers"]
pub use self::PortA as PortB;
#[doc = "Port C Registers"]
pub use self::PortA as PortC;
#[doc = "Port D Registers"]
pub use self::PortA as PortD;
#[doc = "Port E Registers"]
pub use self::PortA as PortE;
#[doc = "Port F Registers"]
pub use self::PortA as PortF;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`]
module"]
#[doc(alias = "EXTIPSELL")]
pub type Extipsell = crate::Reg<extipsell::EXTIPSELLrs>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External Interrupt Port Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`]
module"]
#[doc(alias = "EXTIPSELH")]
pub type Extipselh = crate::Reg<extipselh::EXTIPSELHrs>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: External Interrupt Pin Select Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinsell`]
module"]
#[doc(alias = "EXTIPINSELL")]
pub type Extipinsell = crate::Reg<extipinsell::EXTIPINSELLrs>;
#[doc = "External Interrupt Pin Select Low Register"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: External Interrupt Pin Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinselh`]
module"]
#[doc(alias = "EXTIPINSELH")]
pub type Extipinselh = crate::Reg<extipinselh::EXTIPINSELHrs>;
#[doc = "External Interrupt Pin Select High Register"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extirise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`]
module"]
#[doc(alias = "EXTIRISE")]
pub type Extirise = crate::Reg<extirise::EXTIRISErs>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extifall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`]
module"]
#[doc(alias = "EXTIFALL")]
pub type Extifall = crate::Reg<extifall::EXTIFALLrs>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
#[doc = "EXTILEVEL (rw) register accessor: External Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extilevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extilevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extilevel`]
module"]
#[doc(alias = "EXTILEVEL")]
pub type Extilevel = crate::Reg<extilevel::EXTILEVELrs>;
#[doc = "External Interrupt Level Register"]
pub mod extilevel;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IFrs>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IFSrs>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IFCrs>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IENrs>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "EM4WUEN (rw) register accessor: EM4 Wake Up Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`]
module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::EM4WUENrs>;
#[doc = "EM4 Wake Up Enable Register"]
pub mod em4wuen;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::ROUTEPENrs>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::ROUTELOC0rs>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "INSENSE (rw) register accessor: Input Sense Register\n\nYou can [`read`](crate::Reg::read) this register and get [`insense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@insense`]
module"]
#[doc(alias = "INSENSE")]
pub type Insense = crate::Reg<insense::INSENSErs>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LOCKrs>;
#[doc = "Configuration Lock Register"]
pub mod lock;
