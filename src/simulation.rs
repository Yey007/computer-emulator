use crate::device::Device;

pub fn run_simulation(mut devices: Vec<Box<dyn Device>>) -> ! {
    loop {
        for device in devices.iter_mut() {
            device.tick()
        }
    }
}