use druid::Data;

#[derive(Clone, PartialEq, Data)]
pub enum Tool {
    Arrow, 
    Pen,
    Marker,
    Eraser
}

impl Tool {
    pub fn should_be_written (&self) -> bool {

        !matches!(
            self,
            Tool::Arrow | Tool::Eraser
        )
    }
}

impl Into<u64> for Tool {
    fn into(self) -> u64 {
        match self {
            Tool::Arrow => 0,
            Tool::Pen => 1,
            Tool::Marker => 2, 
            Tool::Eraser => 3,
        }
    }
}

impl From<u64> for Tool {
    fn from(v: u64) -> Self {
        match v {
            1 => Tool::Pen,
            2 => Tool::Marker,
            3 => Tool::Eraser,
            0 | _ => Tool::Arrow
        }
    }
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Arrow
    }
}
/*
impl From<Tool> for u64 {
    fn from(t: Tool) -> Self {
        match t {
            Tool::Arrow => 0,
            Tool::Pen => 1,
            Tool::Marker => 2,
            Tool::Eraser => 3,
        }
    }
}*/
