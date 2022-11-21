use handlebars::{handlebars_helper, Handlebars};

use fake::{
    faker::{
        internet::en::{DomainSuffix, IPv4, IPv6, Username},
        lorem::en::Word,
    },
    Fake,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use chrono::{Duration, Utc};

const FORMAT_ISO: &str = "%FT%T%z";

handlebars_helper!(random_iso_date: |*_args| {
    let mut rng = rand::thread_rng();
    let random_offset = rng.gen_range(0..30);
    let dt = Utc::now() - Duration::days(random_offset);

    dt.format(FORMAT_ISO).to_string()
});

handlebars_helper!(random_string: |*_args|  thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()

);

// see https://crates.io/crates/fake

handlebars_helper!(random_username: |*_args| Username().fake::<String>());
handlebars_helper!(random_domain: |*_args| DomainSuffix().fake::<String>());
handlebars_helper!(random_word: |*_args| Word().fake::<String>());
handlebars_helper!(random_ipv4: |*_args| IPv4().fake::<String>());
handlebars_helper!(random_ipv6: |*_args| IPv6().fake::<String>());

pub fn create<'a>() -> Handlebars<'a> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("string", Box::new(random_string));
    handlebars.register_helper("date_iso", Box::new(random_iso_date));

    handlebars.register_helper("word", Box::new(random_word));

    handlebars.register_helper("domain", Box::new(random_domain));
    handlebars.register_helper("ipv4", Box::new(random_ipv4));
    handlebars.register_helper("ipv6", Box::new(random_ipv6));

    handlebars.register_helper("username", Box::new(random_username));

    return handlebars;
}
