fn print_report(report: &String) {
    println!("Report: {}", report);
}

fn append_signature(report: &mut String) {
    report.push_str("\nSigned by: Misty Waters");
}

fn transfer_ownership(report: String) {
    println!("Ownership moved here: {}", report);
}

fn create_summary() -> String {
    let summary = String::from("Summary: Rust ownership protects data safety.");
    summary
}

// This would fail because it returns a reference to local data:
// fn bad_reference() -> &str {
//     let message = String::from("This will not work");
//     &message
// }

fn main() {
    let mut report = String::from("AI Safety + Rust Homework Report");

    print_report(&report);

    append_signature(&mut report);

    println!("\nAfter signature:");
    print_report(&report);

    let summary = create_summary();
    println!("\n{}", summary);

    transfer_ownership(report);

    // This would fail because report ownership was moved:
    // println!("{}", report);
}