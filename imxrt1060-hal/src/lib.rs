//! NXP iMXRT1060 hardware abstraction layer
//!
//! An [`embedded-hal`](https://crates.io/crates/embedded-hal) implementation
//! targeting processors in NXP's IMXRT106x family.
//!
//! The HAL is a WIP. More documentation will become available once more capabilities
//! are exposed.
//!
//! In some cases, the HAL simply re-exports peripherals from the peripheral access
//! crates (PAC). If they are not re-exported, all PAC components are available
//! in the `pac` module.

#![no_std]

pub use imxrt1060_pac as pac;

pub mod ccm;
mod clockspeed;
pub mod gpio;
pub mod iomuxc;
pub mod pit;

pub use clockspeed::set_arm_clock;

pub mod dcdc {
    use imxrt1060_pac as pac;
    pub struct DCDC(pub(crate) pac::DCDC);
    impl DCDC {
        pub fn raw(&self) -> &pac::DCDC {
            &self.0
        }
    }
}

pub struct Peripherals {
    pub iomuxc: iomuxc::IOMUXC,
    pub systick: pac::SYST,
    pub ccm: ccm::CCM,
    pub pit: pit::UnclockedPIT,
    pub dcdc: dcdc::DCDC,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        let cp = cortex_m::Peripherals::take()?;
        let p = pac::Peripherals::take()?;
        Some(Peripherals::new(p, cp))
    }

    fn new(p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {
        Peripherals {
            iomuxc: iomuxc::IOMUXC::new(p.IOMUXC),
            systick: cp.SYST,
            ccm: ccm::CCM::new(p.CCM, p.CCM_ANALOG),
            pit: pit::UnclockedPIT::new(p.PIT),
            dcdc: dcdc::DCDC(p.DCDC),
        }
    }
}
