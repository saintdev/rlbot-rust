use crate::ffi;

impl ffi::LiveDataPacket {
    /// Yields the [`PlayerInfo`](ffi::PlayerInfo) for each player in the match.
    pub fn cars(&self) -> impl Iterator<Item = &ffi::PlayerInfo> {
        self.gameCars.iter().take(self.numCars as usize)
    }

    /// Get the scores for the blue and orange teams, in that order.
    ///
    /// RLBot doesn't seem to return this info(?) so instead we compute it
    /// manually (and inaccurately) by adding up the goals for each individual
    /// player.
    pub fn match_score(&self) -> [i32; 2] {
        let mut result = [0, 0];
        for car in self.cars() {
            result[car.team as usize] += car.score.goals;
        }
        result
    }
}

impl ffi::MatchSettings {
    #[doc(hidden)]
    #[deprecated(note = "this method has been renamed to `rlbot_vs_allstar`")]
    pub fn simple_1v1(rlbot_name: &str, allstar_name: &str) -> Self {
        Self::rlbot_vs_allstar(rlbot_name, allstar_name)
    }

    /// Create a `MatchSettings` for a 1v1 game with Team Blue as an
    /// RLBot-controlled bot, and Team Orange as a Psyonix all-star bot.
    pub fn rlbot_vs_allstar(rlbot_name: &str, allstar_name: &str) -> Self {
        let mut result = ffi::MatchSettings {
            numPlayers: 2,
            ..Default::default()
        };

        result.playerConfiguration[0].bot = true;
        result.playerConfiguration[0].rlbotControlled = true;
        result.playerConfiguration[0].set_name(rlbot_name);

        result.playerConfiguration[1].bot = true;
        result.playerConfiguration[1].botSkill = 1.0;
        result.playerConfiguration[1].set_name(allstar_name);
        result.playerConfiguration[1].team = 1;

        result
    }

    /// Create a `MatchSettings` for a 1v1 game with two Psyonix all-star bots.
    pub fn allstar_vs_allstar(blue_name: &str, orange_name: &str) -> Self {
        let mut result = ffi::MatchSettings {
            numPlayers: 2,
            ..Default::default()
        };

        result.playerConfiguration[0].bot = true;
        result.playerConfiguration[0].botSkill = 1.0;
        result.playerConfiguration[0].set_name(blue_name);

        result.playerConfiguration[1].bot = true;
        result.playerConfiguration[1].botSkill = 1.0;
        result.playerConfiguration[1].set_name(orange_name);
        result.playerConfiguration[1].team = 1;

        result
    }
}

impl ffi::PlayerConfiguration {
    /// Populate the `Name` field from a string.
    pub fn set_name(&mut self, name: &str) {
        for (i, cp) in name.encode_utf16().enumerate() {
            self.name[i] = cp.into();
        }
    }
}
