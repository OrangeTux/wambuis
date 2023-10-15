use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{error::Error, fmt, fs};

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
/// State of the charge process.
enum ChargeState {
    /// State is not known
    Unknown,

    /// Battery is charging.
    Charging,

    /// Battery is discharging.
    Discharging,
    NotCharging,

    /// Battery is full.
    Full,
}

impl fmt::Display for ChargeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match &self {
            ChargeState::Unknown => "unknown",
            ChargeState::Charging => "charging",
            ChargeState::Discharging => "discharging",
            ChargeState::NotCharging => "not_charging",
            ChargeState::Full => "discharging",
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
            "Full" => Ok(Self::Full),
            _ => Err(ParseError::ValueError(format!(
                "failed to parse \"{value}\", expected either \"Charging\", \"Discharging\", or \"Full\""
            ))),
        }
    }
}

#[derive(Debug)]
struct BatteryStatus {
    moment: SystemTime,
    charge_state: ChargeState,
    voltage: f32,
    energy: f32,
    consumption: f32,
    capacity: usize,
}

impl BatteryStatus {
    /// Return a CSV formatted string.
    pub fn to_csv(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}, {}\n",
            self.moment.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            self.charge_state,
            self.voltage,
            self.energy,
            self.consumption,
            self.capacity
        )
    }
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SoC: {}%, {}Wh, {}V, {}W",
            self.capacity, self.energy, self.voltage, self.consumption
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
        let consumption: f32 = extract_value_for("POWER_SUPPLY_POWER_NOW", &value)?
            .parse::<f32>()
            .map_err(|_| ParseError::ValueError(format!("failed to parse \"{value}\" as f32")))?
            / 1_000_000.0;

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
            moment: SystemTime::now(),
            consumption,
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

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("battery.csv")?;
    file.write_all(status.to_csv().as_bytes())?;

    Ok(())
}
