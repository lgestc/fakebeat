use handlebars::{handlebars_helper, Handlebars, JsonValue};

use fake::Fake;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use anyhow::Result;

use chrono::{Duration, Utc};

const FORMAT_ISO: &str = "%FT%T%z";

fn unwrap_number(args: Vec<&JsonValue>, default: i64) -> i64 {
    match args.get(0) {
        Some(v) => match v.as_i64() {
            Some(v) => v,
            None => default,
        },
        None => default,
    }
}

pub fn create<'a>() -> Handlebars<'a> {
    handlebars_helper!(date_range: |*args| {
        let mut rng = rand::thread_rng();

        let days_back = unwrap_number(args, 1);

        let random_offset = rng.gen_range(0..days_back);
        let dt = Utc::now() - Duration::days(random_offset);

        dt.format(FORMAT_ISO).to_string()
    });

    handlebars_helper!(now: |*_args| {
        let dt = Utc::now();

        dt.format(FORMAT_ISO).to_string()
    });

    handlebars_helper!(random_string: |*_args|  thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()

    );

    let mut handlebars = Handlebars::new();

    handlebars.register_helper("string", Box::new(random_string));
    handlebars.register_helper("date_range", Box::new(date_range));
    handlebars.register_helper("now", Box::new(now));

    macro_rules! register_faker_helpers {
        (    $($i:ident : $p:path), *) => {
                $(
                    handlebars_helper!($i: |*_args| $p().fake::<String>());
                    handlebars.register_helper(stringify!($i), Box::new($i));
                )*
        };
    }

    register_faker_helpers!(
        username: fake::faker::internet::en::Username,
        domain: fake::faker::internet::en::DomainSuffix,
        ipv4: fake::faker::internet::en::IPv4,
        ipv6: fake::faker::internet::en::IPv6,
        word: fake::faker::lorem::en::Word
    );

    return handlebars;
}
