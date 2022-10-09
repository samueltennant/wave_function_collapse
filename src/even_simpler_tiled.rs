use image::{ImageError, Rgba};

#[derive(Debug, Eq, PartialEq)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    direction: Dir,
    this: Rgba<u8>,
    other: Rgba<u8>,
}

impl Rule {
    fn new(dir: Dir, this: &Rgba<u8>, other: &Rgba<u8>) -> Rule {
        Rule {
            direction: dir,
            this: *this,
            other: *other,
        }
    }
}

pub fn generate() -> Result<(), ImageError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use image::Pixel;

    use super::*;

    #[test]
    fn test_create_rule() {
        let this = Rgba::from_slice(&[0u8, 0u8, 0u8, 0u8]);
        let other = Rgba::from_slice(&[255u8, 0u8, 0u8, 0u8]);

        let rule = Rule::new(Dir::Up, &this.clone(), &other.clone());
        assert_eq!(
            rule,
            Rule {
                direction: Dir::Up,
                this: this.clone(),
                other: other.clone()
            }
        );
    }
}
