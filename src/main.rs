use cpuid_cache_info::{get_cache_info};

fn main() {
    let caches = get_cache_info();

    for (level, info) in &caches {
        println!("{} Cache:", level);
        println!("  Size:          {} KB", info.size_kb);
        println!("  Line size:     {} bytes", info.line_size);
        println!("  Associativity: {}", info.associativity);
        println!("  Partitions:    {}", info.partitions);
        println!();
    }
    if let Some(l3) = caches.get("L1") {
        println!("L3 Cache:");
        println!("  Size:          {} KB", l3.size_kb);
        println!("  Line size:     {} bytes", l3.line_size);
        println!("  Associativity: {}", l3.associativity);
        println!("  Partitions:    {}", l3.partitions);
    } else {
        println!("L3 cache info not found.");
    }
}