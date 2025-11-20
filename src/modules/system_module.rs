use colored::Colorize;
use sysinfo::{Disks, Networks, System};

/// Display system information
pub fn display_system_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "System Information".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    // OS Information
    println!("\n{}", "Operating System:".yellow().bold());
    println!(
        "  Name:         {}",
        System::name()
            .unwrap_or_else(|| "Unknown".to_string())
            .green()
    );
    println!(
        "  Kernel:       {}",
        System::kernel_version()
            .unwrap_or_else(|| "Unknown".to_string())
            .green()
    );
    println!(
        "  OS Version:   {}",
        System::os_version()
            .unwrap_or_else(|| "Unknown".to_string())
            .green()
    );
    println!(
        "  Host Name:    {}",
        System::host_name()
            .unwrap_or_else(|| "Unknown".to_string())
            .green()
    );

    // CPU Information
    println!("\n{}", "CPU:".yellow().bold());
    println!("  CPUs:         {}", sys.cpus().len().to_string().green());
    if let Some(cpu) = sys.cpus().first() {
        println!("  Brand:        {}", cpu.brand().green());
        println!(
            "  Frequency:    {} MHz",
            cpu.frequency().to_string().green()
        );
    }

    // Memory Information
    println!("\n{}", "Memory:".yellow().bold());
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let free_mem = sys.available_memory();

    println!("  Total:        {} GB", format_bytes(total_mem).green());
    println!("  Used:         {} GB", format_bytes(used_mem).green());
    println!("  Available:    {} GB", format_bytes(free_mem).green());
    println!(
        "  Usage:        {}%",
        ((used_mem as f64 / total_mem as f64) * 100.0)
            .round()
            .to_string()
            .yellow()
    );
}

/// Display CPU information
pub fn display_cpu_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "CPU Information".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("\n{} {}", "CPU".yellow().bold(), i.to_string().green());
        println!("  Brand:        {}", cpu.brand().green());
        println!(
            "  Frequency:    {} MHz",
            cpu.frequency().to_string().green()
        );
        println!(
            "  Usage:        {}%",
            cpu.cpu_usage().round().to_string().yellow()
        );
    }
}

/// Display memory information
pub fn display_memory_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "Memory Information".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let free_mem = sys.available_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    println!("\n{}", "RAM:".yellow().bold());
    println!("  Total:        {} GB", format_bytes(total_mem).green());
    println!("  Used:         {} GB", format_bytes(used_mem).green());
    println!("  Available:    {} GB", format_bytes(free_mem).green());
    println!(
        "  Usage:        {}%",
        ((used_mem as f64 / total_mem as f64) * 100.0)
            .round()
            .to_string()
            .yellow()
    );

    println!("\n{}", "Swap:".yellow().bold());
    println!("  Total:        {} GB", format_bytes(total_swap).green());
    println!("  Used:         {} GB", format_bytes(used_swap).green());
    if total_swap > 0 {
        println!(
            "  Usage:        {}%",
            ((used_swap as f64 / total_swap as f64) * 100.0)
                .round()
                .to_string()
                .yellow()
        );
    }
}

/// Display disk information
pub fn display_disk_info() {
    let disks = Disks::new_with_refreshed_list();

    println!("{}", "Disk Information".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    for disk in &disks {
        println!(
            "\n{} {}",
            "Disk:".yellow().bold(),
            disk.name().to_string_lossy().green()
        );
        println!(
            "  Mount Point:  {}",
            disk.mount_point().display().to_string().green()
        );
        println!(
            "  File System:  {}",
            disk.file_system().to_string_lossy().green()
        );
        println!(
            "  Total:        {} GB",
            format_bytes(disk.total_space()).green()
        );
        println!(
            "  Available:    {} GB",
            format_bytes(disk.available_space()).green()
        );
        println!(
            "  Used:         {} GB",
            format_bytes(disk.total_space() - disk.available_space()).green()
        );

        let usage_percent = if disk.total_space() > 0 {
            ((disk.total_space() - disk.available_space()) as f64 / disk.total_space() as f64)
                * 100.0
        } else {
            0.0
        };
        println!(
            "  Usage:        {}%",
            usage_percent.round().to_string().yellow()
        );
    }
}

/// Display network interfaces
pub fn display_network_info() {
    let networks = Networks::new_with_refreshed_list();

    println!("{}", "Network Interfaces".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    for (interface_name, network) in &networks {
        println!(
            "\n{} {}",
            "Interface:".yellow().bold(),
            interface_name.green()
        );
        println!(
            "  Received:     {} MB",
            format_bytes(network.total_received()).green()
        );
        println!(
            "  Transmitted:  {} MB",
            format_bytes(network.total_transmitted()).green()
        );
        println!(
            "  Packets Rx:   {}",
            network.total_packets_received().to_string().green()
        );
        println!(
            "  Packets Tx:   {}",
            network.total_packets_transmitted().to_string().green()
        );
    }
}

/// Format bytes to human-readable format
fn format_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / 1_073_741_824.0;
    format!("{:.2}", gb)
}
