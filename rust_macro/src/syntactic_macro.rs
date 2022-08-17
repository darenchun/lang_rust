/* syntactic macros */
// syntactic macros in rust is a collection of rules where
// the left-hand side dictates how the rule should be matched to an input,
// and the right-hand side dictates how the rule should expand to.
// a rule maps to an expression on the right hand side via '=>' syntax.
// variables local to a rule are declared using the '$' sign.
macro_rules! factorial {
    ($e:expr) => {{
        let mut result = 1;
        for i in 1..($e + 1) {
            result = result * i;
        }
        result
    }};
}
