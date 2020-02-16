#[derive(PartialEq, Debug)]
pub enum Note {
    A,
    As,
    B,
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
}
impl Note {
    pub fn from_str(name: &str) -> Result<Note, &'static str> {
        match name {
            "A" => Ok(Note::A),
            "A#" => Ok(Note::As),
            "B" => Ok(Note::B),
            "C" => Ok(Note::C),
            "C#" => Ok(Note::Cs),
            "D" => Ok(Note::D),
            "D#" => Ok(Note::Ds),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "F#" => Ok(Note::Fs),
            "G" => Ok(Note::G),
            "G#" => Ok(Note::Gs),
            _ => Err("String couldn't be parsed"),
        }
    }
    pub fn half_step_up(&self) -> Note {
        match self {
            Note::A => Note::As,
            Note::As => Note::B,
            Note::B => Note::C,
            Note::C => Note::Cs,
            Note::Cs => Note::D,
            Note::D => Note::Ds,
            Note::Ds => Note::E,
            Note::E => Note::F,
            Note::F => Note::Fs,
            Note::Fs => Note::G,
            Note::G => Note::Gs,
            Note::Gs => Note::A,
        }
    }

    pub fn n_half_steps_up(&self, n: u32) -> Note {}

    pub fn half_step_down(&self) -> Note {}
}

#[cfg(test)]
mod tests {
    use crate::Note;
    #[test]
    fn test_from_str() {
        assert_eq!(Note::A, Note::from_str("A").unwrap());
        assert_eq!(Note::As, Note::from_str("A#").unwrap());
        assert_eq!(Note::B, Note::from_str("B").unwrap());
        assert_eq!(Note::C, Note::from_str("C").unwrap());
        assert_eq!(Note::Cs, Note::from_str("C#").unwrap());
        assert_eq!(Note::D, Note::from_str("D").unwrap());
        assert_eq!(Note::Ds, Note::from_str("D#").unwrap());
        assert_eq!(Note::E, Note::from_str("E").unwrap());
        assert_eq!(Note::F, Note::from_str("F").unwrap());
        assert_eq!(Note::Fs, Note::from_str("F#").unwrap());
        assert_eq!(Note::G, Note::from_str("G").unwrap());
        assert_eq!(Note::Gs, Note::from_str("G#").unwrap());
    }
    #[test]
    fn test_half_step_up() {
        assert_eq!(Note::A.half_step_up(), Note::As);
        assert_eq!(Note::As.half_step_up(), Note::B);
        assert_eq!(Note::B.half_step_up(), Note::C);
        assert_eq!(Note::C.half_step_up(), Note::Cs);
        assert_eq!(Note::Cs.half_step_up(), Note::D);
        assert_eq!(Note::D.half_step_up(), Note::Ds);
        assert_eq!(Note::Ds.half_step_up(), Note::E);
        assert_eq!(Note::E.half_step_up(), Note::F);
        assert_eq!(Note::F.half_step_up(), Note::Fs);
        assert_eq!(Note::Fs.half_step_up(), Note::G);
        assert_eq!(Note::G.half_step_up(), Note::Gs);
        assert_eq!(Note::Gs.half_step_up(), Note::A);
    }
}
