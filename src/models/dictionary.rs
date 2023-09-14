use crate::{
    error::{ApiError, ApiResult},
    models::{Counters, Entry, LanguageCode},
};
use chrono::{DateTime, Utc};
use csv::StringRecord;
use enum_map::EnumMap;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub type DictionaryHandle = RwLock<Dictionary>;

#[derive(Debug)]
pub struct Dictionary {
    url: String,
    pub entries: HashMap<u32, Entry>,
    pub counters: Counters,
    pub last_update: DateTime<Utc>,
}

impl Dictionary {
    pub async fn from_url(url: &str) -> ApiResult<DictionaryHandle> {
        let mut dict = Dictionary {
            url: url.to_string(),
            entries: HashMap::new(),
            counters: Default::default(),
            last_update: Utc::now(),
        };
        dict.update().await?;
        Ok(RwLock::new(dict))
    }

    pub async fn update(&mut self) -> ApiResult<()> {
        let (entries, counters) = fetch_dict(&self.url).await?;
        self.entries = entries;
        self.counters = counters;
        self.last_update = Utc::now();
        Ok(())
    }
}

async fn fetch_dict(url: &str) -> ApiResult<(HashMap<u32, Entry>, Counters)> {
    let response = reqwest::get(url).await?.text().await?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(response.as_bytes());

    let mut entries = HashMap::new();

    let mut records = reader.records();
    let _header = records.next();

    println!("Reading counters");
    let r_counters = records.next().ok_or(ApiError::MissingDictHeaders)??;
    println!("{:?}", r_counters);

    fn read_int(r: &StringRecord, index: usize) -> Result<u32, ApiError> {
        Ok(r.get(index)
            .ok_or(ApiError::MissingDictHeaders)?
            .replace(".", "")
            .parse()?)
    }

    let raw_counters = RawCounters {
        total: read_int(&r_counters, 0)?,
        sem: read_int(&r_counters, 1)?,
        lat: read_int(&r_counters, 8)?,
        iro: read_int(&r_counters, 9)?,
        por: read_int(&r_counters, 10)?,
        spa: read_int(&r_counters, 11)?,
        cat: read_int(&r_counters, 12)?,
        occ: read_int(&r_counters, 13)?,
        fra: read_int(&r_counters, 14)?,
        srd: read_int(&r_counters, 15)?,
        ita: read_int(&r_counters, 16)?,
        rom: read_int(&r_counters, 17)?,
        eng: read_int(&r_counters, 18)?,
        fol: read_int(&r_counters, 19)?,
        frk: read_int(&r_counters, 20)?,
        sla: read_int(&r_counters, 21)?,
    };
    let counters = Counters::from(raw_counters);
    println!("{:?}", counters);

    let _ = records.next();

    println!("Reading entries");

    for r in records {
        if let Ok(r) = r {
            if let Ok(r) = r.deserialize::<RawEntry>(None) {
                if let Ok(r) = Entry::try_from(r) {
                    entries.insert(r.id, r);
                }
            }
        }
    }

    println!("Entries count {}", entries.len());
    Ok((entries, counters))
}

#[derive(Deserialize, Debug)]
pub struct RawEntry {
    pub id: u32,
    pub sem: Option<u32>,
    pub category: Option<String>,
    pub topic: Option<String>,
    pub sub_topic: Option<String>,
    pub sub_sub_topic: Option<String>,
    pub essential_flag: Option<String>,
    pub basic_flag: Option<String>,
    pub lat: Option<String>,
    pub iro: Option<String>,
    pub por: Option<String>,
    pub spa: Option<String>,
    pub cat: Option<String>,
    pub occ: Option<String>,
    pub fra: Option<String>,
    pub srd: Option<String>,
    pub ita: Option<String>,
    pub rom: Option<String>,
    pub eng: Option<String>,
    pub fol: Option<String>,
    pub frk: Option<String>,
    pub sla: Option<String>,
}

impl TryFrom<RawEntry> for Entry {
    type Error = ApiError;

    fn try_from(r: RawEntry) -> Result<Self, Self::Error> {
        let mut data = EnumMap::default();
        data[LanguageCode::NeoLatino] = r.lat;
        data[LanguageCode::InterRomanico] = r.iro;
        data[LanguageCode::Portuguese] = r.por;
        data[LanguageCode::Spanish] = r.spa;
        data[LanguageCode::Catalan] = r.cat;
        data[LanguageCode::Occitan] = r.occ;
        data[LanguageCode::French] = r.fra;
        data[LanguageCode::Sardinian] = r.srd;
        data[LanguageCode::Italian] = r.ita;
        data[LanguageCode::Romanian] = r.rom;
        data[LanguageCode::English] = r.eng;
        data[LanguageCode::Folksprak] = r.fol;
        data[LanguageCode::Frenkisch] = r.frk;
        data[LanguageCode::InterSlavic] = r.sla;

        Ok(Entry { id: r.id, data })
    }
}

#[derive(Deserialize, Debug)]
pub struct RawCounters {
    pub total: u32,
    pub sem: u32,
    pub lat: u32,
    pub iro: u32,
    pub por: u32,
    pub spa: u32,
    pub cat: u32,
    pub occ: u32,
    pub fra: u32,
    pub srd: u32,
    pub ita: u32,
    pub rom: u32,
    pub eng: u32,
    pub fol: u32,
    pub frk: u32,
    pub sla: u32,
}

impl From<RawCounters> for Counters {
    fn from(r: RawCounters) -> Self {
        let mut lang = EnumMap::default();
        lang[LanguageCode::NeoLatino] = r.lat;
        lang[LanguageCode::InterRomanico] = r.iro;
        lang[LanguageCode::Portuguese] = r.por;
        lang[LanguageCode::Spanish] = r.spa;
        lang[LanguageCode::Catalan] = r.cat;
        lang[LanguageCode::Occitan] = r.occ;
        lang[LanguageCode::French] = r.fra;
        lang[LanguageCode::Sardinian] = r.srd;
        lang[LanguageCode::Italian] = r.ita;
        lang[LanguageCode::Romanian] = r.rom;
        lang[LanguageCode::English] = r.eng;
        lang[LanguageCode::Folksprak] = r.fol;
        lang[LanguageCode::Frenkisch] = r.frk;
        lang[LanguageCode::InterSlavic] = r.sla;

        Counters {
            total: r.total,
            sem_total: r.sem,
            lang_total: lang,
        }
    }
}
