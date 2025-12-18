use crate::{
    CapabilityPhase, CapabilitySetting, ReferenceCapability, ReferenceSetting, ReferenceSettingId,
    ReferenceValue, SettingValue,
};

pub trait KnownOptions
where
    Self: Sized,
{
    fn known_options() -> Vec<Self>;
}

impl KnownOptions for CapabilityPhase {
    fn known_options() -> Vec<CapabilityPhase> {
        vec![
            CapabilityPhase {
                id: "cckg:ExecutionPhase".to_owned(),
                name: "run".to_owned(),
                can_follow_phases: vec![
                    "cckg:PreheatInitializationPhase".to_owned(),
                    "cckg:SetUpPhase".to_owned(),
                ],
            },
            CapabilityPhase {
                id: "cckg:PreheatInitializationPhase".to_owned(),
                name: "preheat".to_owned(),
                can_follow_phases: vec!["cckg:SetUpPhase".to_owned()],
            },
            CapabilityPhase {
                id: "cckg:SetUpPhase".to_owned(),
                name: "set up".to_owned(),
                can_follow_phases: vec!["cckg:PreheatInitializationPhase".to_owned()],
            },
            CapabilityPhase {
                id: "cckg:TakeDownPhase".to_owned(),
                name: "take down".to_owned(),
                can_follow_phases: vec![
                    "cckg:ExecutionPhase".to_owned(),
                    "cckg:UpdatePhase".to_owned(),
                ],
            },
            CapabilityPhase {
                id: "cckg:UpdatePhase".to_owned(),
                name: "modify".to_owned(),
                can_follow_phases: vec![
                    "cckg:ExecutionPhase".to_owned(),
                    "cckg:UpdatePhase".to_owned(),
                ],
            },
        ]
    }
}

impl KnownOptions for ReferenceCapability {
    fn known_options() -> Vec<ReferenceCapability> {
        vec![
            ReferenceCapability {
                id: "cckg:Bake".to_owned(),
                name: "bake".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Beat".to_owned(),
                name: "beat".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Cook".to_owned(),
                name: "cook".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Knead".to_owned(),
                name: "knead".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Mix".to_owned(),
                name: "mix".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Prove".to_owned(),
                name: "proof".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Steam".to_owned(),
                name: "steam".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Stir".to_owned(),
                name: "stir".to_owned(),
            },
            ReferenceCapability {
                id: "cckg:Whisk".to_owned(),
                name: "whisk".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:AssistedStir".to_owned(),
                name: "Assisted Stir".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Beat".to_owned(),
                name: "Beat".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Bechamel1".to_owned(),
                name: "Bechamel 1".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Bechamel2".to_owned(),
                name: "Bechamel 2".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Blender".to_owned(),
                name: "Blender".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ChopHerbs".to_owned(),
                name: "Chop Herbs".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ChopMeat".to_owned(),
                name: "Chop Meat".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ChopNuts".to_owned(),
                name: "Chop Nuts".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ChopVegetables".to_owned(),
                name: "Chop Vegetables".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:CoarseBlend".to_owned(),
                name: "Coarse Blend".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Cook".to_owned(),
                name: "Cook".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Dicing".to_owned(),
                name: "Dicing".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:DoughKneading".to_owned(),
                name: "Dough Kneading".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:DoughProving".to_owned(),
                name: "Dough Proving".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:FettuccinePasta".to_owned(),
                name: "Fettuccine Pasta".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:FoodMill".to_owned(),
                name: "Food Mill".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:FoodMincer".to_owned(),
                name: "Food Mincer".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:FoodProcessor".to_owned(),
                name: "Food Processor".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:FrozenDessertMaker".to_owned(),
                name: "Frozen Dessert Maker".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:GrindPastes".to_owned(),
                name: "Grind Pastes".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:GrindSpices".to_owned(),
                name: "Grind Spices".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:IceCrush".to_owned(),
                name: "Ice Crush".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Knead".to_owned(),
                name: "Knead".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:LasagnaPastaRoller".to_owned(),
                name: "Lasagna Pasta Roller".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:MeatBrowning1".to_owned(),
                name: "Meat Browning 1".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:MeatBrowning2".to_owned(),
                name: "Meat Browning 2".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:MiniChopper".to_owned(),
                name: "Mini Chopper".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Mix".to_owned(),
                name: "Mix".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Popcorn".to_owned(),
                name: "Popcorn".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Prove".to_owned(),
                name: "Prove".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:PulseBlender".to_owned(),
                name: "Blender Pulsing".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:PulseFoodProc".to_owned(),
                name: "Food Processor Pulsing".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Puree".to_owned(),
                name: "Puree".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Roux".to_owned(),
                name: "Roux".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ShallowFry1".to_owned(),
                name: "Shallow Fry 1".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ShallowFry2".to_owned(),
                name: "Shallow Fry 2".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ShortCrustPastry1".to_owned(),
                name: "Short Crust Pastry 1".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:ShortCrustPastry2".to_owned(),
                name: "Short Crust Pastry 2".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:SlicingGratingFirmIngredients".to_owned(),
                name: "Slicing Grating Firm Ingredients".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:SlicingGratingSoftIngredients".to_owned(),
                name: "Slicing Grating Soft Ingredients".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:SmoothBlend".to_owned(),
                name: "Smooth Blend".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:SpaghettiPasta".to_owned(),
                name: "Spaghetti Pasta".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Steam".to_owned(),
                name: "Steam".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Stir".to_owned(),
                name: "Stir".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:StirAndWhisk".to_owned(),
                name: "Stir + Whisk".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:VegetableSaute1".to_owned(),
                name: "Vegetable Sauté 1".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:VegetableSaute2".to_owned(),
                name: "Vegetable Sauté 2".to_owned(),
            },
            ReferenceCapability {
                id: "kitchenos:Kenwood:Whisk".to_owned(),
                name: "Whisk".to_owned(),
            },
        ]
    }
}

