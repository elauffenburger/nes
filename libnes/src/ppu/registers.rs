pub struct PpuCtrlRegister {
    pub nametable_addr: u16,
    pub vram_addr_incr_type: bool,
    pub sprite_pattern_table_addr: u16,
    pub bg_pattern_table_addr: u16,
    pub sprite_size_type: bool,
    pub ppu_master_slave_select: bool,
    pub gen_nmi: bool,
}