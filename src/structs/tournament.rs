
use chrono::{
    DateTime,
    TimeZone,
    Utc
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    enums::*,
    event::*,
    image::*,
    page_info::*,
    participant::*,
    user::*,
};

/// Equivalent for start.gg TournamentConnection.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGTournamentConnection {
    pub nodes:      Vec<GGTournament>,
    pub page_info:  Option<Box<GGPageInfo>>,
}

impl GGTournamentConnection {

    /// Returns the page info of the connection.
    ///
    /// Returns empty page info if not set or wasn't queried.
    pub fn page_info(&self) -> GGPageInfo {
        let mut result: GGPageInfo = Default::default();
        if self.page_info.is_some() {
            result = *self.page_info.as_ref().unwrap().clone();
        }
        return result;
    }

}

/// Equivalent for start.gg Tournament.
///
/// Each element in the structure is optional, allowing a user to only query values they want.
/// Given each is an option and not a requirement, a method is included for each element with the same name.
/// These methods will unwrap and return the proper value without any unwrapping or references needed.
/// Certain methods (see participants()) will return a vector of the data type instead of a connection to a vector, done to simplify the API and make the start.gg api easier to work with.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGTournament {
    
    #[serde(rename(serialize = "addrState",                 deserialize = "addrState"))]
    pub addr_state:                     Option<String>,
    pub admins:                         Option<Vec<GGUser>>,
    pub city:                           Option<String>,
    
    #[serde(rename(serialize = "countryCode",               deserialize = "countryCode"))]
    pub country_code:                   Option<String>,
    
    #[serde(rename(serialize = "createdAt",                 deserialize = "createdAt"))]
    pub created_at:                     Option<i64>,
    pub currency:                       Option<String>,
    
    #[serde(rename(serialize = "endAt",                     deserialize = "endAt"))]
    pub end_at:                         Option<i64>,

    #[serde(rename(serialize = "eventRegistrationClosesAt", deserialize = "eventRegistrationClosesAt"))]
    pub event_registration_closes_at:   Option<i64>,
    pub events:                         Option<Vec<GGEvent>>,
    
    #[serde(rename(serialize = "hasOfflineEvents",          deserialize = "hasOfflineEvents"))]
    pub has_offline_events:             Option<bool>,

    #[serde(rename(serialize = "hasOnlineEvents",           deserialize = "hasOnlineEvents"))]
    pub has_online_events:              Option<bool>,
    pub hashtag:                        Option<String>,
    pub id:                             Option<GGID>,
    pub images:                         Option<Vec<GGImage>>,
    
    #[serde(rename(serialize = "isOnline",                  deserialize = "isOnline"))]
    pub is_online:                      Option<bool>,
    
    #[serde(rename(serialize = "isRegistrationOpen",        deserialize = "isRegistrationOpen"))]
    pub is_registration_open:           Option<bool>,
    pub lat:                            Option<f64>,
    // pub links
    pub lng:                            Option<f64>,
    
    #[serde(rename(serialize = "mapsPlaceId",               deserialize = "mapsPlaceId"))]
    pub maps_place_id:                  Option<String>,
    pub name:                           Option<String>,
    
    #[serde(rename(serialize = "numAttendees",              deserialize = "numAttendees"))]
    pub num_attendees:                  Option<i64>,
    pub owner:                          Option<Box<GGUser>>,
    pub participants:                   Option<GGParticipantConnection>,
    
    #[serde(rename(serialize = "postalCode",                deserialize = "postalCode"))]
    pub postal_code:                    Option<String>,
    
    #[serde(rename(serialize = "primaryContact",            deserialize = "primaryContact"))]
    pub primary_contact:                Option<String>,
    
    #[serde(rename(serialize = "primaryContactType",        deserialize = "primaryContactType"))]
    pub primary_contact_type:           Option<String>,
    // publishing: JSON
    
    #[serde(rename(serialize = "registrationClosesAt",      deserialize = "registrationClosesAt"))]
    pub registration_closes_at:         Option<i64>,
    pub rules:                          Option<String>,
    
    #[serde(rename(serialize = "shortSlug",                 deserialize = "shortSlug"))]
    pub short_slug:                     Option<String>,
    pub slug:                           Option<String>,
    
    #[serde(rename(serialize = "startAt",                   deserialize = "startAt"))]
    pub start_at:                       Option<i64>,
    pub state:                          Option<i64>,
    // pub stations
    // pub streamQueue
    // pub streams
    
    #[serde(rename(serialize = "teamCreationClosesAt",      deserialize = "teamCreationClosesAt"))]
    pub team_creation_closes_at:        Option<i64>,
    // pub teams
    pub timezone:                       Option<String>,
    
    #[serde(rename(serialize = "tournamentType",            deserialize = "tournamentType"))]
    pub tournament_type:                Option<i64>,
    
    #[serde(rename(serialize = "updatedAt",                 deserialize = "updatedAt"))]
    pub updated_at:                     Option<i64>,
    pub url:                            Option<String>,
    
    #[serde(rename(serialize = "venueAddress",              deserialize = "venueAddress"))]
    pub venue_address:                  Option<String>,
    
    #[serde(rename(serialize = "venueName",                 deserialize = "venueName"))]
    pub venue_name:                     Option<String>,
    // pub waves

}

