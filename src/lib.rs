use raw_cpuid::CpuId;
use std::collections::BTreeMap;

pub struct CacheInfo {
    pub size_kb: usize,
    pub line_size: usize,
    pub associativity: usize,
    pub partitions: usize,
}

pub fn get_cache_info() -> BTreeMap<String, CacheInfo> {
    let cpuid = CpuId::new();
    let mut caches = BTreeMap::new();

    if let Some(cache_params) = cpuid.get_cache_parameters() {
        for cache in cache_params {
            let associativity = cache.associativity() as usize;
            let partitions = cache.physical_line_partitions() as usize;
            let line_size = cache.coherency_line_size() as usize;
            let sets = cache.sets() as usize;

            let size_bytes = associativity * partitions * line_size * sets;
            let size_kb = size_bytes / 1024;

            let level = cache.level();
            let key = format!("L{}", level);

            caches.insert(
                key,
                CacheInfo {
                    size_kb,
                    line_size,
                    associativity,
                    partitions,
                },
            );
        }
    }

    caches
}