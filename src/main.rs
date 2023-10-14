use std::{fmt, fs, error::Error};

#[derive(Debug)]
/// The type returned in the event of a parse error.
enum ParseError {
    /// Indicating that the given attribute hasn't been found.
    AttributeNotFound(String),

    /// Indicate that a value failed to parse.
    ValueError(String),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::AttributeNotFound(attribute) => format!("attribute \"{attribute}\" not found"),
            Self::ValueError(reason) => reason.to_string(),
        };

        write!(f, "error while parsing: {reason}")
    }
}

#[derive(Debug)]
enum ChargeState {
    Charging,
    Discharging,
}

impl fmt::Display for ChargeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match &self {
            ChargeState::Charging => "charging",
            ChargeState::Discharging => "discharging",
        };
        write!(f, "{state}")
    }
}
impl TryFrom<&str> for ChargeState {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Charging" => Ok(Self::Charging),
            "Discharging" => Ok(Self::Discharging),
            _ => Err(ParseError::ValueError(format!(
                "failed to parse \"{value}\", expected either \"Charging\" or \"Discharging\""
            ))),
        }
    }
}

#[derive(Debug)]
struct BatteryStatus {
    charge_state: ChargeState,
    voltage: f32,
    energy: f32,
    capacity: usize,
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SoC: {}%, {}Wh, {} V",
            self.capacity, self.energy, self.voltage
        )
    }
}

fn extract_value_for<'a>(needle: &'a str, haystack: &'a str) -> Result<&'a str, ParseError> {
    let line = haystack
        .lines()
        .find(|line| line.contains(needle))
        .ok_or(ParseError::AttributeNotFound(needle.to_owned()))?;
    let elements: Vec<&str> = line.split('=').collect();
    Ok(elements[1])
}

impl TryFrom<String> for BatteryStatus {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let capacity: usize = extract_value_for("POWER_SUPPLY_CAPACITY", &value)?
            .parse()
            .map_err(|_| ParseError::ValueError(format!("failed to parse \"{value}\" as f32")))?;

        let voltage: f32 = extract_value_for("POWER_SUPPLY_VOLTAGE_NOW", &value)?
            .parse::<f32>()
            .map_err(|_| ParseError::ValueError(format!("failed to parse \"{value}\" as f32")))?
            / 1_000_000.0;

        let energy: f32 = extract_value_for("POWER_SUPPLY_ENERGY_NOW", &value)?
            .parse::<f32>()
            .map_err(|_| ParseError::ValueError(format!("failed to parse \"{value}\" as f32")))?
            / 1_000_000.0;

        let charge_state: ChargeState =
            extract_value_for("POWER_SUPPLY_STATUS", &value)?.try_into()?;

        Ok(BatteryStatus {
            capacity,
            voltage,
            energy,
            charge_state,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("/sys/class/power_supply/BAT0/uevent")?;
    let status: BatteryStatus = content.try_into()?;

    println!("{status}");

    Ok(())
}
