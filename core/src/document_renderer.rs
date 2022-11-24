use handlebars::{handlebars_helper, Handlebars, HelperDef, JsonValue};

use fake::{faker::boolean::en::Boolean, Fake};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use anyhow::Result;

use chrono::{Duration, Utc};

const FORMAT_ISO: &str = "%FT%T%z";

pub struct DocumentRenderer<'a> {
    generators: Vec<String>,
    handlebars: Handlebars<'a>,
}

impl<'a> DocumentRenderer<'a> {
    pub fn render(&self, template: &str) -> Result<String> {
        match self.handlebars.render_template(template, &()) {
            Ok(document_string) => Ok(document_string),
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    }

    pub fn get_generators(&self) -> Vec<String> {
        self.generators.clone()
    }

    pub fn new() -> Self {
        let mut generators = Vec::<String>::new();

        let mut handlebars = Handlebars::new();

        let mut register_helper = |name: &str, def: Box<dyn HelperDef + Send + Sync + 'a>| {
            handlebars.register_helper(name, def);
            generators.push(name.to_owned());
        };

        handlebars_helper!(date_range: |*args| {
            let mut rng = rand::thread_rng();

            let days_back = extract_arg(args, 0).as_i64().unwrap_or(1);

            let random_offset = rng.gen_range(0..days_back);
            let dt = Utc::now() - Duration::days(random_offset);

            dt.format(FORMAT_ISO).to_string()
        });
        register_helper("DateRange", Box::new(date_range));

        handlebars_helper!(boolean: |*args| {
            let ratio: u64 = extract_arg(args, 0).as_u64().unwrap_or(128);

            Boolean(ratio as u8).fake::<bool>()
        });
        register_helper("Boolean", Box::new(boolean));

        handlebars_helper!(now: |*_args| {
            let dt = Utc::now();

            dt.format(FORMAT_ISO).to_string()
        });
        register_helper("Now", Box::new(now));

        handlebars_helper!(hash: |*_args|  thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()

        );
        register_helper("Hash", Box::new(hash));

        macro_rules! register_faker_helpers {
            (    $($i:ident : $p:path), *) => {
                    $(
                        handlebars_helper!($i: |*_args| $p().fake::<String>());
                        register_helper(stringify!($i), Box::new($i));
                    )*
            };
        }

        register_faker_helpers!(
            // Numbers
            Digit: fake::faker::number::en::Digit,
            // Internet
            Username: fake::faker::internet::en::Username,
            DomainSuffix: fake::faker::internet::en::DomainSuffix,
            IPv4: fake::faker::internet::en::IPv4,
            IPv6: fake::faker::internet::en::IPv6,
            IP: fake::faker::internet::en::IP,
            MACAddress: fake::faker::internet::en::MACAddress,
            FreeEmail: fake::faker::internet::en::FreeEmail,
            SafeEmail: fake::faker::internet::en::SafeEmail,
            FreeEmailProvider: fake::faker::internet::en::FreeEmailProvider,
            // HTTP
            // RfcStatusCode: fake::faker::http::RfcStatusCode,
            // ValidStatusCode: fake::faker::http::ValidStatusCode,
            // Lorem ipsum
            Word: fake::faker::lorem::en::Word,
            // Name
            FirstName: fake::faker::name::en::FirstName,
            LastName: fake::faker::name::en::LastName,
            Title: fake::faker::name::en::Title,
            Suffix: fake::faker::name::en::Suffix,
            Name: fake::faker::name::en::Name,
            NameWithTitle: fake::faker::name::en::NameWithTitle,
            //Filesystem
            FilePath: fake::faker::filesystem::en::FilePath,
            FileName: fake::faker::filesystem::en::FileName,
            FileExtension: fake::faker::filesystem::en::FileExtension,
            DirPath: fake::faker::filesystem::en::DirPath,
            // Company
            CompanySuffix: fake::faker::company::en::CompanySuffix,
            CompanyName: fake::faker::company::en::CompanyName,
            Buzzword: fake::faker::company::en::Buzzword,
            BuzzwordMiddle: fake::faker::company::en::BuzzwordMiddle,
            BuzzwordTail: fake::faker::company::en::BuzzwordTail,
            CatchPhase: fake::faker::company::en::CatchPhase,
            BsVerb: fake::faker::company::en::BsVerb,
            BsAdj: fake::faker::company::en::BsAdj,
            BsNoun: fake::faker::company::en::BsNoun,
            Bs: fake::faker::company::en::Bs,
            Profession: fake::faker::company::en::Profession,
            Industry: fake::faker::company::en::Industry,
            // Address
            CityPrefix: fake::faker::address::en::CityPrefix,
            CitySuffix: fake::faker::address::en::CitySuffix,
            CityName: fake::faker::address::en::CityName,
            CountryName: fake::faker::address::en::CountryName,
            CountryCode: fake::faker::address::en::CountryCode,
            StreetSuffix: fake::faker::address::en::StreetSuffix,
            StreetName: fake::faker::address::en::StreetName,
            TimeZone: fake::faker::address::en::TimeZone,
            StateName: fake::faker::address::en::StateName,
            StateAbbr: fake::faker::address::en::StateAbbr,
            SecondaryAddressType: fake::faker::address::en::SecondaryAddressType,
            SecondaryAddress: fake::faker::address::en::SecondaryAddress,
            ZipCode: fake::faker::address::en::ZipCode,
            PostCode: fake::faker::address::en::PostCode,
            BuildingNumber: fake::faker::address::en::BuildingNumber,
            Latitude: fake::faker::address::en::Latitude,
            Longitude: fake::faker::address::en::Longitude
        );

        return Self {
            handlebars,
            generators,
        };
    }
}

fn extract_arg(args: Vec<&JsonValue>, arg: usize) -> JsonValue {
    match args.get(arg) {
        Some(v) => v.to_owned().to_owned(),
        None => serde_json::Value::default(),
    }
}

pub fn create<'a>() -> DocumentRenderer<'a> {
    DocumentRenderer::new()
}
