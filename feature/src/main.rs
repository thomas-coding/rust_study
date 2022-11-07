use std::env;

fn main() {
    #[cfg(feature = "feature1")]
    {
        println!("feature1 enable 1");
        println!("feature1 enable 2");
    }

    #[cfg(feature = "feature2")]
    println!("feature2 enable");

    #[cfg(feature = "feature3")]
    println!("feature3 enable");

    let test = env::var("CARGO_FEATURE_TEST").is_ok();
    println!("test {}", test);
}
