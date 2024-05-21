use esp_idf_svc::hal::{gpio::PinDriver, peripherals::Peripherals};
use esp_idf_sys::esp_deep_sleep;
use log::info;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("initializing camera and flash");
    let peripherals = Peripherals::take().unwrap();

    // ESP32-CAM has an onboard Flash LED addressed at GPIO-4
    let mut flash = PinDriver::output(peripherals.pins.gpio4)?;
    flash.set_low()?;

    // Initialize the camera
    let camera = esp_camera_rs::Camera::new(
        peripherals.pins.gpio32,
        peripherals.pins.gpio0,
        peripherals.pins.gpio5,
        peripherals.pins.gpio18,
        peripherals.pins.gpio19,
        peripherals.pins.gpio21,
        peripherals.pins.gpio36,
        peripherals.pins.gpio39,
        peripherals.pins.gpio34,
        peripherals.pins.gpio35,
        peripherals.pins.gpio25,
        peripherals.pins.gpio23,
        peripherals.pins.gpio22,
        peripherals.pins.gpio26,
        peripherals.pins.gpio27,
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_UXGA,
        esp_idf_sys::camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
    )?;

    info!("initialization complete!");

    // Turn on the flash and take a picture.
    // You probably want to keep it on for a while and adjust
    // the camera exposure to get a good image, but this is
    // mainly here to show that the program is working.
    info!("taking a picture");
    flash.set_high()?;
    let framebuffer = camera.get_framebuffer();
    flash.set_low()?;

    if let Some(framebuffer) = framebuffer {
        info!(
            "took picture: {width}x{height} {size} bytes",
            width = framebuffer.width(),
            height = framebuffer.height(),
            size = framebuffer.data().len(),
        );

        // TODO: Do something with the framebuffer.
        // the JPEG-encoded byte data is in framebuffer.data()
        // so you can just dump this to a file or send it over
        // the network as "image.jpeg" and it should work.
    } else {
        panic!("failed to take image");
    };

    // Send the board into deep sleep. This will basically turn off everything
    // and consume very little power, and then "reboot" the device, restarting
    // the pin initialization and taking another picture.
    let delay = Duration::from_secs(60);
    info!("finished, entering deep sleep for {delay:#?}");
    unsafe { esp_deep_sleep(delay.as_micros() as u64) }
}
