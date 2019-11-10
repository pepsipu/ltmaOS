use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::gdt::SegmentSelector;
use x86_64::instructions::{tables::load_tss, segmentation::set_cs};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct Gdt {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
    pub gdt: GlobalDescriptorTable
}

lazy_static! {
    static ref GDT: Gdt = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        Gdt { code_selector, tss_selector, gdt}
    };
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

pub fn init() {
    GDT.gdt.load();
    unsafe {
        set_cs(GDT.code_selector);
        load_tss(GDT.tss_selector);
    }
}