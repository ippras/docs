//! Fatty acids documents

use crate::asset;

pub const C16: &str = asset!("_new/en/fatty_acids/c16.md");
pub const C18: &str = asset!("_new/en/fatty_acids/c18.md");
pub const C18C9: &str = asset!("_new/en/fatty_acids/c18c9.md");
pub const C18C9C12: &str = asset!("_new/en/fatty_acids/c18c9c12.md");
pub const C18C9C12C15: &str = asset!("_new/en/fatty_acids/c18c9c12c15.md");
pub const C20: &str = asset!("_new/en/fatty_acids/c20.md");
pub const C20C5C8C11C14C17: &str = asset!("_new/en/fatty_acids/c20c5c8c11c14c17.md");
pub const C22: &str = asset!("_new/en/fatty_acids/c22.md");
pub const C22C4C7C10C13C16C19: &str = asset!("_new/en/fatty_acids/c22c4c7c10c13c16c19.md");
pub const C22C13: &str = asset!("_new/en/fatty_acids/c22c13.md");
pub const C24: &str = asset!("_new/en/fatty_acids/c24.md");
pub const C24C15: &str = asset!("_new/en/fatty_acids/c24c15.md");

// By chain length

// pub const SHORT_CHAIN: &str = asset!("en/fatty_acids/ShortChain.md");
// pub const MEDIUM_CHAIN: &str = asset!("en/fatty_acids/MediumChain.md");
// pub const LONG_CHAIN: &str = asset!("en/fatty_acids/LongChain.md");
// pub const VERY_LONG_CHAIN: &str = asset!("en/fatty_acids/VeryLongChain.md");

// By double bounds parity

pub const TRANS: &str = asset!("en/fatty_acids/Trans.md");

// By unsaturated count

pub const MONOUNSATURATED: &str = asset!("en/fatty_acids/Monounsaturated.md");
pub const POLYUNSATURATED: &str = asset!("en/fatty_acids/Polyunsaturated.md");
pub const SATURATED: &str = asset!("en/fatty_acids/Saturated.md");
pub const UNSATURATED: &str = asset!("en/fatty_acids/Unsaturated.md");

// By unsaturated offset

pub const OMEGA_3: &str = asset!("en/fatty_acids/Omega-3.md");
pub const OMEGA_6: &str = asset!("en/fatty_acids/Omega-6.md");
pub const OMEGA_9: &str = asset!("en/fatty_acids/Omega-9.md");

// By unsaturated pattern

pub const CONJUGATED: &str = asset!("en/fatty_acids/Conjugated.md");

// Complex
pub const EICOSAPENTAENOIC_AND_DOCOSAHEXAENOIC: &str = asset!("en/fatty_acids/EicosapentaenoicAndDocosahexaenoic.md");
pub const FISH_LIPID_QUALITY: &str = asset!("en/fatty_acids/FishLipidQuality.md");
pub const HEALTH_PROMOTING_INDEX: &str = asset!("en/fatty_acids/HealthPromotingIndex.md");
pub const HYPOCHOLESTEROLEMIC_TO_HYPERCHOLESTEROLEMIC: &str = asset!("en/fatty_acids/HypocholesterolemicToHypercholesterolemic.md");
pub const INDEX_OF_ATHEROGENICITY: &str = asset!("en/fatty_acids/IndexOfAtherogenicity.md");
pub const INDEX_OF_THROMBOGENICITY: &str = asset!("en/fatty_acids/IndexOfThrombogenicity.md");
pub const LINOLEIC_TO_ALPHA_LINOLENIC: &str = asset!("en/fatty_acids/LinoleicToAlphaLinolenic.md");
pub const POLYUNSATURATED_6_TO_POLYUNSATURATED_3: &str = asset!("en/fatty_acids/Polyunsaturated-6ToPolyunsaturated-3.md");
pub const POLYUNSATURATED_TO_SATURATED: &str = asset!("en/fatty_acids/PolyunsaturatedToSaturated.md");
pub const UNSATURATION_INDEX: &str = asset!("en/fatty_acids/UnsaturationIndex.md");

// Factors
pub const ENRICHMENT_FACTOR: &str = asset!("en/fatty_acids/EnrichmentFactor.md");
pub const SELECTIVITY_FACTOR: &str = asset!("en/fatty_acids/SelectivityFactor.md");

pub mod biodiesel;
