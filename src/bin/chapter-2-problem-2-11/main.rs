fn main() {
    // A. k + 1
    //
    // B. Let's look at the value table if two input values are the same
    //
    // Step        *x           *y
    // Initially   a            a
    // Step 1.1    a            a ^ a
    //      1.2    0            0
    // Step 2.1    0 ^ 0        0
    //      2.2    0            0
    // Step 3.1    0            0 ^ 0
    //      3.2    0            0
    //
    // Basically, x and y would point to the same place, and at one point, once
    // the value would be `a ^ a` which is always `0`.
    //
    // C. We can do `first < last`, which would skip the middle value.
}
