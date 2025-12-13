use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub image: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForkedIntoOtherLocale {
    pub id: String,
    pub locale: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReferenceUnit {
    pub abbreviation: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReferenceIngredient {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReferencePreparation {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecipeIngredient {
    pub quantity: Quantity,
    pub reference_ingredient: ReferenceIngredient,
    pub reference_preparations: Option<Vec<ReferencePreparation>>,
    pub source_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReferenceTag {
    pub category: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CapabilityPhase {
    pub can_follow_phases: Vec<String>,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReferenceCapability {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecipeStepCapabilitySettingReferenceSetting {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecipeStepCapabilitySettingValueReferenceValue {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct CapabilitySetting {
    pub reference_setting: RecipeStepCapabilitySettingReferenceSetting,
    pub value: RecipeStepCapabilitySettingValue,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StepCapability {
    pub phase: CapabilityPhase,
    pub reference_capability: ReferenceCapability,
    pub settings: Option<Vec<CapabilitySetting>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Quantity {
    pub amount: Option<f64>,
    pub reference_unit: ReferenceUnit,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StepIngredient {
    pub ingredient_idx: u8,
    pub quantity: Quantity,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecipeStep {
    pub capability: Option<StepCapability>,
    pub ingredients: Option<Vec<StepIngredient>>,
    pub source_text: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub author: Author,
    pub created_at: DateTime<FixedOffset>,
    pub created_by_id: String,
    pub description: String,
    pub etag: String,
    pub forked_into_other_locales: Vec<ForkedIntoOtherLocale>,
    pub id: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub locale: String,
    pub modified_at: DateTime<FixedOffset>,
    pub name: String,
    pub organization_id: String,
    pub published_at: DateTime<FixedOffset>,
    pub reference_tags: Vec<ReferenceTag>,
    pub serves: u8,
    pub state: String,
    pub steps: Vec<RecipeStep>,
    pub total_time: String,
    pub visibility: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cook_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prep_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_role: Option<String>,
}
