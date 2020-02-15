#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `APORTCONFLICT`"]
pub type APORTCONFLICT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
