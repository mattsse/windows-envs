#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        dbg!(
            std::env::var("CARGO_CFG_TARGET_FAMILY")
        );
        dbg!(
            std::env::var("CARGO_CFG_TARGET_OS")
        );
        dbg!(
            std::env::var("CARGO_CFG_TARGET_ARCH")
        );
        dbg!(
            std::env::var("CARGO_CFG_TARGET_VENDOR")
        );
        dbg!(
            std::env::var("CARGO_CFG_TARGET_ENV")
        );
        dbg!(
            std::env::var("CARGO_CFG_WINDOWS")
        );
    }
}
