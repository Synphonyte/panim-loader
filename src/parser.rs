use nom::bytes::streaming::take_till;
use nom::combinator::{eof, map, map_res};
use nom::error::Error;
use nom::multi::{count, many_till};
use nom::number::streaming::{le_f32, le_u32, le_u8};
use nom::sequence::{terminated, tuple};
use nom::{Finish, IResult};
use std::str::from_utf8;
use nom::HexDisplay;

#[derive(Clone, Debug, PartialEq)]
pub struct PropsAnimation<'a> {
    pub fps: f32,
    pub animations: Vec<Animation<'a>>,
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
        tuple((le_f32, many_till(animation, eof))),
        |(fps, (animations, _))| PropsAnimation { fps, animations },
    )(input)
    .finish()
    .map(|(_, props_animation)| props_animation)
    .map_err(|e| Error::new( e.input.to_hex(2), e.code))
}

pub fn animation(input: &[u8]) -> IResult<&[u8], Animation> {
    animation_header(input).and_then(|(input, header)| {
        animation_values(&header, input)
            .map(|(input, values)| (input, Animation { header, values }))
    })
}

pub fn animation_header(input: &[u8]) -> IResult<&[u8], AnimationHeader> {
    map(
        tuple((zero_term_str, zero_term_str, le_u32, le_u32, le_u8)),
        |(object_name, property_name, frame_start, frame_end, typ)| AnimationHeader {
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
mod tests {
    use super::*;

    const single_anim: &[u8] = include_bytes!("../assets/single_anim.panim");

    macro_rules! anim_values {
        () => {
            AnimationValues(vec![
                0.0, 0.04296834, 0.1562492, 0.31640565, 0.5, 0.6835944, 0.8437508, 0.95703185, 1.0,
            ])
        };
    }

    macro_rules! anim_header {
        () => {
            AnimationHeader {
                object_name: "Orange Side Streaks",
                property_name: "opacity",
                frame_start: 549,
                frame_end: 557,
                typ: 0,
            }
        };
    }

    macro_rules! anim {
        () => {
            Animation {
                header: anim_header!(),
                values: anim_values!(),
            }
        };
    }

    macro_rules! props_anim {
        () => {
            PropsAnimation {
                fps: 20.0,
                animations: vec![anim!()],
            }
        };
    }

    #[test]
    fn test_props_animation() {
        let output = props_animation(single_anim).unwrap();
        assert_eq!(output, props_anim!());
    }

    #[test]
    fn test_animation() {
        let input = &single_anim[4..];
        let (remainder, output) = animation(input).unwrap();
        assert_eq!(output, anim!());
        assert_eq!(remainder, &[]);
    }

    #[test]
    fn test_animation_header() {
        let input = &single_anim[4..41];
        let (remainder, output) = animation_header(input).unwrap();
        assert_eq!(output, anim_header!());
        assert_eq!(remainder, &[]);
    }

    #[test]
    fn test_animation_values() {
        let input = &single_anim[41..];
        let (remainder, output) = animation_values(&anim_header!(), input).unwrap();
        assert_eq!(output, anim_values!());
        assert_eq!(remainder, &[]);
    }

    #[test]
    fn test_zero_term_str() {
        let input = b"hello\0world\0";
        let (remainder, output) = zero_term_str(input).unwrap();
        assert_eq!(output, "hello");
        assert_eq!(remainder, b"world\0");
    }
}
