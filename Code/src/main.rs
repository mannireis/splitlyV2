#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals};
use rmk::nrf52840_ble::initialize_nrf52840_ble_keyboard_and_run;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBD => embassy_nrf::usb::vbus_detect::InterruptHandler;
    UARTE0_UART0 => embassy_nrf::uart::InterruptHandler<peripherals::UARTE0>;
    SAADC => embassy_nrf::saadc::InterruptHandler;
    TIMER0 => embassy_nrf::timer::InterruptHandler<peripherals::TIMER0>;
    TIMER1 => embassy_nrf::timer::InterruptHandler<peripherals::TIMER1>;
    TIMER2 => embassy_nrf::timer::InterruptHandler<peripherals::TIMER2>;
    RTC0 => embassy_nrf::rtc::InterruptHandler<peripherals::RTC0>;
    RTC1 => embassy_nrf::rtc::InterruptHandler<peripherals::RTC1>;
    GPIOTE => embassy_nrf::gpiote::InterruptHandler;
    RADIO => embassy_nrf::radio::InterruptHandler;
    TEMP => embassy_nrf::temp::InterruptHandler;
    RNG => embassy_nrf::rng::InterruptHandler;
    TIMER3 => embassy_nrf::timer::InterruptHandler<peripherals::TIMER3>;
    TIMER4 => embassy_nrf::timer::InterruptHandler<peripherals::TIMER4>;
    ECB => embassy_nrf::ecb::InterruptHandler;
    CCM_AAR => embassy_nrf::ccm::InterruptHandler;
    WDT => embassy_nrf::wdt::InterruptHandler;
    QDEC => embassy_nrf::qdec::InterruptHandler;
    COMP_LPCOMP => embassy_nrf::comp::InterruptHandler;
    SWI0_EGU0 => embassy_nrf::swi::InterruptHandler<peripherals::SWI0_EGU0>;
    SWI1_EGU1 => embassy_nrf::swi::InterruptHandler<peripherals::SWI1_EGU1>;
    SWI2_EGU2 => embassy_nrf::swi::InterruptHandler<peripherals::SWI2_EGU2>;
    SWI3_EGU3 => embassy_nrf::swi::InterruptHandler<peripherals::SWI3_EGU3>;
    SWI4_EGU4 => embassy_nrf::swi::InterruptHandler<peripherals::SWI4_EGU4>;
    SWI5_EGU5 => embassy_nrf::swi::InterruptHandler<peripherals::SWI5_EGU5>;
    PWM0 => embassy_nrf::pwm::InterruptHandler<peripherals::PWM0>;
    PWM1 => embassy_nrf::pwm::InterruptHandler<peripherals::PWM1>;
    PWM2 => embassy_nrf::pwm::InterruptHandler<peripherals::PWM2>;
    PWM3 => embassy_nrf::pwm::InterruptHandler<peripherals::PWM3>;
    PDM => embassy_nrf::pdm::InterruptHandler;
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => embassy_nrf::spim::InterruptHandler<peripherals::TWISPI0>;
    SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 => embassy_nrf::spim::InterruptHandler<peripherals::TWISPI1>;
    SPIM2_SPIS2_SPI2 => embassy_nrf::spim::InterruptHandler<peripherals::SPI2>;
    SPIM3 => embassy_nrf::spim::InterruptHandler<peripherals::SPI3>;
    NFCT => embassy_nrf::nfct::InterruptHandler;
    I2S => embassy_nrf::i2s::InterruptHandler<peripherals::I2S>;
    IPC => embassy_nrf::ipc::InterruptHandler;
    QSPI => embassy_nrf::qspi::InterruptHandler<peripherals::QSPI>;
    CRYPTOCELL => embassy_nrf::cryptocell::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting Splitly V2 keyboard");

    let p = embassy_nrf::init(Default::default());

    // Initialize RMK with keyboard configuration
    initialize_nrf52840_ble_keyboard_and_run(
        include_str!("../keyboard.toml"),
        p,
    ).await;
}
