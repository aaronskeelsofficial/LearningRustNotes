fn main() {

/*  Rust handles type conversions via a few main pillars
 *      as: These are Rust defined conversions. You can't make one yourself.
 *      From/Into: These are trait defined conversions. You can define them. They are assumed to always work and never have a fail state (converting string to int can fail and wouldn't use this). If you define From, you get the conjugate Into for free implemented by Rust. If you attempt to define Into and override the Rust provided one, you get an error. Takeaway: Define "From"s only, Rust will define "Into"s for you.
 *      TryFrom/TryInto: There are like From/Into but are assumed to have a potential fail state.
 *      FromStr: This is an entire subsystem of TryFrom/TryInto that relates specifically to strings.
 */

    let x = 300_u16 as u8; // 44
    //As behavior

    impl From<i32> for u32 {
        fn from(value: i32) -> u32 {
            value.max(0) as u32
        }
    }
    // This would not compile due to the "orphan rule" which disallows defining a foreign trait across all foreign types including generics. One of those MUST belong to your crate for you to be allowed to define it.

    impl TryFrom<i32> for Port {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            // Convert value into Port, or return an error
        }
    }

    impl FromStr for Port {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Interpret s as a Port, or return an error
        }
    }
}