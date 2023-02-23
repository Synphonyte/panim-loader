//! This is a loader for the PANIM format, which is a simple animation format for custom properties in Blender.
//! This is meant to be used in conjunction with the [PANIM Blender Exporter](https://github.com/Synphonyte/blender-panim-exporter).
//!
//! Please also check there to see the details of the binary file format.
//!
//! The primary use case for this file type is to export more animation data on top of what can be stored in GLTF files.
//!
//! # Usage
//!
//! ```
//! use panim_loader::PropertiesAnimation;
//!
//! let anims = PropertiesAnimation::from_file("assets/single_anim.panim").unwrap();
//! let value = anims.get_animation_value_at_time(&anims.animations[0], 10.0);
//! ```

pub mod errors;
mod parser;

/// A Properties Animation file containing all the animations for all exported properties of all objects in the scene.
#[derive(Clone, Debug, PartialEq)]
pub struct PropertiesAnimation {
    /// Version of the file format as a tuple of semver (major, minor, patch).
    pub version: (u16, u16, u16),

    /// The number of frames per second.
    pub fps: f32,

    /// The list of animations.
    pub animations: Vec<Animation>,
}

/// An animation for a single property of a single object.
#[derive(Clone, Debug, PartialEq)]
pub struct Animation {
    /// The unique name of the object.
    pub object_name: String,

    /// The name of the property.
    pub property_name: String,

    /// The first frame of the animation.
    pub frame_start: u32,

    /// The last frame of the animation (including).
    pub frame_end: u32,

    /// The list of values for each frame starting with `frame_start` and ending with `frame_end`.
    pub frame_values: Vec<f32>,
}

/// The result type for parsing a Properties Animation file.
type Result = std::result::Result<PropertiesAnimation, errors::Error>;

impl PropertiesAnimation {
    /// Parses a Properties Animation file from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result {
        let props_animation = parser::props_animation(bytes)?;
        Ok(props_animation.into())
    }

    /// Parses a Properties Animation file from a file path.
    pub fn from_file(path: &str) -> Result {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// Returns the value of the animation at the given time (in seconds).
    pub fn get_animation_value_at_time(&self, animation: &Animation, elapsed_time: f32) -> f32 {
        animation.get_interpolated_value_at_frame(self.fps * elapsed_time)
    }
}

impl Animation {
    /// Returns the value of the animation at the given frame.
    /// If a frame is before the start of the animation, the first value is returned.
    /// If a frame is after the end of the animation, the last value is returned.
    pub fn get_value_at_exact_frame(&self, frame: u32) -> f32 {
        if frame <= self.frame_start {
            return self.frame_values[0];
        } else if frame >= self.frame_end {
            return self.frame_values[self.frame_values.len() - 1];
        }

        let index = (frame - self.frame_start) as usize;
        self.frame_values[index]
    }

    /// Returns the value of the animation at the given frame.
    /// If a frame is before the start of the animation, the first value is returned.
    /// If a frame is after the end of the animation, the last value is returned.
    /// If a frame is between two frames (i.e. is not an integer), the value is linearly interpolated.
    pub fn get_interpolated_value_at_frame(&self, frame: f32) -> f32 {
        let lower_frame = frame.floor();

        let fraction = frame - lower_frame;

        let lower_frame = lower_frame as u32;
        let upper_frame = lower_frame + 1;

        let lower_value = self.get_value_at_exact_frame(lower_frame);
        let upper_value = self.get_value_at_exact_frame(upper_frame);

        lower_value + (upper_value - lower_value) * fraction
    }
}

impl<'a> From<parser::PropsAnimation<'a>> for PropertiesAnimation {
    fn from(props_animation: parser::PropsAnimation<'a>) -> Self {
        PropertiesAnimation {
            version: props_animation.semver(),
            fps: props_animation.fps,
            animations: props_animation
                .animations
                .into_iter()
                .map(|animation| animation.into())
                .collect(),
        }
    }
}

impl<'a> From<parser::Animation<'a>> for Animation {
    fn from(animation: parser::Animation<'a>) -> Self {
        Animation {
            object_name: animation.header.object_name.to_string(),
            property_name: animation.header.property_name.to_string(),
            frame_start: animation.header.frame_start,
            frame_end: animation.header.frame_end,
            frame_values: animation.values.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::{anim, anim_header, anim_values, props_anim};

    #[test]
    fn it_works() {
        let result = crate::PropertiesAnimation::from_file("assets/single_anim.panim").unwrap();
        assert_eq!(result, props_anim!().into());

        let animation = &result.animations[0];
        assert_eq!(animation.get_value_at_exact_frame(10), 0.0);
        assert_eq!(animation.get_value_at_exact_frame(200), 1.0);
        assert_eq!(animation.get_value_at_exact_frame(90), 0.5);
        assert_eq!(animation.get_interpolated_value_at_frame(85.5), 0.18612504);

        assert_eq!(
            result.get_animation_value_at_time(animation, 85.4 / 24.0),
            0.18015012
        );
    }
}
