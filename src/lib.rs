use std::fmt;

#[derive(PartialEq, Debug, Clone)]
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

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
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
    pub fn as_str(&self) -> &str {
        match self {
            Note::A => "A",
            Note::As => "A#",
            Note::B => "B",
            Note::C => "C",
            Note::Cs => "C#",
            Note::D => "D",
            Note::Ds => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::Fs => "F#",
            Note::G => "G",
            Note::Gs => "G#",
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

    pub fn n_half_steps_up(&self, n: u32) -> Note {
        if n == 0 {
            return self.clone();
        }
        let mut temp_note = self.half_step_up();
        for _ in 1..n {
            temp_note = temp_note.half_step_up();
        }
        temp_note
    }

    pub fn half_step_down(&self) -> Note {
        self.n_half_steps_up(11)
    }

    pub fn n_half_steps_down(&self, n: u32) -> Note {
        if n == 0 {
            return self.clone();
        }
        let mut temp_note = self.half_step_down();
        for _ in 1..n {
            temp_note = temp_note.half_step_down();
        }
        temp_note
    }
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
    fn test_as_str() {
        assert_eq!(Note::A.as_str(), "A");
        assert_eq!(Note::As.as_str(), "A#");
        assert_eq!(Note::B.as_str(), "B");
        assert_eq!(Note::C.as_str(), "C");
        assert_eq!(Note::Cs.as_str(), "C#");
        assert_eq!(Note::D.as_str(), "D");
        assert_eq!(Note::Ds.as_str(), "D#");
        assert_eq!(Note::E.as_str(), "E");
        assert_eq!(Note::F.as_str(), "F");
        assert_eq!(Note::Fs.as_str(), "F#");
        assert_eq!(Note::G.as_str(), "G");
        assert_eq!(Note::Gs.as_str(), "G#");
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

    #[test]
    fn test_n_half_steps_up() {
        assert_eq!(Note::A.n_half_steps_up(1), Note::As);
        assert_eq!(Note::C.n_half_steps_up(5), Note::F);
        assert_eq!(Note::C.n_half_steps_up(0), Note::C);
    }
    #[test]
    fn test_half_step_down() {
        assert_eq!(Note::A.half_step_down(), Note::Gs);
        assert_eq!(Note::F.half_step_down(), Note::E);
    }

    #[test]
    fn test_n_half_steps_down() {
        assert_eq!(Note::A.n_half_steps_down(1), Note::Gs);
        assert_eq!(Note::F.n_half_steps_down(5), Note::C);
        assert_eq!(Note::C.n_half_steps_down(0), Note::C);
    }
}
