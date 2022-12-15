use std::fs;

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    // Sand coming in at 500,0. Sand falls down, then down left, then down right. New sand appears when sand can't move.
    // How many sand falls in before it starts falling into abyss?
    // Pseudo code/ rough thoughts:
    // Once one grain falls in the abyss all future ones will do so
    // Abyss is just one y coord further than the lowest point (or highest y index) in our shape.
    // We can start by making a sand granule fall straight to lowest point below it
    // We can just make an array that indicates if anything is in a cell instead of a bunch of objects
    // Don't forget to discount the last granule that falls off.

    // while y < abyss
    // next_pos = [(0,+1), (-1, +1), (1, 1)]
    // for p in pos
    // loop
    //    (x,y) = map.get(x+pos.0, y+pos.0){
    //     some(1) => continue,
    //     some(0) => move here
    //     None => in the abyss
    // }
}
