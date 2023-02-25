use nom::bytes::streaming::{take, take_till};
use nom::combinator::{eof, map, map_res};
use nom::error::Error;
use nom::multi::{count, many_till};
use nom::number::streaming::{le_f32, le_u32, le_u8};
use nom::sequence::{terminated, tuple};
use nom::HexDisplay;
use nom::{Finish, IResult};
use std::str::from_utf8;

#[derive(Clone, Debug, PartialEq)]
pub struct PropsAnimation<'a> {
    pub version: u32,
    pub fps: f32,
    pub animations: Vec<Animation<'a>>,
}

impl PropsAnimation<'_> {
    pub fn semver(&self) -> (u16, u16, u16) {
        let major = (self.version >> 20) as u16;
        let minor = ((self.version >> 10) & 0x3FF) as u16;
        let patch = (self.version & 0x3FF) as u16;
        (major, minor, patch)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Animation<'a> {
    pub header: AnimationHeader<'a>,
    pub values: AnimationValues,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationHeader<'a> {
    pub object_name: &'a str,
    pub property_name: &'a str,
    pub frame_start: u32,
    pub frame_end: u32,
    pub typ: u8, // TODO : enum when used
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationValues(pub Vec<f32>);

pub fn props_animation(input: &[u8]) -> Result<PropsAnimation, Error<String>> {
    map(
        tuple((le_u32, le_f32, many_till(animation, eof))),
        |(version, fps, (animations, _))| PropsAnimation {
            version,
            fps,
            animations,
        },
    )(input)
    .finish()
    .map(|(_, props_animation)| props_animation)
    .map_err(|e| Error::new(e.input.to_hex(2), e.code))
}

pub fn animation(input: &[u8]) -> IResult<&[u8], Animation> {
    animation_header(input).and_then(|(input, header)| {
        animation_values(&header, input)
            .map(|(input, values)| (input, Animation { header, values }))
    })
}

pub fn animation_header(input: &[u8]) -> IResult<&[u8], AnimationHeader> {
    map(
        tuple((
            zero_term_str,
            zero_term_str,
            le_u32,
            le_u32,
            le_u8,
            take(32_usize),
        )),
        |(object_name, property_name, frame_start, frame_end, typ, _)| AnimationHeader {
            object_name,
            property_name,
            frame_start,
            frame_end,
            typ,
        },
    )(input)
}

pub fn animation_values<'a>(
    header: &AnimationHeader,
    input: &'a [u8],
) -> IResult<&'a [u8], AnimationValues> {
    map(
        count(le_f32, (header.frame_end - header.frame_start + 1) as usize),
        |values| AnimationValues(values),
    )(input)
}

pub fn zero_term_str(input: &[u8]) -> IResult<&[u8], &str> {
    terminated(map_res(take_till(|c| c == 0), from_utf8), le_u8)(input)
}

#[allow(non_upper_case_globals)]
#[cfg(test)]
pub(crate) mod tests {
    const single_anim: &[u8] = include_bytes!("../assets/single_anim.panim");
    const multi_anim: &[u8] = include_bytes!("../assets/multi_anim.panim");

    macro_rules! first_anim_values {
        () => {
            crate::parser::AnimationValues(vec![
                0.0,
                0.007250005,
                0.028000012,
                0.060750026,
                0.10400003,
                0.15625003,
                0.21600005,
                0.28175005,
                0.352,
                0.42525005,
                0.5,
                0.57475,
                0.648,
                0.71825,
                0.7839999,
                0.84374994,
                0.896,
                0.9392501,
                0.972,
                0.99275005,
                1.0,
            ])
        };
    }

    macro_rules! first_anim_header {
        () => {
            crate::parser::AnimationHeader {
                object_name: "Cube",
                property_name: "opacity",
                frame_start: 80,
                frame_end: 100,
                typ: 0,
            }
        };
    }

    macro_rules! first_anim {
        () => {
            crate::parser::Animation {
                header: first_anim_header!(),
                values: first_anim_values!(),
            }
        };
    }

    macro_rules! single_props_anim {
        () => {
            crate::parser::PropsAnimation {
                version: 2048,
                fps: 24.0,
                animations: vec![first_anim!()],
            }
        };
    }

    pub(crate) use first_anim;
    pub(crate) use first_anim_header;
    pub(crate) use first_anim_values;
    pub(crate) use single_props_anim;

    #[test]
    fn test_multi_props_animation() {
        let output = crate::parser::props_animation(multi_anim).unwrap();
        assert_eq!(
            output,
            crate::parser::PropsAnimation {
                version: 2048,
                fps: 24.0,
                animations: vec![
                    first_anim!(),
                    crate::parser::Animation {
                        header: crate::parser::AnimationHeader {
                            object_name: "Cube",
                            property_name: "other",
                            frame_start: 100,
                            frame_end: 105,
                            typ: 0,
                        },
                        values: crate::parser::AnimationValues(vec![
                            1.0, 1.8320012, 3.8160005, 6.183999, 8.167998, 9.0
                        ])
                    },
                    crate::parser::Animation {
                        header: crate::parser::AnimationHeader {
                            object_name: "Empty",
                            property_name: "bla",
                            frame_start: 20,
                            frame_end: 30,
                            typ: 0,
                        },
                        values: crate::parser::AnimationValues(vec![
                            1.0,
                            0.972,
                            0.89599997,
                            0.78400004,
                            0.648,
                            0.5,
                            0.352,
                            0.21600008,
                            0.10399997,
                            0.027999878,
                            0.0
                        ]),
                    },
                ],
            }
        );
    }

    #[test]
    fn test_single_props_animation() {
        let output = crate::parser::props_animation(single_anim).unwrap();
        assert_eq!(output, single_props_anim!());
    }

    #[test]
    fn test_first_animation() {
        let input = &single_anim[8..];
        let (remainder, output) = crate::parser::animation(input).unwrap();
        assert_eq!(output, first_anim!());
        assert_eq!(remainder, &[]);
    }

    #[test]
    fn test_first_animation_header() {
        let input = &single_anim[8..62];
        let (remainder, output) = crate::parser::animation_header(input).unwrap();
        assert_eq!(output, first_anim_header!());
        assert_eq!(remainder, &[]);
    }

    #[test]
    fn test_first_animation_values() {
        let input = &single_anim[62..];
        let (remainder, output) =
            crate::parser::animation_values(&first_anim_header!(), input).unwrap();
        assert_eq!(output, first_anim_values!());
        assert_eq!(remainder, &[]);
    }

    #[test]
    fn test_semver() {
        let input = single_props_anim!();
        let output = input.semver();
        assert_eq!(output, (0, 2, 0));
    }

    #[test]
    fn test_zero_term_str() {
        let input = b"hello\0world\0";
        let (remainder, output) = crate::parser::zero_term_str(input).unwrap();
        assert_eq!(output, "hello");
        assert_eq!(remainder, b"world\0");
    }
}
