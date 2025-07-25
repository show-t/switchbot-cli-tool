pub mod domain {
    pub mod repositories {
        mod device_repository;
        pub use device_repository::DeviceRepository;
    } 

    pub mod models {
        mod device;
        pub use device::Device;

        pub mod value_objects {
            mod device_id;
            pub use device_id::DeviceId;

            mod command;
            pub use command::Command;
        }
    }
}

pub mod infrastructure {
    pub mod api {
        mod switchbot_api;
        pub use switchbot_api::SwitchBotApi;
    }
    
    pub mod io {
        mod device_file_writer;
        pub use device_file_writer::DeviceFileWriter;
    }
}

pub mod application {
    mod control_device;
    pub use control_device::ControlDeviceUseCase;

    pub mod export_devices;
    pub use export_devices::export_devices_to_file;
}

pub mod presentation {
    pub mod cli {
        mod args;
        pub use args::CliArgs;
        pub use args::{
            Cli,
            Commands
        };
    }
}
