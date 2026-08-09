use crate::apu::spc700::Spc700;

pub fn mov_x_imm(spc700: &mut Spc700) {
    let value = spc700.read_byte();
    spc700.registers.x = value;

    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_sp_x(spc700: &mut Spc700) {
    spc700.registers.sp = spc700.registers.x;
}

pub fn mov_a_imm(spc700: &mut Spc700) {
    let value = spc700.read_byte();
    spc700.registers.a = value;

    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_ind_x_a(spc700: &mut Spc700) {
    spc700.write(
        spc700.get_direct_page_address(spc700.registers.x as u32),
        spc700.registers.a,
    );
}

pub fn mov_a_y(spc700: &mut Spc700) {
    spc700.registers.a = spc700.registers.y;

    spc700.set_n(spc700.registers.y);
    spc700.set_z(spc700.registers.y);
}

pub fn mov_x_a(spc700: &mut Spc700) {
    spc700.registers.x = spc700.registers.a;

    spc700.set_n(spc700.registers.a);
    spc700.set_z(spc700.registers.a);
}

pub fn mov_dp_imm(spc700: &mut Spc700) {
    let immediate = spc700.read_byte();
    let offset = spc700.read_byte() as u32;

    spc700.write(spc700.get_direct_page_address(offset), immediate);
}

pub fn mov_y_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_value = spc700.read(spc700.get_direct_page_address(offset));

    spc700.registers.y = dp_value;
    spc700.set_n(dp_value);
    spc700.set_z(dp_value);
}

pub fn mov_a_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_value = spc700.read(spc700.get_direct_page_address(offset));

    spc700.registers.a = dp_value;
    spc700.set_n(dp_value);
    spc700.set_z(dp_value);
}

pub fn mov_dp_y(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;

    spc700.write(spc700.get_direct_page_address(offset), spc700.registers.y);
}

pub fn mov_dp_a(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;

    spc700.write(spc700.get_direct_page_address(offset), spc700.registers.a);
}

pub fn mov_dp_y_a(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let pointer = spc700.read_word_direct(spc700.get_direct_page_address(offset)) as u32;
    let address = pointer.wrapping_add(spc700.registers.y as u32);

    spc700.write(address, spc700.registers.a);
}

pub fn movw_ya_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);

    spc700.registers.a = spc700.read(dp_address);
    spc700.registers.y = spc700.read(dp_address.wrapping_add(1));

    spc700
        .registers
        .psw
        .set_zero(spc700.registers.a == 0 && spc700.registers.y == 0);

    spc700
        .registers
        .psw
        .set_negative(spc700.registers.y & 0x80 != 0);
}

pub fn movw_dp_ya(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);

    spc700.write(dp_address, spc700.registers.a);
    spc700.write(dp_address.wrapping_add(1), spc700.registers.y);
}

pub fn mov_y_imm(spc700: &mut Spc700) {
    let value = spc700.read_byte();
    spc700.registers.y = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_x(spc700: &mut Spc700) {
    spc700.registers.a = spc700.registers.x;
    spc700.set_n(spc700.registers.x);
    spc700.set_z(spc700.registers.x);
}

pub fn mov_y_a(spc700: &mut Spc700) {
    spc700.registers.y = spc700.registers.a;
    spc700.set_n(spc700.registers.a);
    spc700.set_z(spc700.registers.a);
}

pub fn mov_x_sp(spc700: &mut Spc700) {
    spc700.registers.x = spc700.registers.sp;
    spc700.set_n(spc700.registers.sp);
    spc700.set_z(spc700.registers.sp);
}

pub fn mov_a_dp_x(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let value = spc700.read(spc700.get_direct_page_address((offset + x) & 0xFF));
    spc700.registers.a = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_abs(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    spc700.registers.a = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_abs_x(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address.wrapping_add(spc700.registers.x as u32));
    spc700.registers.a = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_abs_y(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address.wrapping_add(spc700.registers.y as u32));
    spc700.registers.a = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_ind_x(spc700: &mut Spc700) {
    let address = spc700.get_direct_page_address(spc700.registers.x as u32);
    let value = spc700.read(address);
    spc700.registers.a = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_ind_x_inc(spc700: &mut Spc700) {
    let address = spc700.get_direct_page_address(spc700.registers.x as u32);
    let value = spc700.read(address);
    spc700.registers.a = value;
    spc700.registers.x = spc700.registers.x.wrapping_add(1);
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_ind_dp_y(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let pointer = spc700.read_word_direct(spc700.get_direct_page_address(offset)) as u32;
    let value = spc700.read(pointer.wrapping_add(spc700.registers.y as u32));
    spc700.registers.a = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_a_ind_dp_x(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let pointer =
        spc700.read_word_direct(spc700.get_direct_page_address((offset + x) & 0xFF)) as u32;
    let value = spc700.read(pointer);
    spc700.registers.a = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_x_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let value = spc700.read(spc700.get_direct_page_address(offset));
    spc700.registers.x = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_x_dp_y(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let y = spc700.registers.y as u32;
    let value = spc700.read(spc700.get_direct_page_address((offset + y) & 0xFF));
    spc700.registers.x = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_x_abs(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    spc700.registers.x = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_y_dp_x(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let value = spc700.read(spc700.get_direct_page_address((offset + x) & 0xFF));
    spc700.registers.y = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_y_abs(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    spc700.registers.y = value;
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_dp_dp(spc700: &mut Spc700) {
    let src_offset = spc700.read_byte() as u32;
    let dst_offset = spc700.read_byte() as u32;
    let value = spc700.read(spc700.get_direct_page_address(src_offset));
    spc700.write(spc700.get_direct_page_address(dst_offset), value);
}

pub fn mov_dp_x(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    spc700.write(spc700.get_direct_page_address(offset), spc700.registers.x);
}

pub fn mov_dp_x_a(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    spc700.write(
        spc700.get_direct_page_address((offset + x) & 0xFF),
        spc700.registers.a,
    );
}

pub fn mov_dp_x_y(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    spc700.write(
        spc700.get_direct_page_address((offset + x) & 0xFF),
        spc700.registers.y,
    );
}

pub fn mov_dp_y_x(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let y = spc700.registers.y as u32;
    spc700.write(
        spc700.get_direct_page_address((offset + y) & 0xFF),
        spc700.registers.x,
    );
}

pub fn mov_abs_a(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    spc700.write(address, spc700.registers.a);
}

pub fn mov_abs_x(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    spc700.write(address, spc700.registers.x);
}

pub fn mov_abs_y(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    spc700.write(address, spc700.registers.y);
}

pub fn mov_abs_x_a(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    spc700.write(
        address.wrapping_add(spc700.registers.x as u32),
        spc700.registers.a,
    );
}

pub fn mov_abs_y_a(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    spc700.write(
        address.wrapping_add(spc700.registers.y as u32),
        spc700.registers.a,
    );
}

pub fn mov_ind_x_inc_a(spc700: &mut Spc700) {
    let address = spc700.get_direct_page_address(spc700.registers.x as u32);
    spc700.write(address, spc700.registers.a);
    spc700.registers.x = spc700.registers.x.wrapping_add(1);
}

pub fn mov_ind_dp_x_a(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let pointer =
        spc700.read_word_direct(spc700.get_direct_page_address((offset + x) & 0xFF)) as u32;
    spc700.write(pointer, spc700.registers.a);
}
