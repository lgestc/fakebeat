use handlebars::{handlebars_helper, Handlebars};

use fake::Fake;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use chrono::{Duration, Utc};

const FORMAT_ISO: &str = "%FT%T%z";

pub fn create<'a>() -> Handlebars<'a> {
    handlebars_helper!(random_iso_date: |*_args| {
        let mut rng = rand::thread_rng();
        let random_offset = rng.gen_range(0..30);
        let dt = Utc::now() - Duration::days(random_offset);

        dt.format(FORMAT_ISO).to_string()
    });

    let mut handlebars = Handlebars::new();

    handlebars_helper!(random_string: |*_args|  thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()

    );

    macro_rules! helpers {
        (    $hb:ident, $($i:ident : $p:path), *) => {
                $(
                    handlebars_helper!($i: |*_args| $p().fake::<String>());
                    handlebars.register_helper(stringify!($i), Box::new($i));
                )*
        };
    }

    handlebars.register_helper("string", Box::new(random_string));
    handlebars.register_helper("date_iso", Box::new(random_iso_date));

    helpers!(
        handlebars,
        username: fake::faker::internet::en::Username,
        domain: fake::faker::internet::en::DomainSuffix,
        ipv4: fake::faker::internet::en::IPv4,
        ipv6: fake::faker::internet::en::IPv6,
        word: fake::faker::lorem::en::Word
    );

    return handlebars;
}
