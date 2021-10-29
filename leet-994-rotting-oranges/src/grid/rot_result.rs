pub enum RottingState {
    Rotting, // no oranges were rotted
    Stuck,   // all remaining oranges were rotted
    Done,    // some remaining oranges were rotted
}

pub enum RotResult {
    Rot(RottingState), // fresh oranges left to rot
    Complete,          // no fresh oranges left to rot
}
