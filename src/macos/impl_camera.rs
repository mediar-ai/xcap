use objc2::rc::{autoreleasepool, Retained};
use objc2_av_foundation::{
    AVCaptureDevice, AVCaptureDeviceDiscoverySession, AVCaptureDevicePosition,
    AVCaptureDeviceTypeBuiltInWideAngleCamera,
};
use objc2_foundation::{NSArray, NSString};

#[derive(Debug, Clone)]
pub(crate) struct ImplCamera {
    pub device_id: String,
    pub name: String,
    pub device_type: String,
    device_position: Option<AVCaptureDevicePosition>,
    pub can_use_flash: bool,
}

impl ImplCamera {
    pub fn all() -> Result<Vec<ImplCamera>, ()> {
        let mut camera_list: Vec<ImplCamera> = Vec::new();
        autoreleasepool(|_| unsafe {
            let device_types = NSArray::<NSString>::from_slice(
                vec![AVCaptureDeviceTypeBuiltInWideAngleCamera].as_slice(),
            );
            let session =
                AVCaptureDeviceDiscoverySession::discoverySessionWithDeviceTypes_mediaType_position(
                    &device_types,
                    None,
                    AVCaptureDevicePosition::Unspecified,
                );
            let devices = session.devices().to_vec();
            for device in devices {
                camera_list.push(ImplCamera::new(device));
            }
        });
        Ok(camera_list)
    }

    pub fn new(device: Retained<AVCaptureDevice>) -> ImplCamera {
        unsafe {
            let id = device.uniqueID().to_string();
            let name = device.localizedName().to_string();
            let device_type = device.deviceType().to_string();
            let position = device.position().clone();
            let flash = device.isFlashAvailable();

            ImplCamera {
                can_use_flash: flash,
                device_position: Some(position),
                name,
                device_id: id,
                device_type,
            }
        }
    }
}
