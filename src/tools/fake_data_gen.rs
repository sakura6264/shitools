use super::*;
use eframe::egui;
use fake::Fake;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FakeDataGen {
    output: String,
    fake_data_type: FakeDataType,
    locale: Locale,
}

impl FakeDataGen {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            fake_data_type: FakeDataType::Lorem((0, 10)),
            locale: Locale::En,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum FakeDataType {
    Lorem((usize, usize)),
    Name,
    Internet((usize, usize)),
    Company,
    Currency,
    Creditcard,
    Address(u8),
    Barcode,
    PhoneNumber,
    Job,
    Filesystem,
    Finance,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Locale {
    En,
    Fr,
    Cn,
    Tw,
}

impl ToolComponent for FakeDataGen {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Copy").clicked() {
                ui.output_mut(|reader| {
                    reader.copied_text = self.output.clone();
                });
            }
            if ui.button("Clear").clicked() {
                self.output.clear();
            }
        });
        ui.add(
            egui::TextEdit::multiline(&mut self.output)
                .desired_rows(5)
                .desired_width(f32::INFINITY),
        );
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.locale, Locale::En, "en");
            ui.radio_value(&mut self.locale, Locale::Fr, "fr");
            ui.radio_value(&mut self.locale, Locale::Cn, "zh-cn");
            ui.radio_value(&mut self.locale, Locale::Tw, "zh-tw");
        });
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut self.fake_data_type,
                FakeDataType::Lorem((0, 10)),
                "Lorem",
            );
            ui.radio_value(&mut self.fake_data_type, FakeDataType::Name, "Name");
            ui.radio_value(
                &mut self.fake_data_type,
                FakeDataType::Internet((8, 16)),
                "Internet",
            );
            ui.radio_value(&mut self.fake_data_type, FakeDataType::Company, "Company");
            ui.radio_value(&mut self.fake_data_type, FakeDataType::Currency, "Currency");
        });
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut self.fake_data_type,
                FakeDataType::Creditcard,
                "Creditcard",
            );
            ui.radio_value(
                &mut self.fake_data_type,
                FakeDataType::Address(0),
                "Address",
            );
            ui.radio_value(&mut self.fake_data_type, FakeDataType::Barcode, "Barcode");
            ui.radio_value(
                &mut self.fake_data_type,
                FakeDataType::PhoneNumber,
                "PhoneNumber",
            );
            ui.radio_value(&mut self.fake_data_type, FakeDataType::Job, "Job");
            ui.radio_value(
                &mut self.fake_data_type,
                FakeDataType::Filesystem,
                "Filesystem",
            );
            ui.radio_value(&mut self.fake_data_type, FakeDataType::Finance, "Finance");
        });
        ui.separator();
        match self.fake_data_type {
            FakeDataType::Lorem((ref mut s, ref mut b)) => {
                if ui.button("Word").clicked() {
                    self.output = match self.locale {
                        Locale::En => fake::faker::lorem::en::Word().fake(),
                        Locale::Fr => fake::faker::lorem::fr_fr::Word().fake(),
                        Locale::Cn => fake::faker::lorem::zh_cn::Word().fake(),
                        Locale::Tw => fake::faker::lorem::zh_tw::Word().fake(),
                    }
                }
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(s).speed(1f32));
                    ui.label("< Length <");
                    ui.add(egui::DragValue::new(b).speed(1f32));
                });
                ui.horizontal(|ui| {
                    if ui.button("Sentence").clicked() && *s <= *b {
                        self.output = match self.locale {
                            Locale::En => fake::faker::lorem::en::Sentence(*s..*b).fake(),
                            Locale::Fr => fake::faker::lorem::fr_fr::Sentence(*s..*b).fake(),
                            Locale::Cn => fake::faker::lorem::zh_cn::Sentence(*s..*b).fake(),
                            Locale::Tw => fake::faker::lorem::zh_tw::Sentence(*s..*b).fake(),
                        }
                    }
                    if ui.button("Paragraph").clicked() && *s <= *b {
                        self.output = match self.locale {
                            Locale::En => fake::faker::lorem::en::Paragraph(*s..*b).fake(),
                            Locale::Fr => fake::faker::lorem::fr_fr::Paragraph(*s..*b).fake(),
                            Locale::Cn => fake::faker::lorem::zh_cn::Paragraph(*s..*b).fake(),
                            Locale::Tw => fake::faker::lorem::zh_tw::Paragraph(*s..*b).fake(),
                        }
                    }
                });
            }
            FakeDataType::Name => {
                ui.horizontal(|ui| {
                    if ui.button("Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::name::en::Name().fake(),
                            Locale::Fr => fake::faker::name::fr_fr::Name().fake(),
                            Locale::Cn => fake::faker::name::zh_cn::Name().fake(),
                            Locale::Tw => fake::faker::name::zh_tw::Name().fake(),
                        }
                    }
                    if ui.button("First Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::name::en::FirstName().fake(),
                            Locale::Fr => fake::faker::name::fr_fr::FirstName().fake(),
                            Locale::Cn => fake::faker::name::zh_cn::FirstName().fake(),
                            Locale::Tw => fake::faker::name::zh_tw::FirstName().fake(),
                        }
                    }
                    if ui.button("Last Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::name::en::LastName().fake(),
                            Locale::Fr => fake::faker::name::fr_fr::LastName().fake(),
                            Locale::Cn => fake::faker::name::zh_cn::LastName().fake(),
                            Locale::Tw => fake::faker::name::zh_tw::LastName().fake(),
                        }
                    }
                    if ui.button("Suffix").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::name::en::Suffix().fake(),
                            Locale::Fr => fake::faker::name::fr_fr::Suffix().fake(),
                            Locale::Cn => fake::faker::name::zh_cn::Suffix().fake(),
                            Locale::Tw => fake::faker::name::zh_tw::Suffix().fake(),
                        }
                    }
                    if ui.button("Title").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::name::en::Title().fake(),
                            Locale::Fr => fake::faker::name::fr_fr::Title().fake(),
                            Locale::Cn => fake::faker::name::zh_cn::Title().fake(),
                            Locale::Tw => fake::faker::name::zh_tw::Title().fake(),
                        }
                    }
                    if ui.button("Name with title").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::name::en::NameWithTitle().fake(),
                            Locale::Fr => fake::faker::name::fr_fr::NameWithTitle().fake(),
                            Locale::Cn => fake::faker::name::zh_cn::NameWithTitle().fake(),
                            Locale::Tw => fake::faker::name::zh_tw::NameWithTitle().fake(),
                        }
                    }
                });
            }
            FakeDataType::Internet((ref mut s, ref mut b)) => {
                ui.horizontal(|ui| {
                    if ui.button("Username").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::Username().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::Username().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::Username().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::Username().fake(),
                        }
                    }
                    if ui.button("Free Email").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::FreeEmail().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::FreeEmail().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::FreeEmail().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::FreeEmail().fake(),
                        }
                    }
                    if ui.button("Safe Email").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::SafeEmail().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::SafeEmail().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::SafeEmail().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::SafeEmail().fake(),
                        }
                    }
                    if ui.button("Free Email Provider").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::FreeEmailProvider().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::FreeEmailProvider().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::FreeEmailProvider().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::FreeEmailProvider().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Domain Suffix").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::DomainSuffix().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::DomainSuffix().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::DomainSuffix().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::DomainSuffix().fake(),
                        }
                    }
                    if ui.button("User Agent").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::UserAgent().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::UserAgent().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::UserAgent().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::UserAgent().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("IP").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::IP().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::IP().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::IP().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::IP().fake(),
                        }
                    }
                    if ui.button("IPv4").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::IPv4().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::IPv4().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::IPv4().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::IPv4().fake(),
                        }
                    }
                    if ui.button("IPv6").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::IPv6().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::IPv6().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::IPv6().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::IPv6().fake(),
                        }
                    }
                    if ui.button("MAC Address").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::internet::en::MACAddress().fake(),
                            Locale::Fr => fake::faker::internet::fr_fr::MACAddress().fake(),
                            Locale::Cn => fake::faker::internet::zh_cn::MACAddress().fake(),
                            Locale::Tw => fake::faker::internet::zh_tw::MACAddress().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(s).speed(1f32));
                    ui.label("< Length <");
                    ui.add(egui::DragValue::new(b).speed(1f32));
                });
                if ui.button("Password").clicked() && *s <= *b {
                    self.output = match self.locale {
                        Locale::En => fake::faker::internet::en::Password(*s..*b).fake(),
                        Locale::Fr => fake::faker::internet::fr_fr::Password(*s..*b).fake(),
                        Locale::Cn => fake::faker::internet::zh_cn::Password(*s..*b).fake(),
                        Locale::Tw => fake::faker::internet::zh_tw::Password(*s..*b).fake(),
                    };
                };
            }
            FakeDataType::Company => {
                ui.horizontal(|ui| {
                    if ui.button("Company Suffix").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::CompanySuffix().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::CompanySuffix().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::CompanySuffix().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::CompanySuffix().fake(),
                        }
                    }
                    if ui.button("Company Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::CompanyName().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::CompanyName().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::CompanyName().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::CompanyName().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Buzzword").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::Buzzword().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::Buzzword().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::Buzzword().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::Buzzword().fake(),
                        }
                    }
                    if ui.button("Buzzword Middle").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::BuzzwordMiddle().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::BuzzwordMiddle().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::BuzzwordMiddle().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::BuzzwordMiddle().fake(),
                        }
                    }
                    if ui.button("Buzzword Tail").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::BuzzwordTail().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::BuzzwordTail().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::BuzzwordTail().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::BuzzwordTail().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Catch Phrase").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::CatchPhrase().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::CatchPhrase().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::CatchPhrase().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::CatchPhrase().fake(),
                        }
                    }
                    if ui.button("Bs Verb").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::BsVerb().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::BsVerb().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::BsVerb().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::BsVerb().fake(),
                        }
                    }
                    if ui.button("Bs Adj").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::BsAdj().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::BsAdj().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::BsAdj().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::BsAdj().fake(),
                        }
                    }
                    if ui.button("Bs Noun").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::BsNoun().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::BsNoun().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::BsNoun().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::BsNoun().fake(),
                        }
                    }
                    if ui.button("Bs").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::Bs().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::Bs().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::Bs().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::Bs().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Profession").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::Profession().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::Profession().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::Profession().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::Profession().fake(),
                        }
                    }
                    if ui.button("Industry").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::company::en::Industry().fake(),
                            Locale::Fr => fake::faker::company::fr_fr::Industry().fake(),
                            Locale::Cn => fake::faker::company::zh_cn::Industry().fake(),
                            Locale::Tw => fake::faker::company::zh_tw::Industry().fake(),
                        }
                    }
                });
            }
            FakeDataType::Currency => {
                ui.horizontal(|ui| {
                    if ui.button("Currency Code").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::currency::en::CurrencyCode().fake(),
                            Locale::Fr => fake::faker::currency::fr_fr::CurrencyCode().fake(),
                            Locale::Cn => fake::faker::currency::zh_cn::CurrencyCode().fake(),
                            Locale::Tw => fake::faker::currency::zh_tw::CurrencyCode().fake(),
                        }
                    }
                    if ui.button("Currency Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::currency::en::CurrencyName().fake(),
                            Locale::Fr => fake::faker::currency::fr_fr::CurrencyName().fake(),
                            Locale::Cn => fake::faker::currency::zh_cn::CurrencyName().fake(),
                            Locale::Tw => fake::faker::currency::zh_tw::CurrencyName().fake(),
                        }
                    }
                    if ui.button("Currency Symbol").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::currency::en::CurrencySymbol().fake(),
                            Locale::Fr => fake::faker::currency::fr_fr::CurrencySymbol().fake(),
                            Locale::Cn => fake::faker::currency::zh_cn::CurrencySymbol().fake(),
                            Locale::Tw => fake::faker::currency::zh_tw::CurrencySymbol().fake(),
                        }
                    }
                });
            }
            FakeDataType::Creditcard => {
                if ui.button("Credit Card Number").clicked() {
                    self.output = match self.locale {
                        Locale::En => fake::faker::creditcard::en::CreditCardNumber().fake(),
                        Locale::Fr => fake::faker::creditcard::fr_fr::CreditCardNumber().fake(),
                        Locale::Cn => fake::faker::creditcard::zh_cn::CreditCardNumber().fake(),
                        Locale::Tw => fake::faker::creditcard::zh_tw::CreditCardNumber().fake(),
                    }
                }
            }
            FakeDataType::Address(ref mut p) => {
                ui.horizontal(|ui| {
                    if ui.button("City Prefix").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::CityPrefix().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::CityPrefix().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::CityPrefix().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::CityPrefix().fake(),
                        }
                    }
                    if ui.button("City Suffix").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::CitySuffix().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::CitySuffix().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::CitySuffix().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::CitySuffix().fake(),
                        }
                    }
                    if ui.button("City Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::CityName().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::CityName().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::CityName().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::CityName().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Country Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::CountryName().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::CountryName().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::CountryName().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::CountryName().fake(),
                        }
                    }
                    if ui.button("Country Code").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::CountryCode().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::CountryCode().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::CountryCode().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::CountryCode().fake(),
                        }
                    }
                    if ui.button("Street Suffix").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::StreetSuffix().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::StreetSuffix().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::StreetSuffix().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::StreetSuffix().fake(),
                        }
                    }
                    if ui.button("Street Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::StreetName().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::StreetName().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::StreetName().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::StreetName().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Time Zone").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::TimeZone().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::TimeZone().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::TimeZone().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::TimeZone().fake(),
                        }
                    }
                    if ui.button("State Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::StateName().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::StateName().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::StateName().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::StateName().fake(),
                        }
                    }
                    if ui.button("State Abbr").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::StateAbbr().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::StateAbbr().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::StateAbbr().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::StateAbbr().fake(),
                        }
                    }
                    if ui.button("Secondary Address Type").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::SecondaryAddressType().fake(),
                            Locale::Fr => {
                                fake::faker::address::fr_fr::SecondaryAddressType().fake()
                            }
                            Locale::Cn => {
                                fake::faker::address::zh_cn::SecondaryAddressType().fake()
                            }
                            Locale::Tw => {
                                fake::faker::address::zh_tw::SecondaryAddressType().fake()
                            }
                        }
                    }
                    if ui.button("Secondary Address").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::SecondaryAddress().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::SecondaryAddress().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::SecondaryAddress().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::SecondaryAddress().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Zip Code").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::ZipCode().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::ZipCode().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::ZipCode().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::ZipCode().fake(),
                        }
                    }
                    if ui.button("Post Code").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::PostCode().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::PostCode().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::PostCode().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::PostCode().fake(),
                        }
                    }
                    if ui.button("Building Number").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::BuildingNumber().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::BuildingNumber().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::BuildingNumber().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::BuildingNumber().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Latitude").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::Latitude().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::Latitude().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::Latitude().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::Latitude().fake(),
                        }
                    }
                    if ui.button("Longitude").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::Longitude().fake(),
                            Locale::Fr => fake::faker::address::fr_fr::Longitude().fake(),
                            Locale::Cn => fake::faker::address::zh_cn::Longitude().fake(),
                            Locale::Tw => fake::faker::address::zh_tw::Longitude().fake(),
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    ui.add(egui::DragValue::new(p).speed(1f32));
                    if ui.button("Geohash").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::address::en::Geohash(*p).fake(),
                            Locale::Fr => fake::faker::address::fr_fr::Geohash(*p).fake(),
                            Locale::Cn => fake::faker::address::zh_cn::Geohash(*p).fake(),
                            Locale::Tw => fake::faker::address::zh_tw::Geohash(*p).fake(),
                        }
                    }
                });
            }
            FakeDataType::Barcode => {
                ui.horizontal(|ui| {
                    if ui.button("ISBN").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::barcode::en::Isbn().fake(),
                            Locale::Fr => fake::faker::barcode::fr_fr::Isbn().fake(),
                            Locale::Cn => fake::faker::barcode::zh_cn::Isbn().fake(),
                            Locale::Tw => fake::faker::barcode::zh_tw::Isbn().fake(),
                        }
                    }
                    if ui.button("ISBN13").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::barcode::en::Isbn13().fake(),
                            Locale::Fr => fake::faker::barcode::fr_fr::Isbn13().fake(),
                            Locale::Cn => fake::faker::barcode::zh_cn::Isbn13().fake(),
                            Locale::Tw => fake::faker::barcode::zh_tw::Isbn13().fake(),
                        }
                    }
                    if ui.button("ISBN10").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::barcode::en::Isbn10().fake(),
                            Locale::Fr => fake::faker::barcode::fr_fr::Isbn10().fake(),
                            Locale::Cn => fake::faker::barcode::zh_cn::Isbn10().fake(),
                            Locale::Tw => fake::faker::barcode::zh_tw::Isbn10().fake(),
                        }
                    }
                });
            }
            FakeDataType::PhoneNumber => {
                ui.horizontal(|ui| {
                    if ui.button("Phone Number").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::phone_number::en::PhoneNumber().fake(),
                            Locale::Fr => fake::faker::phone_number::fr_fr::PhoneNumber().fake(),
                            Locale::Cn => fake::faker::phone_number::zh_cn::PhoneNumber().fake(),
                            Locale::Tw => fake::faker::phone_number::zh_tw::PhoneNumber().fake(),
                        }
                    }
                    if ui.button("Cell Number").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::phone_number::en::CellNumber().fake(),
                            Locale::Fr => fake::faker::phone_number::fr_fr::CellNumber().fake(),
                            Locale::Cn => fake::faker::phone_number::zh_cn::CellNumber().fake(),
                            Locale::Tw => fake::faker::phone_number::zh_tw::CellNumber().fake(),
                        }
                    }
                });
            }
            FakeDataType::Job => {
                ui.horizontal(|ui| {
                    if ui.button("Job Title").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::job::en::Title().fake(),
                            Locale::Fr => fake::faker::job::fr_fr::Title().fake(),
                            Locale::Cn => fake::faker::job::zh_cn::Title().fake(),
                            Locale::Tw => fake::faker::job::zh_tw::Title().fake(),
                        }
                    }
                    if ui.button("Job Field").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::job::en::Field().fake(),
                            Locale::Fr => fake::faker::job::fr_fr::Field().fake(),
                            Locale::Cn => fake::faker::job::zh_cn::Field().fake(),
                            Locale::Tw => fake::faker::job::zh_tw::Field().fake(),
                        }
                    }
                    if ui.button("Job Seniority").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::job::en::Seniority().fake(),
                            Locale::Fr => fake::faker::job::fr_fr::Seniority().fake(),
                            Locale::Cn => fake::faker::job::zh_cn::Seniority().fake(),
                            Locale::Tw => fake::faker::job::zh_tw::Seniority().fake(),
                        }
                    }
                    if ui.button("Position").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::job::en::Position().fake(),
                            Locale::Fr => fake::faker::job::fr_fr::Position().fake(),
                            Locale::Cn => fake::faker::job::zh_cn::Position().fake(),
                            Locale::Tw => fake::faker::job::zh_tw::Position().fake(),
                        }
                    }
                });
            }
            FakeDataType::Filesystem => {
                ui.horizontal(|ui| {
                    if ui.button("File Path").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::filesystem::en::FilePath().fake(),
                            Locale::Fr => fake::faker::filesystem::fr_fr::FilePath().fake(),
                            Locale::Cn => fake::faker::filesystem::zh_cn::FilePath().fake(),
                            Locale::Tw => fake::faker::filesystem::zh_tw::FilePath().fake(),
                        }
                    }
                    if ui.button("File Name").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::filesystem::en::FileName().fake(),
                            Locale::Fr => fake::faker::filesystem::fr_fr::FileName().fake(),
                            Locale::Cn => fake::faker::filesystem::zh_cn::FileName().fake(),
                            Locale::Tw => fake::faker::filesystem::zh_tw::FileName().fake(),
                        }
                    }
                    if ui.button("File Extension").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::filesystem::en::FileExtension().fake(),
                            Locale::Fr => fake::faker::filesystem::fr_fr::FileExtension().fake(),
                            Locale::Cn => fake::faker::filesystem::zh_cn::FileExtension().fake(),
                            Locale::Tw => fake::faker::filesystem::zh_tw::FileExtension().fake(),
                        }
                    }
                    if ui.button("Dir Path").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::filesystem::en::DirPath().fake(),
                            Locale::Fr => fake::faker::filesystem::fr_fr::DirPath().fake(),
                            Locale::Cn => fake::faker::filesystem::zh_cn::DirPath().fake(),
                            Locale::Tw => fake::faker::filesystem::zh_tw::DirPath().fake(),
                        }
                    }
                });
            }
            FakeDataType::Finance => {
                ui.horizontal(|ui| {
                    if ui.button("Bic").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::finance::en::Bic().fake(),
                            Locale::Fr => fake::faker::finance::fr_fr::Bic().fake(),
                            Locale::Cn => fake::faker::finance::zh_cn::Bic().fake(),
                            Locale::Tw => fake::faker::finance::zh_tw::Bic().fake(),
                        }
                    }
                    if ui.button("Isin").clicked() {
                        self.output = match self.locale {
                            Locale::En => fake::faker::finance::en::Isin().fake(),
                            Locale::Fr => fake::faker::finance::fr_fr::Isin().fake(),
                            Locale::Cn => fake::faker::finance::zh_cn::Isin().fake(),
                            Locale::Tw => fake::faker::finance::zh_tw::Isin().fake(),
                        }
                    }
                });
            }
        }
    }
}
