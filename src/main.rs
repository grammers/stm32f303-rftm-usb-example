#![no_main]
#![no_std]

extern crate stm32f3xx_hal as hal;
extern crate panic_semihosting;
extern crate cortex_m_semihosting;

use cortex_m_semihosting::hprintln;
use cortex_m::asm::delay;
use rtfm::app;

use hal::{prelude::*, stm32, hal::digital::v2::OutputPin, gpio::{Output, PushPull}};

use stm32_usbd::{UsbBus, UsbBusType};
use usb_device::bus;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

fn configure_usb_clock() {
    let rcc = unsafe { &*stm32::RCC::ptr() };
    rcc.cfgr.modify(|_, w| w.usbpre().set_bit());
}

#[app(device = stm32f3xx_hal::stm32)]
const APP: () = {

	static mut LED_S: hal::gpio::gpioe::PE13<Output<PushPull>> = ();
    static mut LED_COUNT: usize = 8;
    static mut USB_DEV: UsbDevice<'static, UsbBusType> = ();
    static mut SERIAL: SerialPort<'static, UsbBusType> = ();

	#[init]
	fn init() {
  
  		static mut USB_BUS: Option<bus::UsbBusAllocator<UsbBusType>> = None;

        // Cortex-M peripherals
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let clocks = rcc
			.cfgr
            .sysclk(48.mhz())
            .pclk1(24.mhz())
			.pclk2(24.mhz())
            .freeze(&mut flash.acr);

        // Init USB serial
        hprintln!("Initialising USB serial device").unwrap();

		// LEDs
		let mut gpioe = device.GPIOE.split(&mut rcc.ahb);
		let led_s = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);


		let mut gpioa = device.GPIOA.split(&mut rcc.ahb);
        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
		let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
		usb_dp.set_low().expect("could not set pin for synk");
		delay(clocks.sysclk().0 / 100);

		let usb_dm = gpioa.pa11.into_af14(&mut gpioa.moder, &mut gpioa.afrh);
		let usb_dp = usb_dp.into_af14(&mut gpioa.moder, &mut gpioa.afrh);

		configure_usb_clock();

        *USB_BUS = Some(UsbBus::new(device.USB, (usb_dm, usb_dp)));
        let serial = SerialPort::new(USB_BUS.as_ref().unwrap());
       let usb_dev =
            UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build();

		LED_S = led_s;
		USB_DEV = usb_dev;
		SERIAL = serial;

	}

    #[interrupt(resources = [USB_DEV, SERIAL])]
    fn USB_HP_CAN_TX() {
//        hprintln!("DB usb_hp_can_tx").unwrap();
        usb_poll(&mut resources.USB_DEV, &mut resources.SERIAL);
    }

    #[interrupt(resources = [USB_DEV, SERIAL, LED_S])]
    fn USB_LP_CAN_RX0() {
		resources.LED_S.set_high().expect("expected LED to blink");
        usb_poll(&mut resources.USB_DEV, &mut resources.SERIAL);
		//hprintln!("DB usb_lp_can_rx0").unwrap();
		resources.LED_S.set_low().expect("expected LED to blink");
    }
};

fn usb_poll<B: bus::UsbBus>(
    usb_dev: &mut UsbDevice<'static, B>,
    serial: &mut SerialPort<'static, B>,
) -> bool {
    if !usb_dev.poll(&mut [serial]) {
        return false;
    }

    let mut buf = [0u8; 64];

    match serial.read(&mut buf) {
        Ok(count) if count > 0 => {
            // Echo back in upper case
			hprintln!("ok").unwrap();
            for c in buf[0..count].iter_mut() {
                if 0x61 <= *c && *c <= 0x7a {
                    *c &= !0x20;
                }
            }

            serial.write(&buf[0..count]).ok();
        }
        _ => {}
    }
	return true;
}
