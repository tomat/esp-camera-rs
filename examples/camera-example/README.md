This example assumes you have already set up all the tools necessary to compile Rust for ESP32.

When run using `cargo run`, it will compile and flash the program to an ESP32-CAM mounted on an
ESP32-CAM-MB board connected via USB to the host machine.

The program will:

    1. Initialize the onboard OV2640 camera as well as the bright white LED (flash).
    2. Turn on the flash.
    3. Take a jpeg-encoded picture using the camera.
    4. Print information about the image: width, height and size in bytes.
    5. Enter deep sleep for 60 seconds, after which the device starts the process.