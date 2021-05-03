extern crate daemonize;
extern crate btleplug;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use daemonize::Daemonize;

mod settings;

use btleplug::api::{Central, CentralEvent};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};

fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().unwrap();
    adapters.into_iter().nth(0).unwrap()
}

fn main() {
    let s = settings::Settings::new();
    match s {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
        _ => ()
    }
    let s = s.unwrap();

    let stdout = std::fs::File::create(format!("{}{}.out", s.daemon.workdir, s.daemon.name)).unwrap();
    let stderr = std::fs::File::create(format!("{}{}.err", s.daemon.workdir, s.daemon.name)).unwrap();

    if s.debug {
        println!("{:?}", s);
    }

    let user: &str = &s.daemon.user;
    let group: &str = &s.daemon.group;

    let daemonize = Daemonize::new()
        .pid_file(format!("{}{}.pid", s.daemon.workdir, s.daemon.name))
        .chown_pid_file(true)
        .working_directory(s.daemon.workdir)
        .user(user)
        .group(group)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            let manager = Manager::new().unwrap();
            let central = get_central(&manager);
            let event_receiver = central.event_receiver().unwrap();
            central.start_scan().unwrap();

            while let Ok(event) = event_receiver.recv() {
                match event {
                    CentralEvent::ManufacturerDataAdvertisement {
                        address,
                        manufacturer_id,
                        data,
                    } => {
                        println!(
                            "ManufacturerDataAdvertisement: {:?}, {:x}, {:?}",
                            address, manufacturer_id, data
                        );
                    }
                    _ => {}
                }
            }
        },
        Err(e) => println!("Error, {}", e),
    }
}
