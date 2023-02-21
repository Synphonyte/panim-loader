pub mod errors;
mod parser;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertiesAnimation {
    pub fps: f32,
    pub animations: Vec<Animation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Animation {
    pub object_name: String,
    pub property_name: String,
    pub frame_start: u32,
    pub frame_end: u32,
    pub frame_values: Vec<f32>,
}

type Result = std::result::Result<PropertiesAnimation, errors::Error>;

impl PropertiesAnimation {
    pub fn from_bytes(bytes: &[u8]) -> Result {
        let props_animation = parser::props_animation(bytes)?;
        Ok(props_animation.into())
    }

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
