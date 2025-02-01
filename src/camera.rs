use crate::{platform::impl_camera::ImplCamera, XCapResult};

#[derive(Debug, Clone)]
pub struct Camera {
    pub(crate) impl_camera: ImplCamera,
}

impl Camera {
    pub(crate) fn new(impl_camera: ImplCamera) -> Camera {
        Camera { impl_camera }
    }
}

impl Camera {
    pub fn all() -> XCapResult<Vec<Camera>> {
        let cameras = ImplCamera::all()
            .unwrap_or(Vec::new())
            .iter()
            .map(|impl_camera| Camera::new(impl_camera.clone()))
            .collect();

        Ok(cameras)
    }
}

impl Camera {
    pub fn id(&self) -> String {
        self.impl_camera.device_id.clone()
    }

    pub  fn name(&self) -> String {
        self.impl_camera.name.clone()
    }

    pub fn can_use_flash(&self) -> bool {
        self.impl_camera.can_use_flash
    }

    pub fn device_type(&self) -> String {
        self.impl_camera.device_type.clone()
    }
}
