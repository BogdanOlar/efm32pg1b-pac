#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    init: Init,
    poly: Poly,
    inputdata: Inputdata,
    inputdatahword: Inputdatahword,
    inputdatabyte: Inputdatabyte,
    data: Data,
    datarev: Datarev,
    databyterev: Databyterev,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 - Command Register
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    ///0x08 - CRC Init Value
    #[inline(always)]
    pub const fn init(&self) -> &Init {
        &self.init
    }
    ///0x0c - CRC Polynomial Value
    #[inline(always)]
    pub const fn poly(&self) -> &Poly {
        &self.poly
    }
    ///0x10 - Input 32-bit Data Register
    #[inline(always)]
    pub const fn inputdata(&self) -> &Inputdata {
        &self.inputdata
    }
    ///0x14 - Input 16-bit Data Register
    #[inline(always)]
    pub const fn inputdatahword(&self) -> &Inputdatahword {
        &self.inputdatahword
    }
    ///0x18 - Input 8-bit Data Register
    #[inline(always)]
    pub const fn inputdatabyte(&self) -> &Inputdatabyte {
        &self.inputdatabyte
    }
    ///0x1c - CRC Data Register
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    ///0x20 - CRC Data Reverse Register
    #[inline(always)]
    pub const fn datarev(&self) -> &Datarev {
        &self.datarev
    }
    ///0x24 - CRC Data Byte Reverse Register
    #[inline(always)]
    pub const fn databyterev(&self) -> &Databyterev {
        &self.databyterev
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
///INIT (rw) register accessor: CRC Init Value
///
///You can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@init`]
///module
#[doc(alias = "INIT")]
pub type Init = crate::Reg<init::INITrs>;
///CRC Init Value
pub mod init;
///POLY (rw) register accessor: CRC Polynomial Value
///
///You can [`read`](crate::Reg::read) this register and get [`poly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@poly`]
///module
#[doc(alias = "POLY")]
pub type Poly = crate::Reg<poly::POLYrs>;
///CRC Polynomial Value
pub mod poly;
///INPUTDATA (rw) register accessor: Input 32-bit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@inputdata`]
///module
#[doc(alias = "INPUTDATA")]
pub type Inputdata = crate::Reg<inputdata::INPUTDATArs>;
///Input 32-bit Data Register
pub mod inputdata;
///INPUTDATAHWORD (rw) register accessor: Input 16-bit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputdatahword::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatahword::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@inputdatahword`]
///module
#[doc(alias = "INPUTDATAHWORD")]
pub type Inputdatahword = crate::Reg<inputdatahword::INPUTDATAHWORDrs>;
///Input 16-bit Data Register
pub mod inputdatahword;
///INPUTDATABYTE (rw) register accessor: Input 8-bit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputdatabyte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatabyte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@inputdatabyte`]
///module
#[doc(alias = "INPUTDATABYTE")]
pub type Inputdatabyte = crate::Reg<inputdatabyte::INPUTDATABYTErs>;
///Input 8-bit Data Register
pub mod inputdatabyte;
///DATA (r) register accessor: CRC Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@data`]
///module
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DATArs>;
///CRC Data Register
pub mod data;
///DATAREV (r) register accessor: CRC Data Reverse Register
///
///You can [`read`](crate::Reg::read) this register and get [`datarev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@datarev`]
///module
#[doc(alias = "DATAREV")]
pub type Datarev = crate::Reg<datarev::DATAREVrs>;
///CRC Data Reverse Register
pub mod datarev;
///DATABYTEREV (r) register accessor: CRC Data Byte Reverse Register
///
///You can [`read`](crate::Reg::read) this register and get [`databyterev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@databyterev`]
///module
#[doc(alias = "DATABYTEREV")]
pub type Databyterev = crate::Reg<databyterev::DATABYTEREVrs>;
///CRC Data Byte Reverse Register
pub mod databyterev;
