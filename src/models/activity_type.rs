/*
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ActivityType : An enumeration of the types an activity may have. Note that this enumeration does not include new sport types (e.g. MountainBikeRide, EMountainBikeRide), activities with these sport types will have the corresponding activity type (e.g. Ride for MountainBikeRide, EBikeRide for EMountainBikeRide)
/// An enumeration of the types an activity may have. Note that this enumeration does not include new sport types (e.g. MountainBikeRide, EMountainBikeRide), activities with these sport types will have the corresponding activity type (e.g. Ride for MountainBikeRide, EBikeRide for EMountainBikeRide)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActivityType {
    #[serde(rename = "AlpineSki")]
    AlpineSki,
    #[serde(rename = "BackcountrySki")]
    BackcountrySki,
    #[serde(rename = "Canoeing")]
    Canoeing,
    #[serde(rename = "Crossfit")]
    Crossfit,
    #[serde(rename = "EBikeRide")]
    EBikeRide,
    #[serde(rename = "Elliptical")]
    Elliptical,
    #[serde(rename = "Golf")]
    Golf,
    #[serde(rename = "Handcycle")]
    Handcycle,
    #[serde(rename = "Hike")]
    Hike,
    #[serde(rename = "IceSkate")]
    IceSkate,
    #[serde(rename = "InlineSkate")]
    InlineSkate,
    #[serde(rename = "Kayaking")]
    Kayaking,
    #[serde(rename = "Kitesurf")]
    Kitesurf,
    #[serde(rename = "NordicSki")]
    NordicSki,
    #[serde(rename = "Ride")]
    Ride,
    #[serde(rename = "RockClimbing")]
    RockClimbing,
    #[serde(rename = "RollerSki")]
    RollerSki,
    #[serde(rename = "Rowing")]
    Rowing,
    #[serde(rename = "Run")]
    Run,
    #[serde(rename = "Sail")]
    Sail,
    #[serde(rename = "Skateboard")]
    Skateboard,
    #[serde(rename = "Snowboard")]
    Snowboard,
    #[serde(rename = "Snowshoe")]
    Snowshoe,
    #[serde(rename = "Soccer")]
    Soccer,
    #[serde(rename = "StairStepper")]
    StairStepper,
    #[serde(rename = "StandUpPaddling")]
    StandUpPaddling,
    #[serde(rename = "Surfing")]
    Surfing,
    #[serde(rename = "Swim")]
    Swim,
    #[serde(rename = "Velomobile")]
    Velomobile,
    #[serde(rename = "VirtualRide")]
    VirtualRide,
    #[serde(rename = "VirtualRun")]
    VirtualRun,
    #[serde(rename = "Walk")]
    Walk,
    #[serde(rename = "WeightTraining")]
    WeightTraining,
    #[serde(rename = "Wheelchair")]
    Wheelchair,
    #[serde(rename = "Windsurf")]
    Windsurf,
    #[serde(rename = "Workout")]
    Workout,
    #[serde(rename = "Yoga")]
    Yoga,

}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AlpineSki => write!(f, "AlpineSki"),
            Self::BackcountrySki => write!(f, "BackcountrySki"),
            Self::Canoeing => write!(f, "Canoeing"),
            Self::Crossfit => write!(f, "Crossfit"),
            Self::EBikeRide => write!(f, "EBikeRide"),
            Self::Elliptical => write!(f, "Elliptical"),
            Self::Golf => write!(f, "Golf"),
            Self::Handcycle => write!(f, "Handcycle"),
            Self::Hike => write!(f, "Hike"),
            Self::IceSkate => write!(f, "IceSkate"),
            Self::InlineSkate => write!(f, "InlineSkate"),
            Self::Kayaking => write!(f, "Kayaking"),
            Self::Kitesurf => write!(f, "Kitesurf"),
            Self::NordicSki => write!(f, "NordicSki"),
            Self::Ride => write!(f, "Ride"),
            Self::RockClimbing => write!(f, "RockClimbing"),
            Self::RollerSki => write!(f, "RollerSki"),
            Self::Rowing => write!(f, "Rowing"),
            Self::Run => write!(f, "Run"),
            Self::Sail => write!(f, "Sail"),
            Self::Skateboard => write!(f, "Skateboard"),
            Self::Snowboard => write!(f, "Snowboard"),
            Self::Snowshoe => write!(f, "Snowshoe"),
            Self::Soccer => write!(f, "Soccer"),
            Self::StairStepper => write!(f, "StairStepper"),
            Self::StandUpPaddling => write!(f, "StandUpPaddling"),
            Self::Surfing => write!(f, "Surfing"),
            Self::Swim => write!(f, "Swim"),
            Self::Velomobile => write!(f, "Velomobile"),
            Self::VirtualRide => write!(f, "VirtualRide"),
            Self::VirtualRun => write!(f, "VirtualRun"),
            Self::Walk => write!(f, "Walk"),
            Self::WeightTraining => write!(f, "WeightTraining"),
            Self::Wheelchair => write!(f, "Wheelchair"),
            Self::Windsurf => write!(f, "Windsurf"),
            Self::Workout => write!(f, "Workout"),
            Self::Yoga => write!(f, "Yoga"),
        }
    }
}

impl Default for ActivityType {
    fn default() -> ActivityType {
        Self::AlpineSki
    }
}

