use once_cell::sync::Lazy;
use std::collections::HashMap;

const B_INCR: f32 = 0.293;
const B_DECR: f32 = -0.293;

static BOOSTER_DICT: Lazy<HashMap<&'static str, f32>> = Lazy::new(|| {
    HashMap::from([
        ("absolutely", B_INCR), ("amazingly", B_INCR), ("awfully", B_INCR),
        ("completely", B_INCR), ("considerable", B_INCR), ("considerably", B_INCR),
        ("decidedly", B_INCR), ("deeply", B_INCR), ("effing", B_INCR),
        ("enormous", B_INCR), ("enormously", B_INCR),
        ("almost", B_DECR), ("barely", B_DECR), ("hardly", B_DECR),
        ("just enough", B_DECR), ("kind of", B_DECR), ("kinda", B_DECR),
        ("less", B_DECR), ("little", B_DECR), ("marginal", B_DECR),
        ("somewhat", B_DECR), ("sort of", B_DECR), ("sorta", B_DECR),
    ])
});

static SENTIMENTAL_IDIOMS: Lazy<HashMap<&'static str, f32>> = Lazy::new(|| {
    HashMap::from([
        ("cut the mustard", 2.0), ("hand to mouth", -2.0), ("back handed", -2.0),
        ("blow smoke", -2.0), ("break a leg", -2.0), ("cooking with gas", 2.0),
        ("in the black", 2.0), ("in the red", -2.0), ("on the ball", 2.0),
        ("under the weather", -2.0),
    ])
});

static SPECIAL_CASES: Lazy<HashMap<&'static str, f32>> = Lazy::new(|| {
    HashMap::from([
        ("the shit", 3.0), ("the bomb", 3.0), ("bad ass", 1.5), ("badass", 1.5),
        ("bus stop", 0.0), ("yeah right", -2.0), ("kiss of death", -1.5),
        ("to die for", 3.0), ("beating heart", 3.1), ("broken heart", -2.9),
    ])
});

fn negated(&input)
{
    input =
}

fn main() {
    println!("BOOSTER_DICT: {:?}", *BOOSTER_DICT);
    println!("SENTIMENTAL_IDIOMS: {:?}", *SENTIMENTAL_IDIOMS);
    println!("SPECIAL_CASES: {:?}", *SPECIAL_CASES);
}

