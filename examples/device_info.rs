//! device_info — enumerate all Metal devices and print their capabilities
//! using the raw-ABI bindings (mirrors `MTL::CopyAllDevices()`).

use metal_cpp_rs::objc_abi::autoreleasepool;
use metal_cpp_rs::prelude::*;

fn main() {
    autoreleasepool(|| unsafe {
        let devices = MTLCopyAllDevices();
        if devices.is_nil() {
            eprintln!("MTLCopyAllDevices() returned nil — not running on Apple silicon?");
            return;
        }
        let count = devices.count();
        println!("{count} Metal device(s):");

        for i in 0..count {
            let dev = devices.objectAtIndex(i);
            println!("--- device[{i}] ---");
            println!("  name:                       {}", dev.name().to_string());
            println!("  registryID:                 {}", dev.registryID());
            println!("  hasUnifiedMemory:           {}", dev.hasUnifiedMemory());
            println!("  isLowPower:                 {}", dev.isLowPower());
            println!("  isHeadless:                 {}", dev.isHeadless());
            println!("  isRemovable:                {}", dev.isRemovable());
            println!("  locationNumber:             {}", dev.locationNumber());
            println!("  maxBufferLength:            {}", dev.maxBufferLength());
            println!("  maxThreadgroupMemoryLength: {}", dev.maxThreadgroupMemoryLength());

            let max_threads = dev.maxThreadsPerThreadgroup();
            println!(
                "  maxThreadsPerThreadgroup:   {} x {} x {}",
                max_threads.width, max_threads.height, max_threads.depth
            );

            println!(
                "  recommendedMaxWorkingSet:   {}",
                dev.recommendedMaxWorkingSetSize()
            );
            println!("  currentAllocatedSize:       {}", dev.currentAllocatedSize());
            println!("  queryTimestampFrequency:    {}", dev.queryTimestampFrequency());

            let arch = dev.architecture();
            if !arch.is_nil() {
                println!("  architecture:               {}", arch.name().to_string());
            }

            // Capability probes using MTL::GPUFamily (values verbatim from metal-cpp).
            for (label, family) in [
                ("Apple7", MTLGPUFamily::GPUFamilyApple7),
                ("Apple8", MTLGPUFamily::GPUFamilyApple8),
                ("Apple9", MTLGPUFamily::GPUFamilyApple9),
                ("Mac1", MTLGPUFamily::GPUFamilyMac1),
                ("Mac2", MTLGPUFamily::GPUFamilyMac2),
                ("Metal3", MTLGPUFamily::GPUFamilyMetal3),
                ("Metal4", MTLGPUFamily::GPUFamilyMetal4),
            ] {
                if dev.supportsFamily(family) {
                    println!("  supportsFamily:             {label}");
                }
            }

            println!("  supportsRaytracing:         {}", dev.supportsRaytracing());
            println!("  supports32BitMSAA:          {}", dev.supports32BitMSAA());
            println!("  supportsBCTextureCompression: {}", dev.supportsBCTextureCompression());

            // Exercise a tiny allocation round trip on the first device.
            if i == 0 {
                let queue = dev.newCommandQueue();
                println!("  newCommandQueue:            {}", !queue.is_nil());
                let buffer = dev.newBuffer(256, MTLResourceOptions::ResourceStorageModeShared);
                if !buffer.is_nil() {
                    println!(
                        "  newBuffer(256):             length={} gpuAddress=0x{:x}",
                        buffer.length(),
                        buffer.gpuAddress()
                    );
                }
            }
        }
    });
}
