fn main() {
    // Expression                       Type        Evaluation
    // ---                              ---         ---
    // -2147483647-1  ==  2147483648U   Unsigned    1
    // -2147483647-1  <   2147483647    Signed      1
    // -2147483647-1U <   2147483647    Unsinged    0
    // -2147483647-1  <  -2147483647    Signed      1
    // -2147483647-1U <  -2147483647    Unsigned    1
}
