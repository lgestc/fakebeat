use std::collections::HashMap;

use fake::{
    faker::{boolean::en::Boolean, internet::en::Username},
    Fake,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use chrono::{Duration, Utc};

use serde_json::{from_value, to_value};
use tera::{Context, Function, Result, Tera, Value};

const FORMAT_ISO: &str = "%FT%T%z";

pub struct DocumentRenderer {
    generators: Vec<String>,
    tera: Tera,
}

impl DocumentRenderer {
    pub fn render(&mut self, template: &str) -> anyhow::Result<String> {
        let context = Context::default();

        match self.tera.render_str(template, &context) {
            Ok(document_string) => Ok(document_string),
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    }

    pub fn get_generators(&self) -> Vec<String> {
        self.generators.clone()
    }

    fn register_generator<F: Function + 'static>(&mut self, name: &str, function: F) {
        self.tera.register_function(name, function);
        self.generators.push(name.to_owned());
    }

    fn register_generators(&mut self) {
        self.register_generator(
            "date",
            Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
                match args.get("days_back") {
                    Some(days_back) => match from_value::<i64>(days_back.clone()) {
                        Ok(days_back) => {
                            let mut rng = rand::thread_rng();

                            let random_offset = rng.gen_range(0..days_back);
                            let dt = Utc::now() - Duration::days(random_offset);

                            Ok(to_value(dt.format(FORMAT_ISO).to_string()).unwrap())
                        }
                        Err(_) => Err("".into()),
                    },
                    None => {
                        let now = Utc::now().format(FORMAT_ISO);
                        Ok(to_value(now.to_string()).unwrap())
                    }
                }
            }),
        );

        self.register_generator(
            "now",
            Box::new(move |_: &HashMap<String, Value>| -> Result<Value> {
                let now = Utc::now().format(FORMAT_ISO);
                Ok(to_value(now.to_string()).unwrap_or_default())
            }),
        );

        self.register_generator(
            "hash",
            Box::new(move |_: &HashMap<String, Value>| -> Result<Value> {
                let value = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect::<String>();

                let value = to_value(value).unwrap_or_default();

                Ok(value)
            }),
        );

        macro_rules! register_faker_generators {
            (    $($i:ident : $p:path), *) => {
                    $(
                        self.register_generator(stringify!($i), Box::new(move |_: &HashMap<String, Value>| -> Result<Value> {
                            let value = to_value($p().fake::<String>()).unwrap_or_default();
                            Ok(value)
                        }));
                    )*
                }
            }

        register_faker_generators!(
            // Numbers
            digit: fake::faker::number::en::Digit,
            // Internet
            username: fake::faker::internet::en::Username,
            domainsuffix: fake::faker::internet::en::DomainSuffix,
            ipv4: fake::faker::internet::en::IPv4,
            ipv6: fake::faker::internet::en::IPv6,
            ip: fake::faker::internet::en::IP,
            macaddress: fake::faker::internet::en::MACAddress,
            freeemail: fake::faker::internet::en::FreeEmail,
            safeemail: fake::faker::internet::en::SafeEmail,
            freeemailprovider: fake::faker::internet::en::FreeEmailProvider,
            // HTTP
            // rfcstatuscode: fake::faker::http::RfcStatusCode,
            // validstatuscode: fake::faker::http::ValidStatusCode,
            // Lorem ipsum
            word: fake::faker::lorem::en::Word,
            // Name
            firstname: fake::faker::name::en::FirstName,
            lastname: fake::faker::name::en::LastName,
            title: fake::faker::name::en::Title,
            suffix: fake::faker::name::en::Suffix,
            name: fake::faker::name::en::Name,
            namewithtitle: fake::faker::name::en::NameWithTitle,
            //Filesystem
            filepath: fake::faker::filesystem::en::FilePath,
            filename: fake::faker::filesystem::en::FileName,
            fileextension: fake::faker::filesystem::en::FileExtension,
            dirpath: fake::faker::filesystem::en::DirPath,
            // Company
            companysuffix: fake::faker::company::en::CompanySuffix,
            companyname: fake::faker::company::en::CompanyName,
            buzzword: fake::faker::company::en::Buzzword,
            buzzwordmiddle: fake::faker::company::en::BuzzwordMiddle,
            buzzwordtail: fake::faker::company::en::BuzzwordTail,
            catchphase: fake::faker::company::en::CatchPhase,
            bsverb: fake::faker::company::en::BsVerb,
            bsadj: fake::faker::company::en::BsAdj,
            bsnoun: fake::faker::company::en::BsNoun,
            bs: fake::faker::company::en::Bs,
            profession: fake::faker::company::en::Profession,
            industry: fake::faker::company::en::Industry,
            // Address
            cityprefix: fake::faker::address::en::CityPrefix,
            citysuffix: fake::faker::address::en::CitySuffix,
            cityname: fake::faker::address::en::CityName,
            countryname: fake::faker::address::en::CountryName,
            countrycode: fake::faker::address::en::CountryCode,
            streetsuffix: fake::faker::address::en::StreetSuffix,
            streetname: fake::faker::address::en::StreetName,
            timezone: fake::faker::address::en::TimeZone,
            statename: fake::faker::address::en::StateName,
            stateabbr: fake::faker::address::en::StateAbbr,
            secondaryaddresstype: fake::faker::address::en::SecondaryAddressType,
            secondaryaddress: fake::faker::address::en::SecondaryAddress,
            zipcode: fake::faker::address::en::ZipCode,
            postcode: fake::faker::address::en::PostCode,
            buildingnumber: fake::faker::address::en::BuildingNumber,
            latitude: fake::faker::address::en::Latitude,
            longitude: fake::faker::address::en::Longitude
        );
    }

    fn new() -> Self {
        let tera = Tera::default();

        let generators = Vec::<String>::new();

        return Self { tera, generators };
    }
}

pub struct DocumentRendererFactory {}

impl DocumentRendererFactory {
    pub fn create_renderer() -> DocumentRenderer {
        let mut document_renderer = DocumentRenderer::new();

        document_renderer.register_generators();

        document_renderer
    }
}
