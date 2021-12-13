#![allow(unused)]

pub struct A {
    inner: sha2::Sha256
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // dbg!(
        //     std::env::var("CARGO_CFG_TARGET_FAMILY")
        // );
        // dbg!(
        //     std::env::var("CARGO_CFG_TARGET_OS")
        // );
        // dbg!(
        //     std::env::var("CARGO_CFG_TARGET_ARCH")
        // );
        // dbg!(
        //     std::env::var("CARGO_CFG_TARGET_VENDOR")
        // );
        // dbg!(
        //     std::env::var("CARGO_CFG_TARGET_ENV")
        // );
        // dbg!(
        //     std::env::var("CARGO_CFG_WINDOWS")
        // );

        use sha2;

        #[cfg(any(not(any(target_arch = "x86", target_arch = "x86_64")), target_env = "msvc"))]
        println!("THIS SHOULD BE MSCV REGARDLESS");

        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(target_env = "msvc")))]
        println!("THIS SHOULD BE x86(_x86_64) AND NOT MSVC");
    }
}
