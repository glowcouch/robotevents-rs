use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub venue: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub city: String,
    pub region: Option<String>,
    pub postcode: Option<String>,
    pub country: String,
    pub coordinates: Coordinates,
}

#[cfg(feature = "fake")]
pub struct FakeLocation;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeLocation> for Location {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeLocation, rng: &mut R) -> Self {
        use fake::{
            faker::{
                address::en::{
                    CityName, CountryName, PostCode, SecondaryAddress, StreetName, StreetSuffix,
                },
                company::en::CompanyName,
            },
            Fake,
        };
        let country = CountryName().fake_with_rng(rng);
        let city = CityName().fake_with_rng(rng);
        Location {
            venue: CompanyName().fake_with_rng(rng),
            address_1: format!(
                "{}, {}, {} {} {}",
                country,
                city,
                rng.gen_range(0..=200),
                {
                    let v: String = StreetName().fake_with_rng(rng);
                    v
                },
                {
                    let v: String = StreetSuffix().fake_with_rng(rng);
                    v
                }
            ),
            address_2: match rng.gen_bool(0.05) {
                true => Some(SecondaryAddress().fake_with_rng(rng)),
                false => None,
            },
            city,
            region: None,
            postcode: PostCode().fake_with_rng(rng),
            country,
            coordinates: FakeCoordinates.fake_with_rng(rng),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: Option<f32>,
    pub lon: Option<f32>,
}

#[cfg(feature = "fake")]
pub struct FakeCoordinates;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeCoordinates> for Coordinates {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeCoordinates, rng: &mut R) -> Self {
        Coordinates {
            lat: Some(rng.gen_range(-180.0..180.0)),
            lon: Some(rng.gen_range(-180.0..180.0)),
        }
    }
}