impl GGTournament {

    /// Returns the state address of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn addr_state(&self) -> String {
        let mut result: String = "".to_string();
        if self.addr_state.is_some() {
            result = self.addr_state.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the admins of the tournament.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn admins(&self) -> Vec<GGUser> {
        let mut result: Vec<GGUser> = Vec::new();
        if self.admins.is_some() {
            for admin in self.admins.as_ref().unwrap() {
                result.push(admin.clone());
            }
        }
        return result;
    }

    /// Returns the city of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn city(&self) -> String {
        let mut result: String = "".to_string();
        if self.city.is_some() {
            result = self.city.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the country code of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn country_code(&self) -> String {
        let mut result: String = "".to_string();
        if self.country_code.is_some() {
            result = self.country_code.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the time the tournament was created.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn created_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.created_at.is_some() {
            result = self.created_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the currency of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn currency(&self) -> String {
        let mut result: String = "".to_string();
        if self.currency.is_some() {
            result = self.currency.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the end time of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn end_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.end_at.is_some() {
            result = self.end_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the event registration end time of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn event_registration_closes_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.event_registration_closes_at.is_some() {
            result = self.event_registration_closes_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the events in the tournament.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn events(&self) -> Vec<GGEvent> {
        let mut result: Vec<GGEvent> = Vec::new();
        if self.events.is_some() {
            for event in self.events.as_ref().unwrap() {
                result.push(event.clone());
            }
        }
        return result;
    }

    /// Returns if the tournament has offline events.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn has_offline_events(&self) -> bool {
        let mut result: bool = false;
        if self.has_offline_events.is_some() {
            result = self.has_offline_events.unwrap().clone();
        }
        return result;
    }

    /// Returns if the tournament has online events.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn has_online_events(&self) -> bool {
        let mut result: bool = false;
        if self.has_online_events.is_some() {
            result = self.has_online_events.unwrap().clone();
        }
        return result;
    }

    /// Returns the hashtag of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn hashtag(&self) -> String {
        let mut result: String = "".to_string();
        if self.hashtag.is_some() {
            result = self.hashtag.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> GGID {
        let mut result: GGID = GGID::Int(0);
        if self.id.is_some() {
            match self.id.clone().unwrap() {
                GGID::Int(_) => result = self.id.as_ref().unwrap().clone(),
                GGID::String(_) => result = self.id.as_ref().unwrap().clone(),
            };
        }
        return result;
    }

    /// Returns the images of the tournament.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn images(&self) -> Vec<GGImage> {
        let mut result: Vec<GGImage> = Vec::new();
        if self.images.is_some() {
            for image in self.images.as_ref().unwrap() {
                result.push(image.clone());
            }
        }
        return result;
    }

    /// Returns if the tournament is online.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_online(&self) -> bool {
        let mut result: bool = false;
        if self.is_online.is_some() {
            result = self.is_online.unwrap().clone();
        }
        return result;
    }

    /// Returns if the tournament registration is open.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_registration_open(&self) -> bool {
        let mut result: bool = false;
        if self.is_registration_open.is_some() {
            result = self.is_registration_open.unwrap().clone();
        }
        return result;
    }

    /// Returns the latitude of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn lat(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.lat.is_some() {
            result = self.lat.unwrap().clone();
        }
        return result;
    }

    /// Returns the longitude of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn lng(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.lng.is_some() {
            result = self.lng.unwrap().clone();
        }
        return result;
    }

    /// Returns the maps place id of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn maps_place_id(&self) -> String {
        let mut result: String = "".to_string();
        if self.maps_place_id.is_some() {
            result = self.maps_place_id.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the name of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the number of attendees at the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn num_attendees(&self) -> i64 {
        let mut result: i64 = 0;
        if self.num_attendees.is_some() {
            result = self.num_attendees.unwrap().clone();
        }
        return result;
    }

    /// Returns the owner of the tournament.
    ///
    /// Returns an empty user if not set or wasn't queried.
    pub fn owner(&self) -> GGUser {
        let mut result: GGUser = Default::default();
        if self.owner.is_some() {
            result = *self.owner.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the participants in the tournament.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn participants(&self) -> Vec<GGParticipant> {
        let mut result: Vec<GGParticipant> = Vec::new();
        if self.participants.is_some() {
            for participant in &self.participants.as_ref().unwrap().nodes {
                result.push(participant.clone());
            }
        }
        return result;
    }

    /// Returns the postal code of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn postal_code(&self) -> String {
        let mut result: String = "".to_string();
        if self.postal_code.is_some() {
            result = self.postal_code.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the primary contact of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn primary_contact(&self) -> String {
        let mut result: String = "".to_string();
        if self.primary_contact.is_some() {
            result = self.primary_contact.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the primary contact type of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn primary_contact_type(&self) -> String {
        let mut result: String = "".to_string();
        if self.primary_contact_type.is_some() {
            result = self.primary_contact_type.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the registration close date of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn registration_closes_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.registration_closes_at.is_some() {
            result = self.registration_closes_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the rules of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn rules(&self) -> String {
        let mut result: String = "".to_string();
        if self.rules.is_some() {
            result = self.rules.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the short slug of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn short_slug(&self) -> String {
        let mut result: String = "".to_string();
        if self.short_slug.is_some() {
            result = self.short_slug.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the slug of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn slug(&self) -> String {
        let mut result: String = "".to_string();
        if self.slug.is_some() {
            result = self.slug.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the start time of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn start_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.start_at.is_some() {
            result = self.start_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the state of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn state(&self) -> i64 {
        let mut result: i64 = 0;
        if self.state.is_some() {
            result = self.state.unwrap().clone();
        }
        return result;
    }

    /// Returns the team creation end date of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn team_creation_closes_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.team_creation_closes_at.is_some() {
            result = self.team_creation_closes_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the timezone of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn timezone(&self) -> String {
        let mut result: String = "".to_string();
        if self.timezone.is_some() {
            result = self.timezone.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the type of the tournament.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn tournament_type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.tournament_type.is_some() {
            result = self.tournament_type.unwrap().clone();
        }
        return result;
    }

    /// Returns the time the tournament was last updated.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn updated_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.updated_at.is_some() {
            result = self.updated_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the url of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn url(&self) -> String {
        let mut result: String = "".to_string();
        if self.url.is_some() {
            result = self.url.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the venue address of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn venue_address(&self) -> String {
        let mut result: String = "".to_string();
        if self.venue_address.is_some() {
            result = self.venue_address.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the venue name of the tournament.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn venue_name(&self) -> String {
        let mut result: String = "".to_string();
        if self.venue_name.is_some() {
            result = self.venue_name.clone().unwrap().clone();
        }
        return result;
    }

}
