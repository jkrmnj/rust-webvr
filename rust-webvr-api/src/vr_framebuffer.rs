// Information about a FBO provided by a VRDisplay.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-serialization", derive(Deserialize, Serialize))]
pub struct VRFramebuffer {
    // Framebuffer identifier
    pub id: u32,

    // The attributes set up for this framebuffer
    pub attributes: VRFramebufferAttributes,

    // UVs defining the texture bounds to present to the eye in UV space: [x,y,w,h]
    // Defaults to [0.0, 0.0, 0.5, 1.0]
    pub viewport: VRViewport,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde-serialization", derive(Deserialize, Serialize))]
pub struct VRFramebufferAttributes {
    pub multiview: bool,
    pub depth: bool,
    pub multisampling: bool,
}

impl Default for VRFramebufferAttributes {
     fn default() -> VRFramebufferAttributes {
         Self {
            multiview: false,
            depth: false,
            multisampling: false,
         }
     }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-serialization", derive(Deserialize, Serialize))]
pub struct VRViewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl VRViewport {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}
