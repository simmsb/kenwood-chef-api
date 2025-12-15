use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct IngredientAllowedUnit {
    pub id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Ingredient {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Author {
    pub image: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ForkedIntoOtherLocale {
    pub id: String,
    pub locale: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferenceUnit {
    pub abbreviation: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferenceIngredient {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferencePreparation {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RecipeIngredient {
    pub quantity: Quantity,
    pub reference_ingredient: ReferenceIngredient,
    pub reference_preparations: Option<Vec<ReferencePreparation>>,
    pub source_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferenceTag {
    pub category: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CapabilityPhase {
    pub can_follow_phases: Vec<String>,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferenceCapability {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RecipeStepCapabilitySettingReferenceSetting {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RecipeStepCapabilitySettingValueReferenceValue {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RecipeStepCapabilitySettingValue {
    pub text: String,

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_nit: Option<ReferenceUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value: Option<RecipeStepCapabilitySettingValueReferenceValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CapabilitySetting {
    pub reference_setting: RecipeStepCapabilitySettingReferenceSetting,
    pub value: RecipeStepCapabilitySettingValue,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct StepCapability {
    pub phase: CapabilityPhase,
    pub reference_capability: ReferenceCapability,
    pub settings: Option<Vec<CapabilitySetting>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Quantity {
    pub amount: Option<f64>,
    pub reference_unit: ReferenceUnit,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct StepIngredient {
    pub ingredient_idx: u8,
    pub quantity: Quantity,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RecipeStep {
    pub capability: Option<StepCapability>,
    pub ingredients: Option<Vec<StepIngredient>>,
    pub source_text: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

    #[serde(skip_serializing_if = "Option::is_none", with = "span_field_wise_opt")]
    pub cook_time: Option<jiff::SignedDuration>,

    #[serde(skip_serializing_if = "Option::is_none", with = "span_field_wise_opt")]
    pub prep_time: Option<jiff::SignedDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
