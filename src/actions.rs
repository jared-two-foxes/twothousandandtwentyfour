use crate::{
    model::Model,
    vec2::Vec2,
}

enum Message {
    Compress(vec2),
}

fn update(message: Message, model: &mut Model) -> Option<Message> {
  match message {
    Message::Compress(dir) => {
       compress_left(
         model.grid,
         vec2::new(0,0));
       None
    }  
  }
} 

// going to start on the first square (i, with value x) on the left, 
    // compare it with the next square to its right (j); if we hit an 
    // empty space we'll try the next square (j+1), if its got the same 
    // value as us then combine with us and increment value set j+1 to 
    // empty then increment j, some value not x and x is empty then swap
    // with i increment i and repeat.
pub fn compress_left(grid: &mut Grid<SquareValue>, start: Vec2) -> Result<()> {
    let mut i = 0;
    let mut j = i + 1;
    while i < 4 {
        let a = Vec2::new(i,y);
        let v = grid.at(a).ok_or(SquareValue::Empty);
        let b = Vec2::new(j,y);
        match grid.at(b) {
            Some(SquareValue::Empty) => { j += 1; },
            Some(sv) => { 
                if v == SquareValue::Empty {
                    grid.swap(a,b) 
                } else if sv == v { 
                    v += 1;
                    *sv = SquareValue::Empty;
                } 
                i = j;
            }, 
            None => {  
                i = j;
            },
        }
    }
} 
