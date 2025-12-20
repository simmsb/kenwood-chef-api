pub mod values;
pub use values::KnownOptions;

use chrono::{DateTime, Utc};
use dioxus_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct IngredientAllowedUnit {
    pub id: String,
    pub name: String,

    pub abbreviation: Option<String>,

    pub dimension: Option<String>,
}

impl IngredientAllowedUnit {
    pub fn as_reference_unit(self) -> ReferenceUnit {
        ReferenceUnit {
            id: self.id,
            abbreviation: self.abbreviation.unwrap_or_else(String::new),
            name: self.name,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct Ingredient {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct Author {
    pub image: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct ForkedIntoOtherLocale {
    pub id: String,
    pub locale: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct ReferenceUnit {
    pub abbreviation: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct ReferencePreparation {
    pub id: String,
    pub name: String,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct RecipeIngredient {
    pub quantity: Quantity,
    pub reference_ingredient: Ingredient,

    #[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_preparations: Vec<ReferencePreparation>,
    pub source_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct ReferenceTag {
    pub category: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct CapabilityPhase {
    pub can_follow_phases: Vec<String>,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct ReferenceCapability {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, strum::Display, strum::EnumString)]
pub enum ReferenceSettingId {
    #[serde(rename = "kitchenos:Kenwood:KeepWarmSetting")]
    KeepWarm,

    #[serde(
        rename = "kitchenos:Kenwood:TemperatureSetting",
        alias = "cckg:InternalTemperatureSetting",
        alias = "cckg:TemperatureSetting"
    )]
    Temperature,

    #[serde(rename = "kitchenos:Kenwood:SpeedSetting")]
    Speed,

    #[serde(rename = "kitchenos:Kenwood:TimeSetting", alias = "cckg:TimeSetting")]
    Time,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct ReferenceSetting {
    pub id: ReferenceSettingId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct ReferenceValue {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
#[serde(tag = "type")]
pub enum SettingValue {
    #[serde(rename = "numeric")]
    Numeric {
        reference_unit: Option<ReferenceUnit>,
        text: String,
        value: f64,
    },

    #[serde(rename = "boolean")]
    Boolean { text: String, value: bool },

    #[serde(rename = "nominal")]
    Nominal {
        text: String,
        reference_value: ReferenceValue,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct CapabilitySetting {
    pub reference_setting: ReferenceSetting,
    pub value: SettingValue,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct StepCapability {
    pub phase: CapabilityPhase,
    pub reference_capability: ReferenceCapability,

    #[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub settings: Vec<CapabilitySetting>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct Quantity {
    pub amount: Option<f64>,
    pub reference_unit: ReferenceUnit,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct StepIngredient {
    pub ingredient_idx: u8,
    pub quantity: Quantity,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct RecipeStep {
    pub capability: Option<StepCapability>,

    #[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredients: Vec<StepIngredient>,

    pub source_text: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct Recipe {
    pub author: Author,
    pub created_at: DateTime<Utc>,
    pub created_by_id: String,
    pub description: String,
    pub etag: String,
    pub forked_into_other_locales: Vec<ForkedIntoOtherLocale>,
    pub id: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub locale: String,
    pub modified_at: DateTime<Utc>,
    pub name: String,
    pub organization_id: String,
    pub published_at: DateTime<Utc>,
    pub reference_tags: Vec<ReferenceTag>,
    pub serves: u8,
    pub state: String,
    pub steps: Vec<RecipeStep>,
    #[serde(with = "span_field_wise")]
    pub total_time: jiff::SignedDuration,
    pub visibility: String,

    pub cook_time: Option<jiff::SignedDuration>,

    pub prep_time: Option<jiff::SignedDuration>,

    pub referenced: Option<bool>,

    pub requester_role: Option<String>,
}

pub mod span_field_wise {
    use jiff::{SignedDuration, Span, SpanRelativeTo};
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(date: &SignedDuration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Span::try_from(*date).unwrap().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SignedDuration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let span = Span::deserialize(deserializer)?;

        Ok(span
            .to_duration(SpanRelativeTo::days_are_24_hours())
            .unwrap())
    }
}

pub mod span_field_wise_opt {
    use jiff::{SignedDuration, Span, SpanRelativeTo};
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(date: &Option<SignedDuration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        date.map(|x| Span::try_from(x).unwrap())
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SignedDuration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<Span>::deserialize(deserializer)?
            .map(|x| x.to_duration(SpanRelativeTo::days_are_24_hours()).unwrap()))
    }
}

pub mod traits {
    pub use super::{
        AuthorStoreExt, CapabilityPhaseStoreExt, CapabilitySettingStoreExt,
        ForkedIntoOtherLocaleStoreExt, IngredientAllowedUnitStoreExt, IngredientStoreExt,
        QuantityStoreExt, RecipeIngredientStoreExt, RecipeStepStoreExt, RecipeStoreExt,
        ReferenceCapabilityStoreExt, ReferencePreparationStoreExt, ReferenceSettingStoreExt,
        ReferenceTagStoreExt, ReferenceUnitStoreExt, ReferenceValueStoreExt, SettingValueStoreExt,
        StepCapabilityStoreExt, StepIngredientStoreExt,
    };
}
