use crate::device::Device;

pub fn run_simulation(mut devices: Vec<Box<dyn Device>>, ticks: Option<u32>) {
    let mut tick: u32 = 0;
    let ticks = ticks.unwrap_or(u32::MAX);
    
    loop {
        if tick >= ticks {
            break;
        } 
        
        for device in devices.iter_mut() {
            device.tick(tick)
        }
        tick += 1;
    }
}