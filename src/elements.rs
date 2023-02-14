#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PathElementLabel {
    Move,
    Line,
    Horizontal,
    Vertical,
    CubicBezier,
    End,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PathElementCommand {
    relative: bool,
    label: PathElementLabel,
}
impl PathElementCommand {
    pub fn new(relative:bool, label:PathElementLabel) -> Self {
        Self {
            relative,
            label,
        }
    }

    pub fn from_ch(ch:char) -> Option<Self> {
        Some(match ch {
            'm' | 'M' => Self::new(ch.is_lowercase(), PathElementLabel::Move),
            'l' | 'L' => Self::new(ch.is_lowercase(), PathElementLabel::Line),
            'h' | 'H' => Self::new(ch.is_lowercase(), PathElementLabel::Horizontal),
            'v' | 'V' => Self::new(ch.is_lowercase(), PathElementLabel::Vertical),
            'c' | 'C' => Self::new(ch.is_lowercase(), PathElementLabel::CubicBezier),
            'z' | 'Z' => Self::new(false, PathElementLabel::End),
            _ => return None,
        })
    }

    pub fn relative(&self) -> bool {
        self.relative
    }

    pub fn label(&self) -> PathElementLabel {
        self.label
    }

    pub fn updated(&self) -> Self {
        match self.label {
            PathElementLabel::Move => Self::new(self.relative, PathElementLabel::Line),
            _ => *self,
        }
    }
}