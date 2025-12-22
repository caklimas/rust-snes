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
    pub address: u32,

    pub value: u8,

    pub operation: String,
}

impl<'de> Deserialize<'de> for BusCycle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple: (u32, u8, String) = Deserialize::deserialize(deserializer)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_test_case() {
        let json = r#"
        {
          "name": "ad 00 20",
          "initial": {
            "pc": 4096,
            "s": 253,
            "a": 0,
            "x": 0,
            "y": 0,
            "p": 48,
            "dbr": 0,
            "d": 0,
            "pbr": 0,
            "e": 0,
            "ram": [
              [4096, 173],
              [4097, 0],
              [4098, 32],
              [8192, 66]
            ]
          },
          "final": {
            "pc": 4099,
            "s": 253,
            "a": 66,
            "x": 0,
            "y": 0,
            "p": 48,
            "dbr": 0,
            "d": 0,
            "pbr": 0,
            "e": 0,
            "ram": [
              [4096, 173],
              [4097, 0],
              [4098, 32],
              [8192, 66]
            ]
          },
          "cycles": [
            [4096, 173, "read"],
            [4097, 0, "read"],
            [4098, 32, "read"],
            [8192, 66, "read"]
          ]
        }
        "#;

        let test: TestCase = serde_json::from_str(json).unwrap();

        assert_eq!(test.name, "ad 00 20");
        assert_eq!(test.initial.pc, 4096);
        assert_eq!(test.initial.a, 0);
        assert_eq!(test.initial.db, 0); // dbr in JSON -> db in struct
        assert_eq!(test.initial.pb, 0); // pbr in JSON -> pb in struct
        assert_eq!(test.initial.e, 0);
        assert_eq!(test.initial.ram.len(), 4);
        assert_eq!(test.initial.ram[0], (4096, 173));

        assert_eq!(test.final_state.pc, 4099);
        assert_eq!(test.final_state.a, 66);

        assert_eq!(test.cycles.len(), 4);
        assert_eq!(test.cycles[0].address, 4096);
        assert_eq!(test.cycles[0].value, 173);
        assert_eq!(test.cycles[0].operation, "read");
    }

    #[test]
    fn test_is_emulation_mode() {
        let native_state = CpuState {
            pc: 0,
            s: 0,
            a: 0,
            x: 0,
            y: 0,
            p: 0,
            db: 0,
            d: 0,
            pb: 0,
            e: 0,
            ram: vec![],
        };
        assert!(!native_state.is_emulation_mode());

        let emulation_state = CpuState {
            pc: 0,
            s: 0,
            a: 0,
            x: 0,
            y: 0,
            p: 0,
            db: 0,
            d: 0,
            pb: 0,
            e: 1,
            ram: vec![],
        };
        assert!(emulation_state.is_emulation_mode());
    }
}
