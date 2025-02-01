#![allow(unused)]

#[derive(Debug, Clone)]
pub(crate) struct Camera {
    device_id: String,
    name: String,
    device_type: String,
    device_position: Option<AVCaptureDevicePosition>,
    can_use_flash: bool,
}

impl Camera {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn all() {
        unimplemented!()
    }
}
