///Register `CH2_LINK` reader
pub type R = crate::R<CH2_LINKrs>;
///Register `CH2_LINK` writer
pub type W = crate::W<CH2_LINKrs>;
///Field `LINKMODE` reader - Link Structure Addressing Mode
pub type LinkmodeR = crate::BitReader;
///Field `LINK` reader - Link Next Structure
pub type LinkR = crate::BitReader;
///Field `LINK` writer - Link Next Structure
pub type LinkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINKADDR` reader - Link Structure Address
pub type LinkaddrR = crate::FieldReader<u32>;
///Field `LINKADDR` writer - Link Structure Address
pub type LinkaddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - Link Structure Addressing Mode
    #[inline(always)]
    pub fn linkmode(&self) -> LinkmodeR {
        LinkmodeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Link Next Structure
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31 - Link Structure Address
    #[inline(always)]
    pub fn linkaddr(&self) -> LinkaddrR {
        LinkaddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_LINK")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
impl W {
    ///Bit 1 - Link Next Structure
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LinkW<CH2_LINKrs> {
        LinkW::new(self, 1)
    }
    ///Bits 2:31 - Link Structure Address
    #[inline(always)]
    #[must_use]
    pub fn linkaddr(&mut self) -> LinkaddrW<CH2_LINKrs> {
        LinkaddrW::new(self, 2)
    }
}
///Channel Descriptor Link Structure Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch2_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH2_LINKrs;
impl crate::RegisterSpec for CH2_LINKrs {
    type Ux = u32;
}
///`read()` method returns [`ch2_link::R`](R) reader structure
impl crate::Readable for CH2_LINKrs {}
///`write(|w| ..)` method takes [`ch2_link::W`](W) writer structure
impl crate::Writable for CH2_LINKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH2_LINK to value 0
impl crate::Resettable for CH2_LINKrs {
    const RESET_VALUE: u32 = 0;
}
