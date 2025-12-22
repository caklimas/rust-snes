use super::test_bus::TestBus;
use super::test_format::CpuState;
use rust_snes::cpu::{Cpu, processor_status::ProcessorStatus};

pub fn setup_cpu_from_state(cpu: &mut Cpu, state: &CpuState) {
    cpu.registers.pc = state.pc;
    cpu.registers.s = state.s;
    cpu.registers.a = state.a;
    cpu.registers.x = state.x;
    cpu.registers.y = state.y;
    cpu.registers.p = ProcessorStatus::from_bits_truncate(state.p);
    cpu.registers.db = state.db;
    cpu.registers.d = state.d;
    cpu.registers.pb = state.pb;
    cpu.emulation_mode = state.is_emulation_mode();
    cpu.waiting_for_interrupt = false;
    cpu.stopped = false;
}

pub fn extract_cpu_state(cpu: &Cpu) -> CpuState {
    CpuState {
        pc: cpu.registers.pc,
        s: cpu.registers.s,
        a: cpu.registers.a,
        x: cpu.registers.x,
        y: cpu.registers.y,
        p: cpu.registers.p.bits(),
        db: cpu.registers.db,
        d: cpu.registers.d,
        pb: cpu.registers.pb,
        e: if cpu.emulation_mode { 1 } else { 0 },
        ram: vec![], // Memory is extracted separately
    }
}

/// Sets up a TestBus from test memory state
pub fn setup_bus_from_state(bus: &mut TestBus, state: &CpuState) {
    bus.clear();
    bus.load_memory(&state.ram);
}

/// Extracts memory state from TestBus for the addresses specified in expected state
pub fn extract_memory_state(bus: &TestBus, expected_state: &CpuState) -> Vec<(u32, u8)> {
    let addresses: Vec<u32> = expected_state.ram.iter().map(|(addr, _)| *addr).collect();
    bus.extract_memory(&addresses)
}
