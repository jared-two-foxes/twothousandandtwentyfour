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
       compress(
         model.grid,
         dir);
       None
    },
    _ => None,  
  }
} 

// going to start on the first square (i, with value x) on the left, 
    // compare it with the next square to its right (j); if we hit an 
    // empty space we'll try the next square (j+1), if its got the same 
    // value as us then combine with us and increment value set j+1 to 
    // empty then increment j, some value not x and x is empty then swap
    // with i increment i and repeat.
pub fn compress(grid: &mut Grid<u16>, start: Vec2, direction: Vec2) -> Result<()> {
    let mut i = start;
    let mut j = i + direction;
    while let Some(v) = grid.at(i) {
        match grid.at(j) {
            Some(sv) => { 
                if v == 0 && sv != 0 {
                    grid.swap(i,j);
                    i += direction;
                } else if sv == v { 
                    *v *= 2;
                    *sv = 0;
                    i += direction;
                } 
                j += direction;
            }, 
            None => {  
                break;
            },
        }
    }
} 
