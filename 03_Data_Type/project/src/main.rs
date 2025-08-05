// BMI classification thresholds
const UNDERWEIGHT_THRESHOLD: f64 = 18.5;
const NORMAL_THRESHOLD: f64 = 25.0;
const OVERWEIGHT_THRESHOLD: f64 = 30.0;

fn main() {
    // Predefined patient using a tuple (name, weight in kg, height in meters)
    let patient: (&str, f64, f64) = ("Ada Lovelace", 68.0, 1.65);

    // Destructure the tuple to get the values for mathematical operations
    let (name, weight_kg, height_m) = patient;

    // Calculate BMI
    let bmi = weight_kg / (height_m * height_m);

    // Determine health status using constants for thresholds
    let health_status = if bmi < UNDERWEIGHT_THRESHOLD {
        "Underweight"
    } else if bmi < NORMAL_THRESHOLD {
        "Normal"
    } else if bmi < OVERWEIGHT_THRESHOLD {
        "Overweight"
    } else {
        "Obese"
    };

    // Debug log (optional)
    dbg!(bmi, health_status);

    // Final output using formatting
    println!("--------- Health Advisor Report ---------");
    println!("Name        : {name}");
    println!("Weight (kg) : {:.1}", weight_kg);
    println!("Height (m)  : {:.2}", height_m);
    println!("BMI         : {:.2}", bmi);
    println!("Status      : {health_status}");
}
