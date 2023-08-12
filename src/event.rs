
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
    entrant::*,
    phase::*,
    phase_group::*,
    tournament::*,
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGEvents {
    pub nodes: Vec<GGEvent>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GGEvent {
    
    #[serde(rename(serialize = "checkInBuffer",             deserialize = "checkInBuffer"))]
    pub check_in_buffer:            Option<i64>,

    #[serde(rename(serialize = "checkInDuration",           deserialize = "checkInDuration"))]
    pub check_in_duration:          Option<i64>,

    #[serde(rename(serialize = "checkInEnabled",            deserialize = "checkInEnabled"))]
    pub check_in_enabled:           Option<bool>,
    
    #[serde(rename(serialize = "competitionTier",           deserialize = "competitionTier"))]
    pub competition_tier:           Option<i64>,
    
    #[serde(rename(serialize = "createdAt",                 deserialize = "createdAt"))]
    pub created_at:                 Option<i64>,

    #[serde(rename(serialize = "deckSubmissionDeadline",    deserialize = "deckSubmissionDeadline"))]
    pub deck_submission_deadline:   Option<i64>,
    pub entrants:                   Option<GGEntrants>,

    #[serde(rename(serialize = "hasDecks",                  deserialize = "hasDecks"))]
    pub has_decks:                  Option<bool>,

    #[serde(rename(serialize = "hasTasks",                  deserialize = "hasTasks"))]
    pub has_tasks:                  Option<bool>,
    pub id:                         Option<i64>,
    // pub images:                     Option<Vec<GGImage>>,

    #[serde(rename(serialize = "isOnline",                  deserialize = "isOnline"))]
    pub is_online:                  Option<bool>,
    // pub league:                     Option<GGLeague>,

    #[serde(rename(serialize = "matchRulesMarkdown",        deserialize = "matchRulesMarkdown"))]
    pub match_rules_markdown:       Option<String>,
    pub name:                       Option<String>,

    #[serde(rename(serialize = "numEntrants",               deserialize = "numEntrants"))]
    pub num_entrants:               Option<i64>,
    
    #[serde(rename(serialize = "phaseGroups",               deserialize = "phaseGroups"))]
    pub phase_groups:               Option<Vec<GGPhaseGroup>>,
    pub phases:                     Option<Vec<GGPhase>>,
    
    // #[serde(rename(serialize = "prizingInfo",               deserialize = "prizingInfo"))]
    // pub prizing_info: JSON,
    // pub publishing: JSON,

    #[serde(rename(serialize = "rulesMarkdown",             deserialize = "rulesMarkdown"))]
    pub rules_markdown:             Option<String>,

    #[serde(rename(serialize = "rulesetId",                 deserialize = "rulesetId"))]
    pub ruleset_id:                 Option<i64>,
    // pub sets:                       Option<GGSets>,
    pub slug:                       Option<String>,
    // pub standings:                  Option<GGStandings>,

    #[serde(rename(serialize = "startAt",                   deserialize = "startAt"))]
    pub start_at:                   Option<i64>,
    pub state:                      Option<i64>,
    // pub stations:                   Option<GGStations>,

    #[serde(rename(serialize = "teamManagementDeadline",    deserialize = "teamManagementDeadline"))]
    pub team_management_deadline:   Option<i64>,

    #[serde(rename(serialize = "teamNameAllowed",           deserialize = "teamNameAllowed"))]
    pub team_name_allowed:          Option<bool>,

    // #[serde(rename(serialize = "teamRosterSize",            deserialize = "teamRosterSize"))]
    // pub team_roster_size:           Option<GGTeamRosterSize>,
    pub tournament:                 Option<Box<GGTournament>>,
    pub r#type:                     Option<i64>,
    
    #[serde(rename(serialize = "updatedAt",                 deserialize = "updatedAt"))]
    pub updated_at:                 Option<i64>,
    
    #[serde(rename(serialize = "useEventSeeds",             deserialize = "useEventSeeds"))]
    pub use_event_seeds:            Option<bool>,

    #[serde(rename(serialize = "userEntrant",               deserialize = "userEntrant"))]
    pub user_entrant:               Option<Box<GGEntrant>>,
    // pub videogame:                  Option<GGVideogame>,
    // pub waves:                      Option<Vec<GGWave>>,
}

impl GGEvent {

    /// Returns the check in buffer of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn check_in_buffer(&self) -> i64 {
        let mut result: i64 = 0;
        if self.check_in_buffer.is_some() {
            result = self.check_in_buffer.unwrap().clone();
        }
        return result;
    }
    
    /// Returns the check in duration of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn check_in_duration(&self) -> i64 {
        let mut result: i64 = 0;
        if self.check_in_duration.is_some() {
            result = self.check_in_duration.unwrap().clone();
        }
        return result;
    }

    /// Returns if the event has check in enabled.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn check_in_enabled(&self) -> bool {
        let mut result: bool = false;
        if self.check_in_enabled.is_some() {
            result = self.check_in_enabled.unwrap().clone();
        }
        return result;
    }

    /// Returns the competition tier of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn competition_tier(&self) -> i64 {
        let mut result: i64 = 0;
        if self.competition_tier.is_some() {
            result = self.competition_tier.unwrap().clone();
        }
        return result;
    }

    /// Returns the time the event was created.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn created_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.created_at.is_some() {
            result = self.created_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the deck submission deadline of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn deck_submission_deadline(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.deck_submission_deadline.is_some() {
            result = self.deck_submission_deadline.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the entrants in the event.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn entrants(&self) -> Vec<GGEntrant> {
        let mut result: Vec<GGEntrant> = Vec::new();
        if self.entrants.is_some() {
            for entrant in &self.entrants.as_ref().unwrap().nodes {
                result.push(entrant.clone());
            }
        }
        return result;
    }

    /// Returns if the event has decks.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn has_decks(&self) -> bool {
        let mut result: bool = false;
        if self.has_decks.is_some() {
            result = self.has_decks.unwrap().clone();
        }
        return result;
    }

    /// Returns if the event has tasks.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn has_tasks(&self) -> bool {
        let mut result: bool = false;
        if self.has_tasks.is_some() {
            result = self.has_tasks.unwrap().clone();
        }
        return result;
    }

    /// Returns the id of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.id.is_some() {
            result = self.id.unwrap().clone();
        }
        return result;
    }

    /// Returns if the event is online.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn is_online(&self) -> bool {
        let mut result: bool = false;
        if self.is_online.is_some() {
            result = self.is_online.unwrap().clone();
        }
        return result;
    }

    /// Returns the match rules of the event.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn match_rules_markdown(&self) -> String {
        let mut result: String = "".to_string();
        if self.match_rules_markdown.is_some() {
            result = self.match_rules_markdown.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the name of the event.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn name(&self) -> String {
        let mut result: String = "".to_string();
        if self.name.is_some() {
            result = self.name.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the number of entrants in the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn num_entrants(&self) -> i64 {
        let mut result: i64 = 0;
        if self.num_entrants.is_some() {
            result = self.num_entrants.unwrap().clone();
        }
        return result;
    }

    /// Returns the phase groups in the event.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn phase_groups(&self) -> Vec<GGPhaseGroup> {
        let mut result: Vec<GGPhaseGroup> = Vec::new();
        if self.phase_groups.is_some() {
            for phase_group in self.phase_groups.as_ref().unwrap() {
                result.push(phase_group.clone());
            }
        }
        return result;
    }

    /// Returns the phases in the event.
    ///
    /// Returns an empty vector if not set or wasn't queried.
    pub fn phases(&self) -> Vec<GGPhase> {
        let mut result: Vec<GGPhase> = Vec::new();
        if self.phases.is_some() {
            for phase in self.phases.as_ref().unwrap() {
                result.push(phase.clone());
            }
        }
        return result;
    }

    /// Returns the rules of the event.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn rules_markdown(&self) -> String {
        let mut result: String = "".to_string();
        if self.rules_markdown.is_some() {
            result = self.rules_markdown.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the ruleset id of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn ruleset_id(&self) -> i64 {
        let mut result: i64 = 0;
        if self.ruleset_id.is_some() {
            result = self.ruleset_id.unwrap().clone();
        }
        return result;
    }

    /// Returns the slug of the event.
    ///
    /// Returns an empty string if not set or wasn't queried.
    pub fn slug(&self) -> String {
        let mut result: String = "".to_string();
        if self.slug.is_some() {
            result = self.slug.clone().unwrap().clone();
        }
        return result;
    }

    /// Returns the start time of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn start_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.start_at.is_some() {
            result = self.start_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns the state of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn state(&self) -> i64 {
        let mut result: i64 = 0;
        if self.state.is_some() {
            result = self.state.unwrap().clone();
        }
        return result;
    }

    /// Returns the team management deadline of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn team_management_deadline(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.team_management_deadline.is_some() {
            result = self.team_management_deadline.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns if the event allows team names.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn team_name_allowed(&self) -> bool {
        let mut result: bool = false;
        if self.team_name_allowed.is_some() {
            result = self.team_name_allowed.unwrap().clone();
        }
        return result;
    }

    /// Returns the tournament the event is in.
    ///
    /// Returns an empty tournament if not set or wasn't queried.
    pub fn tournament(&self) -> GGTournament {
        let mut result: GGTournament = Default::default();
        if self.tournament.is_some() {
            result = *self.tournament.as_ref().unwrap().clone();
        }
        return result;
    }

    /// Returns the type of the event.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn r#type(&self) -> i64 {
        let mut result: i64 = 0;
        if self.r#type.is_some() {
            result = self.r#type.unwrap().clone();
        }
        return result;
    }

    /// Returns the time the event was last updated.
    ///
    /// Returns zero if not set or wasn't queried.
    pub fn updated_at(&self) -> DateTime<Utc> {
        let mut result: i64 = 0;
        if self.updated_at.is_some() {
            result = self.updated_at.unwrap().clone();
        }
        return Utc.timestamp_opt(result, 0).unwrap();
    }

    /// Returns if the event uses event seeds.
    ///
    /// Returns false if not set or wasn't queried.
    pub fn use_event_seeds(&self) -> bool {
        let mut result: bool = false;
        if self.use_event_seeds.is_some() {
            result = self.use_event_seeds.unwrap().clone();
        }
        return result;
    }

    /// Returns an entrant for a specific user in the event.
    ///
    /// Returns an empty entrant if not set or wasn't queried.
    pub fn user_entrant(&self) -> GGEntrant {
        let mut result: GGEntrant = Default::default();
        if self.user_entrant.is_some() {
            result = *self.user_entrant.as_ref().unwrap().clone();
        }
        return result;
    }

}
