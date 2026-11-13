pub enum Game {
    CounterStrike,
    Valorant,
    Overwatch,
    TheFinals,
    Deadlock,
}

struct GameEntry {
    name: &'static str,
    yaw: f32,
}

impl GameEntry {
    const fn new(name: &'static str, yaw: f32) -> Self {
        Self { name, yaw }
    }
}

impl Game {
    pub const ALL: [Self; 5] = [
        Self::CounterStrike,
        Self::Valorant,
        Self::Overwatch,
        Self::TheFinals,
        Self::Deadlock,
    ];

    const fn entry(&self) -> GameEntry {
        match self {
            Self::CounterStrike => GameEntry::new("Counter-Strike", 0.022),
            Self::Valorant => GameEntry::new("Valorant", 0.07),
            Self::Overwatch => GameEntry::new("Overwatch", 0.0066),
            Self::TheFinals => GameEntry::new("The Finals", 0.0066 / 6.6),
            // TODO:
            Self::Deadlock => GameEntry::new("Deadlock", 1.0),
        }
    }

    pub const fn name(&self) -> &'static str {
        self.entry().name
    }

    const fn yaw(&self) -> f32 {
        self.entry().yaw
    }

    pub fn cm_to_sens(&self, cm: f32, dpi: u32) -> f32 {
        (2.54 * 360.0) / (dpi as f32 * cm * self.yaw())
    }

    pub fn sens_to_cm(&self, sens: f32, dpi: u32) -> f32 {
        (2.54 * 360.0) / (dpi as f32 * sens * self.yaw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cm_to_sens_and_back() {
        let game = Game::CounterStrike;
        let cm = 55.0;
        let dpi = 1600;

        let sens = game.cm_to_sens(cm, dpi);
        let cm_converted_back = game.sens_to_cm(sens, dpi);

        assert!(
            (cm - cm_converted_back).abs() < 1e-4,
            "cm360 to sensitivity and back to cm360 conversion failed"
        );
    }

    #[test]
    fn test_known_values() {
        let game = Game::CounterStrike;
        let cm = 55.0;
        let dpi = 1600;

        let sens = game.cm_to_sens(cm, dpi);
        let expected_sens = 0.47231;

        assert!(
            (sens - expected_sens).abs() < 1e-3,
            "cm360 to sensitivity conversion failed for known value"
        );
    }
}
