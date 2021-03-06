use {VRDisplayData, VRFramebuffer, VRFramebufferAttributes, VRFrameData, VRFutureFrameData, VRGamepadPtr, VRLayer};
use sparkle::gl::Gl;
use std::sync::Arc;
use std::cell::RefCell;
pub type VRDisplayPtr = Arc<RefCell<dyn VRDisplay>>;

/// The VRDisplay traits forms the base of all VR device implementations
pub trait VRDisplay: Send + Sync {

    /// Returns unique device identifier
    fn id(&self) -> u32;

    /// Returns the current display data.
    fn data(&self) -> VRDisplayData;

    /// Returns gamepads attached to this display
    fn fetch_gamepads(&mut self) -> Result<Vec<VRGamepadPtr>, String>;

    /// Returns the immediate VRFrameData of the HMD
    /// Should be used when not presenting to the device.
    fn immediate_frame_data(&self, near_z: f64, far_z: f64) -> VRFrameData;

    /// Returns the synced VRFrameData to render the next frame.
    /// The future that is returned will resolve with frame data when the
    /// next frame is available.
    #[allow(deprecated)]
    fn future_frame_data(&mut self, near_z: f64, far_z: f64) -> VRFutureFrameData {
        // The default implementation blocks waiting for the display.
        self.sync_poses();
        VRFutureFrameData::resolved(self.synced_frame_data(near_z, far_z))
    }

    /// Returns the synced VRFrameData to render the current frame.
    /// Should be used when presenting to the device.
    /// sync_poses must have been called before this call.
    #[deprecated(since="0.10.3", note="please use `future_frame_data` instead")]
    fn synced_frame_data(&self, next: f64, far_z: f64) -> VRFrameData;

    /// Resets the pose for this display
    fn reset_pose(&mut self);

    /// Synchronization point to keep in step with the HMD
    /// Returns VRFrameData to be used in the next render frame
    /// Must be called in the render thread, before doing any work
    #[deprecated(since="0.10.3", note="please use `future_frame_data` instead")]
    fn sync_poses(&mut self);

    /// Binds the framebuffer to directly render to the HDM
    /// Must be called in the render thread, before doing any work
    fn bind_framebuffer(&mut self, index: u32);

    /// Returns the available FBOs that must be used to render to all eyes
    /// Must be called in the render thread, before doing any work
    fn get_framebuffers(&self) -> Vec<VRFramebuffer>;

    /// Renders a VRLayer from a external texture
    /// Must be called in the render thread
    #[deprecated(since="0.10.3", note="please use `submit_layer` instead")]
    fn render_layer(&mut self, layer: &VRLayer);

    /// Submits frame to the display
    /// Must be called in the render thread
    #[deprecated(since="0.10.3", note="please use `submit_layer` instead")]
    fn submit_frame(&mut self);

    /// Renders a VRLayer from an external texture, and submits it to the device.
    /// Must be called in the render thread
    #[allow(unused_variables)]
    #[allow(deprecated)]
    fn submit_layer(&mut self, gl: &Gl, layer: &VRLayer) {
        self.render_layer(layer);
        self.submit_frame();
    }

    /// Hint to indicate that we are going to start sending frames to the device
    fn start_present(&mut self, _attributes: Option<VRFramebufferAttributes>) {}

    /// Hint to indicate that we are going to stop sending frames to the device
    fn stop_present(&mut self) {}
}

impl PartialEq for dyn VRDisplay {
    fn eq(&self, other: &dyn VRDisplay) -> bool {
        self.id() == other.id()
    }
}
