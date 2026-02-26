use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub initial: CpuState,
    #[serde(rename = "final")]
    pub final_state: CpuState,
    pub cycles: Vec<BusCycle>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CpuState {
    pub pc: u16,

    pub s: u16,

    pub a: u16,

    pub x: u16,

    pub y: u16,

    pub p: u8,

    #[serde(rename = "dbr")]
    pub db: u8,

    pub d: u16,

    #[serde(rename = "pbr")]
    pub pb: u8,

    pub e: u8,

    pub ram: Vec<(u32, u8)>,
}

#[derive(Debug, PartialEq)]
pub struct BusCycle {
    /// None for internal cycles (no bus access)
    pub address: Option<u32>,

    /// Value read or written (None for internal cycles)
    pub value: Option<u8>,

    pub operation: String,
}

impl<'de> Deserialize<'de> for BusCycle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple: (Option<u32>, Option<u8>, String) = Deserialize::deserialize(deserializer)?;
        Ok(BusCycle {
            address: tuple.0,
            value: tuple.1,
            operation: tuple.2,
        })
    }
}

impl CpuState {
    pub fn is_emulation_mode(&self) -> bool {
        self.e != 0
    }
}
