#![feature(test)]

#[cfg(test)]
mod chunks {
    extern crate test;

    use serenity::client::EventHandler;
    use serenity::prelude::GatewayIntents;
    use serenity::Client;

    use self::test::Bencher;

    #[bench]
    fn client_startup(b: &mut Bencher) {
        b.bench(|b| {});
    }
}
