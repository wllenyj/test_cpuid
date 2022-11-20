use kvm_ioctls::{Cap, Kvm, VmFd};

fn main() {
    let kvm = Kvm::new().unwrap();
    let supported_cpuid = kvm.get_supported_cpuid(80).unwrap();

    for c in supported_cpuid.as_slice() {
        println!(
            "function: 0x{:x}, index: {}, flags: {}",
            c.function, c.index, c.flags
        );
        println!(
            "kvm support cpuid: eax: {:x}, ebx: {:x}, ecx: {:x}, edx: {:x}",
            c.eax, c.ebx, c.ecx, c.edx
        );
        let leaf = unsafe { std::arch::x86_64::__cpuid_count(c.function, c.index) };
        println!(
            "       host cpuid: eax: {:x}, ebx: {:x}, ecx: {:x}, edx: {:x}",
            leaf.eax, leaf.ebx, leaf.ecx, leaf.edx
        );
        println!();
    }

    //let max = unsafe { std::arch::x86_64::__get_cpuid_max(0) };
    //println!("max: {:?}, {:x}", max.0, max.1);

    //let ex_func = unsafe { std::arch::x86_64::__cpuid(0x8000_0000) };
    //println!("exterend functions: {:x}", ex_func.eax);

    //let l1 = unsafe { std::arch::x86_64::__cpuid(0x8000_0005) };
    //println!("cpuid: l1: {:?}", l1);

    //let l2 = unsafe { std::arch::x86_64::__cpuid(0x8000_0006) };
    //println!("cpuid: l2: {:?}", l2.ecx / 1024 / 1024);

    //let hc = get_host_cpu_cache_info(2);
    //println!("host cpuid: l2: {:?}", hc);

    //let hl2 = get_host_cpu_cache_info(0x8000_0006);
    //println!("host cpuid: l2: {:?}", hl2.ecx / 1024 / 1024);
}

// get host cache info
fn get_host_cpu_cache_info(func: u32) -> std::arch::x86_64::CpuidResult {
    let leaf = unsafe { std::arch::x86_64::__cpuid(func & 0xffff_fff0) };
    if func > leaf.eax {
        return std::arch::x86_64::CpuidResult {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
        };
    }
    return unsafe { std::arch::x86_64::__cpuid(func) };
}
