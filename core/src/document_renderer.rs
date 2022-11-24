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
            "username",
            Box::new(move |_: &HashMap<String, Value>| -> Result<Value> {
                let value = to_value(Username().fake::<String>()).unwrap_or_default();
                Ok(value)
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

        // register_faker_helpers!(
        //     // Numbers
        //     Digit: fake::faker::number::en::Digit,
        //     // Internet
        //     Username: fake::faker::internet::en::Username,
        //     DomainSuffix: fake::faker::internet::en::DomainSuffix,
        //     IPv4: fake::faker::internet::en::IPv4,
        //     IPv6: fake::faker::internet::en::IPv6,
        //     IP: fake::faker::internet::en::IP,
        //     MACAddress: fake::faker::internet::en::MACAddress,
        //     FreeEmail: fake::faker::internet::en::FreeEmail,
        //     SafeEmail: fake::faker::internet::en::SafeEmail,
        //     FreeEmailProvider: fake::faker::internet::en::FreeEmailProvider,
        //     // HTTP
        //     // RfcStatusCode: fake::faker::http::RfcStatusCode,
        //     // ValidStatusCode: fake::faker::http::ValidStatusCode,
        //     // Lorem ipsum
        //     Word: fake::faker::lorem::en::Word,
        //     // Name
        //     FirstName: fake::faker::name::en::FirstName,
        //     LastName: fake::faker::name::en::LastName,
        //     Title: fake::faker::name::en::Title,
        //     Suffix: fake::faker::name::en::Suffix,
        //     Name: fake::faker::name::en::Name,
        //     NameWithTitle: fake::faker::name::en::NameWithTitle,
        //     //Filesystem
        //     FilePath: fake::faker::filesystem::en::FilePath,
        //     FileName: fake::faker::filesystem::en::FileName,
        //     FileExtension: fake::faker::filesystem::en::FileExtension,
        //     DirPath: fake::faker::filesystem::en::DirPath,
        //     // Company
        //     CompanySuffix: fake::faker::company::en::CompanySuffix,
        //     CompanyName: fake::faker::company::en::CompanyName,
        //     Buzzword: fake::faker::company::en::Buzzword,
        //     BuzzwordMiddle: fake::faker::company::en::BuzzwordMiddle,
        //     BuzzwordTail: fake::faker::company::en::BuzzwordTail,
        //     CatchPhase: fake::faker::company::en::CatchPhase,
        //     BsVerb: fake::faker::company::en::BsVerb,
        //     BsAdj: fake::faker::company::en::BsAdj,
        //     BsNoun: fake::faker::company::en::BsNoun,
        //     Bs: fake::faker::company::en::Bs,
        //     Profession: fake::faker::company::en::Profession,
        //     Industry: fake::faker::company::en::Industry,
        //     // Address
        //     CityPrefix: fake::faker::address::en::CityPrefix,
        //     CitySuffix: fake::faker::address::en::CitySuffix,
        //     CityName: fake::faker::address::en::CityName,
        //     CountryName: fake::faker::address::en::CountryName,
        //     CountryCode: fake::faker::address::en::CountryCode,
        //     StreetSuffix: fake::faker::address::en::StreetSuffix,
        //     StreetName: fake::faker::address::en::StreetName,
        //     TimeZone: fake::faker::address::en::TimeZone,
        //     StateName: fake::faker::address::en::StateName,
        //     StateAbbr: fake::faker::address::en::StateAbbr,
        //     SecondaryAddressType: fake::faker::address::en::SecondaryAddressType,
        //     SecondaryAddress: fake::faker::address::en::SecondaryAddress,
        //     ZipCode: fake::faker::address::en::ZipCode,
        //     PostCode: fake::faker::address::en::PostCode,
        //     BuildingNumber: fake::faker::address::en::BuildingNumber,
        //     Latitude: fake::faker::address::en::Latitude,
        //     Longitude: fake::faker::address::en::Longitude
        // );
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