impl ReferenceValue {
    pub fn stir_settings() -> Vec<ReferenceValue> {
        vec![
            ReferenceValue {
                id: "kitchenos:Kenwood:Speed1".to_owned(),
                name: "Speed level 1".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:Speed2".to_owned(),
                name: "Speed level 2".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:Speed3".to_owned(),
                name: "Speed level 3".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:Speed4".to_owned(),
                name: "Speed level 4".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:Speed5".to_owned(),
                name: "Speed level 5".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:Speed6".to_owned(),
                name: "Speed level 6".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:SpeedMax".to_owned(),
                name: "Max".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:SpeedMin".to_owned(),
                name: "Min".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:SpeedOff".to_owned(),
                name: "No stir".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:SpeedStir1".to_owned(),
                name: "Stir 1".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:SpeedStir2".to_owned(),
                name: "Stir 2".to_owned(),
            },
            ReferenceValue {
                id: "kitchenos:Kenwood:SpeedStir3".to_owned(),
                name: "Stir 3".to_owned(),
            },
            ReferenceValue {
                name: "Stir 4".to_owned(),
                id: "kitchenos:Kenwood:SpeedStir4".to_owned(),
            },
        ]
    }

    pub fn temperature_off() -> ReferenceValue {
        ReferenceValue {
            id: "kitchenos:Kenwood:TemperatureOff".to_owned(),
            name: "No temperature".to_owned(),
        }
    }
}

impl CapabilitySetting {
    pub fn keep_warm_default() -> Self {
        Self {
            reference_setting: ReferenceSettingId::KeepWarm.reference_setting(),
            value: SettingValue::Boolean {
                text: "placeholder".to_owned(),
                value: false,
            },
        }
    }
}

impl ReferenceSettingId {
    pub fn reference_setting(self) -> ReferenceSetting {
        ReferenceSetting {
            id: self,
            name: match self {
                ReferenceSettingId::KeepWarm => "Keep warm",
                ReferenceSettingId::Temperature => "Temperature",
                ReferenceSettingId::Speed => "Speed",
                ReferenceSettingId::Time => "Time",
            }
            .to_owned(),
        }
    }
}
