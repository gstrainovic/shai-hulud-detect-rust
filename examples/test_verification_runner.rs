use shai_hulud_scanner::e2e_tests::E2ETestRunner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running E2E Test Verification...\n");
    
    // Create E2E test runner
    let runner = E2ETestRunner::new(
        "test_verification_detailed.json", 
        "../shai-hulud-detect"
    ).await?;
    
    // Run all tests
    let results = runner.run_all_tests().await?;
    
    // Print detailed summary
    runner.print_test_summary(&results);
    
    // Show additional statistics
    let total_tests = results.len();
    let passed_tests = results.iter().filter(|r| r.passed).count();
    let failed_tests = total_tests - passed_tests;
    
    println!("\n==============================================");
    println!("📊 DETAILED STATISTICS:");
    println!("==============================================");
    println!("✅ Passed: {}", passed_tests);
    println!("❌ Failed: {}", failed_tests);
    println!("📋 Total:  {}", total_tests);
    println!("🎯 Success Rate: {:.1}%", (passed_tests as f64 / total_tests as f64) * 100.0);
    
    if failed_tests > 0 {
        println!("\n🔍 FAILED TEST DETAILS:");
        for result in results.iter().filter(|r| !r.passed) {
            println!("\n❌ {}:", result.test_case_name);
            for issue in &result.issues {
                println!("   • {}", issue);
            }
            if !result.missing_patterns.is_empty() {
                println!("   Missing patterns:");
                for pattern in &result.missing_patterns {
                    println!("     - {}", pattern);
                }
            }
        }
    }
    
    println!("\n==============================================");
    if failed_tests == 0 {
        println!("🎉 ALL TESTS PASSED! Test verification is complete.");
    } else {
        println!("⚠️  {} tests need attention. See details above.", failed_tests);
    }
    
    Ok(())
}