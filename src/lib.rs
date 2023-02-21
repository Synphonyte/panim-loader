pub mod errors;
mod parser;

/// A Properties Animation file containing all the animations for all exported properties of all objects in the scene.
#[derive(Clone, Debug, PartialEq)]
pub struct PropertiesAnimation {
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
}

impl<'a> From<parser::PropsAnimation<'a>> for PropertiesAnimation {
    fn from(props_animation: parser::PropsAnimation<'a>) -> Self {
        PropertiesAnimation {
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
    use super::*;

    #[test]
    fn it_works() {
        let result = PropertiesAnimation::from_file("assets/single_anim.panim").unwrap();
        assert_eq!(
            result,
            PropertiesAnimation {
                fps: 20.0,
                animations: vec![Animation {
                    object_name: "Orange Side Streaks".to_string(),
                    property_name: "opacity".to_string(),
                    frame_start: 549,
                    frame_end: 557,
                    frame_values: vec![
                        0.0, 0.04296834, 0.1562492, 0.31640565, 0.5, 0.6835944, 0.8437508,
                        0.95703185, 1.0,
                    ]
                }],
            }
        );
    }
}
