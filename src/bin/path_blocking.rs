use advent_of_code::path_blocking_analyzer::PathBlockingAnalyzer;

fn main() {
    let analyzer = PathBlockingAnalyzer::new();
    let blocking = analyzer.get_blocking_configurations();

    println!("Found {} configurations that block paths\n", blocking.len());

    // Show first 10 examples
    println!("First 10 blocking configurations:");
    for (i, &config) in blocking.iter().take(10).enumerate() {
        println!("Example {} (config = {}):", i + 1, config);
        analyzer.print_configuration(config);
    }

    // Example: Check a specific configuration
    // Config 0b01000001 = positions 0 and 6 accessible (top-left and bottom-left)
    let test_config = 0b01000001;
    println!("Testing specific configuration {}:", test_config);
    analyzer.print_configuration(test_config);

    if blocking.contains(&test_config) {
        println!("✓ This configuration blocks a path (top-left to bottom-left)");
    } else {
        println!("✗ This configuration does NOT block any path");
    }
}
