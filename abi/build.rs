// use std::process::Command;

fn main() {
    tonic_build::configure()
        // .build_server(false)
        .type_attribute("reservation.ReservationStatus", "#[derive(sqlx::Type)]")
        .type_attribute(
            "reservation.ReservationQuery",
            "#[derive(derive_builder::Builder)]",
        )
        .field_attribute(
            "reservation.ReservationQuery.start",
            "#[builder(setter(into, strip_option))]",
        )
        .out_dir("src/pb")
        .compile_protos(&["protos/reservation.proto"], &["protos"])
        .unwrap();
    println!("cargo:rerun-if-changed=protos/reservation.proto");

    // Command::new("cargo").args(&["fmt"]).output().unwrap();
}

// trait BuilderExt {
//     fn with_sql_type(self, paths: &[&str]) -> Self;
//     fn with_builder(self, paths: &[&str]) -> Self;
//     fn with_builder_into(self, path: &str, fields: &[&str]) -> Self;
//     fn with_builder_option(self, path: &str, fields: &[&str]) -> Self;
// }

// impl BuilderExt for tonic_build::Builder {
//     fn with_sql_type(self, paths: &[&str]) -> Self {
//         paths.iter().fold(self, |builder, path| {
//             builder.type_attribute(path, "#[derive(sqlx::Type)]")
//         })
//     }

//     fn with_builder(self, paths: &[&str]) -> Self {
//         paths.iter().fold(self, |builder, path| {
//             builder.type_attribute(path, "#[derive(derive_builder::Builder)]")
//         })
//     }

//     fn with_builder_into(self, path: &str, fields: &[&str]) -> Self {
//         fields.iter().fold(self, |builder, field| {
//             builder.field_attribute(
//                 &format!("{}.{}", path, field),
//                 format!("#[builder(setter(into))]"),
//             )
//         })
//     }

//     fn with_builder_option(self, path: &str, fields: &[&str]) -> Self {
//         fields.iter().fold(self, |builder, field| {
//             builder.field_attribute(
//                 &format!("{}.{}", path, field),
//                 format!("#[builder(setter(into,strip_option))]"),
//             )
//         })
//     }
// }
