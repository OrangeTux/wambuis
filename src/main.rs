use std::{fmt, fs};

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
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Charging" => Ok(Self::Charging),
            "Discharging" => Ok(Self::Discharging),
            _ => Err(()),
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
        write!(f, "SoC: {}%, {}Wh, {} V", self.capacity, self.energy, self.voltage)
    }
}

fn extract_value_for<'a>(needle: &'a str, haystack: &'a str) -> &'a str {
    let line = haystack.lines().find(|line| line.contains(needle)).unwrap();
    let elements: Vec<&str> = line.split('=').collect();
    elements[1]
}

impl TryFrom<String> for BatteryStatus {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let capacity: usize = extract_value_for("POWER_SUPPLY_CAPACITY", &value)
            .parse()
            .unwrap();

        let voltage: f32 = extract_value_for("POWER_SUPPLY_VOLTAGE_NOW", &value)
            .parse::<f32>()
            .unwrap()
            / 1_000_000.0;

        let energy: f32 = extract_value_for("POWER_SUPPLY_ENERGY_NOW", &value)
            .parse::<f32>()
            .unwrap()
            / 1_000_000.0;

        let charge_state: ChargeState = extract_value_for("POWER_SUPPLY_STATUS", &value)
            .try_into()
            .unwrap();
        Ok(BatteryStatus {
            capacity,
            voltage,
            energy,
            charge_state,
        })
    }
}

fn main() {
    let content = fs::read_to_string("/sys/class/power_supply/BAT0/uevent").unwrap();
    let status: BatteryStatus = content.try_into().unwrap();

    println!("Hello, world! {}", status);
}
