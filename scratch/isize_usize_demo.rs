// A tiny STANDALONE demo of the usize <-> isize boundary.
//
// This is deliberately NOT your Grid. It's a 1D toy (one index walking along a
// line of slots) so you can see ONLY the type logic, with nothing else around
// it. Once it clicks here, you port the *idea* into your 2D Grid yourself.
//
// Run it on its own (it is not part of your cargo build):
//   rustc scratch/isize_usize_demo.rs -o /tmp/demo && /tmp/demo

fn main() {
    // --- The setup ---------------------------------------------------------
    // Slots are numbered 0..width. A slot index is naturally UNSIGNED:
    // there is no such thing as "slot -1". So usize is the honest type for
    // a real, stored position.
    let width: usize = 10;

    // The current position sits at a real slot -> usize is honest here too.
    let pos: usize = 1;

    // The STEP (think: direction * speed) is the SIGNED thing.
    // -1 means "move left", +1 means "move right".
    let step: isize = -1;

    // --- Why you can't just write `pos + step` -----------------------------
    // Two reasons:
    //   1. Rust will not add a usize and an isize. They're different types;
    //      it refuses to silently pick one.
    //   2. Even if it would: if pos were 0 and step were -1, then `0usize - 1`
    //      PANICS (unsigned underflow). You'd never get a -1 you could look at
    //      and say "ah, out of bounds" — your program would just crash first.

    // --- The fix: do the arithmetic in SIGNED space ------------------------
    // Cast pos UP to isize for the math. In isize, -1 is a perfectly normal
    // value, so the result is something you can inspect instead of a crash.
    let candidate: isize = pos as isize + step; // 1 + (-1) = 0; if pos were 0 -> -1
    println!("candidate (signed) = {candidate}");

    // --- Check bounds WHILE STILL SIGNED -----------------------------------
    // THIS is the whole trick. Validate before you convert back.
    // `> 0` rejects both the wall at slot 0 AND any negative value in a single
    // comparison — which is exactly why you want to be in isize right here.
    let in_bounds: bool = candidate > 0 && candidate < (width as isize - 1);

    if in_bounds {
        // --- Only now is converting back to an index safe -----------------
        // We have PROVEN candidate >= 1 on this branch, so `as usize` cannot
        // lose information or wrap. The cast lives at exactly ONE spot: the
        // boundary where a value becomes a real slot index.
        let new_pos: usize = candidate as usize;
        println!("moved to slot {new_pos}");
    } else {
        // Out of bounds. In your real code, THIS is where a bounce happens:
        // flip the step's sign (step = -step) and don't move this frame.
        // Notice the candidate (0 or -1) is simply DISCARDED here — it is
        // never stored anywhere. That's the punchline:
        //
        //   - the CANDIDATE must be isize (it can be negative for a moment)
        //   - the STORED position never actually holds a negative, so it
        //     *could* stay usize... but if you make it isize too, then
        //     `pos as isize` disappears and the only cast left is the one
        //     `as usize` above. Fewer casts, one boundary. That's the trade.
        println!("hit a wall -> would bounce (flip step's sign), not move");
    }
}
