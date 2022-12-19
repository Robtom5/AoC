use std::fs;

fn part1(contents: &str) -> String {
    // Inclusing the starting node we need to create a list of (30-distances) to each node multiplied by node pressure.
    // This will provide us a first node to head to. Each node along the way can be opened provided that the net pressure it releases is
    // greater than the pressure released in one minute by the target node.
    // Once at the target node we can repeat the process (obviously leaving open any open nodes);

    // When evaluating the routes to the trget node we need to allow for detours that still provide a net boon

    // Gut feeling: If we pass by a node then there is no point opening it later (WRONG, consider 0 -- 1 -- 400, it always makes sense to backtrack)
    // A detour is never worth it, it would have scored higher on the worthwhile nodes if that was the case

    // We can start by generating the distnace between all nodes in a 2d array (x->y). Then at each step we can multiple the distance column of our current node
    // By a row of each nodes remaining pressure (or 0 if opened) to work out the best next location

    // Depth first search (A* for distance?)
    // We can ignore analysis of nodes of pressure 0, instead all of their connected nodes can just have their list of connected increased

    // Could work backwards. Start at minute 2, for each node what is the best to go visit next and cache, then go back in time another (2?) minutes (for navi + open)
    // Working backwards, minutes 30 and 29 dont matter (a valve opened on 29 doesnt do anything)
    // The routes between nodes will always have the same relevant weights (as a function of remaining time)
    // EG (X -- 0 -- 400) == f(t) => (t-3) * 400
    // Notably this is
    // t -= 1 // move
    // t -= 1 // move
    // t -= 1 // open
    // score = t*400

    // Provided you can make it to and open a node it doesn't matter what the actual time is

    "".to_string()
}

fn part2(contents: &str) -> String {
    "".to_string()
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let res1 = part1(&contents);
    let res2 = part2(&contents);

    println!("Part 1 {res1} Part 2 {res2}")
}
