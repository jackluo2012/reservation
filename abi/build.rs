fn main() {
    tonic_build::configure()
        .build_server(false)
        .type_attribute("reservation.ReservationStatus", "#[derive(sqlx::Type)]")
        .out_dir("src/pb")
        .compile_protos(&["protos/reservation.proto"], &["protos"])
        .unwrap();
    println!("cargo:rerun-if-changed=protos/reservation.proto");

    // Command::new("cargo").args(&["fmt"]).output().unwrap();
}
