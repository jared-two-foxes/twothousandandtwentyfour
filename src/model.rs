use crate::{ 
    vec2::Vec2, 
    grid::Grid
}
 
enum SquareValue {
    Empty,
    Two,
    Four,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
    OneTwentyEight,
    TwoFiftySix,
    FiveTwelve,
    TenTwentyFour,
    TwentyFourtyEight,
}

pub struct Model {
    pub grid: Grid<SquareValue>,
}

