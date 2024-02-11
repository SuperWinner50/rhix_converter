#![allow(non_snake_case)]
use chrono::TimeZone;
use clap::Parser;
use std::io::Read;

macro_rules! readle {
    ($reader:expr, $ty:ty) => {{
        let mut buf = [0u8; std::mem::size_of::<$ty>()];
        $reader.read_exact(&mut buf).unwrap();
        let x = <$ty>::from_le_bytes(buf);
        x
    }};

    ($reader:expr, $ty:ty, $length:expr) => {{
        let mut buf = vec![0u8; std::mem::size_of::<$ty>() * $length];
        $reader.read_exact(&mut buf).unwrap();
        buf.chunks_exact(std::mem::size_of::<$ty>())
            .map(|v| <$ty>::from_le_bytes(v.try_into().unwrap()))
            .collect::<Vec<$ty>>()
    }};
}

fn read_data(v: u16, data_type: &str) -> f64 {
    match data_type {
        "R" | "REF" | "VEL" | "ZDR" | "KDP" => (v as f64 - 32768.0) / 100.0,
        "PHI" => 360.0 * (v as f64 - 32768.0) / 65535.0,
        "RHO" => 2.0 * (v as f64 - 1.0) / 65534.0,
        "SW" => (v as f64 - 1.0) / 100.0,
        d => panic!("Unknown datatype {}", d),
    }
}


// Docs: https://www.manualslib.com/manual/1935797/Furuno-Wr2120.html?page=72#manual
fn read_file(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref();
    let mut data = &*{
        let bytes = std::fs::read(path).unwrap();
        match path.extension().map(|ex| ex.to_str().unwrap()) {
            Some("gz") => {
                let mut buf = Vec::new();
                flate2::read::GzDecoder::new(&*bytes)
                    .read_to_end(&mut buf)
                    .unwrap();
                buf
            }
            Some("rhix") => bytes,
            _ => panic!("Unknown file type"),
        }
    };

    assert!(
        readle!(data, u16) == 156,
        "Header size is not 156, may have wrong format."
    );
    let _version = readle!(data, u16);
    let start_time = (
        readle!(data, u16),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
    );

    readle!(data, u8);

    let _end_time = (
        readle!(data, u16),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
    );

    readle!(data, u8);

    let _timezone = readle!(data, i16);
    let _productnumber = readle!(data, u16);
    let _modeltype = readle!(data, u16);
    let lat = readle!(data, i32) as f32 / 100000.0;
    let lon = readle!(data, i32) as f32 / 100000.0;
    let _alt = readle!(data, i32);
    let _azi_offset = readle!(data, u16);
    let _tx_freq = readle!(data, u32);
    let _polarization = readle!(data, u16);
    let _gain_h = readle!(data, u16);
    let _gain_v = readle!(data, u16);
    let _half_width_h = readle!(data, u16);
    let _half_width_v = readle!(data, u16);
    let _tx_power_h = readle!(data, u16);
    let _tx_power_v = readle!(data, u16);
    let _radar_const_h = readle!(data, i16);
    let _radar_const_v = readle!(data, i16);
    let _noise_power_h_short = readle!(data, i16);
    let _noise_power_h_long = readle!(data, i16);
    let _thresh_power_short = readle!(data, i16);
    let _thresh_power_long = readle!(data, i16);
    let _tx_pulse_spec = readle!(data, u16);
    let _prf_mode = readle!(data, u16);
    let _prf1 = readle!(data, u16);
    let _prf2 = readle!(data, u16);
    let _prf3 = readle!(data, u16);
    let nyquist = readle!(data, u16) as f32 / 10.0;
    let _sample_num = readle!(data, u16);
    let _tx_pulse_blind_len = readle!(data, u16);
    let _short_pulse_width = readle!(data, u16);
    let _short_pulse_mod_bandwith = readle!(data, u16);
    let _long_pulse_width = readle!(data, u16);
    let _long_pulse_mod_bandwidth = readle!(data, u16);
    let _pulse_switchpoint = readle!(data, u16);
    let _observation_mode = readle!(data, u16);
    let _rotation_speed = readle!(data, u16) as f32 / 10.0 / 60.0 * 360.0;
    let _rays = readle!(data, u16);
    let gates = readle!(data, u16);
    let gate_res = readle!(data, u16);
    let _scan_num = readle!(data, u16);
    let _total_scans = readle!(data, u16);
    let _rain_intensity_est = readle!(data, u16);
    let _zr_coeff_b = readle!(data, u16);
    let _zr_coeff_beta = readle!(data, u16);
    let _kdp_coeff_a = readle!(data, u16);
    let _kdp_coeff_b = readle!(data, u16);
    let _kdp_coeff_c = readle!(data, u16);
    let _zh_corr = readle!(data, u16);
    let _zh_corr_b1 = readle!(data, u16);
    let _zh_corr_b2 = readle!(data, u16);
    let _zh_corr_d1 = readle!(data, u16);
    let _zh_corr_d2 = readle!(data, u16);
    let _air_attenuation = readle!(data, u16);
    let _rain_thresh = readle!(data, u16);
    let record_item = readle!(data, u16);
    let (use_r, use_dbz, use_vel, use_zdr, use_kdp, use_phi, use_rho, use_w, use_quality) = (
        record_item & 1,
        record_item >> 1 & 1,
        record_item >> 2 & 1,
        record_item >> 3 & 1,
        record_item >> 4 & 1,
        record_item >> 5 & 1,
        record_item >> 6 & 1,
        record_item >> 7 & 1,
        record_item >> 8 & 1,
    );
    let _signal_flag = readle!(data, u16);
    let _clutter_ref_file = (
        readle!(data, u16),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
        readle!(data, u8),
    );
    readle!(data, u8);
    readle!(data, u64);

    let mut radar = silv::RadarFile {
        name: "FWLX".into(),
        sweeps: Vec::new(),
        params: std::collections::HashMap::new(),
    };

    let all_data_types = [
        (use_r, "R"),
        (use_dbz, "REF"),
        (use_vel, "VEL"),
        (use_zdr, "ZDR"),
        (use_kdp, "KDP"),
        (use_phi, "PHI"),
        (use_rho, "RHO"),
        (use_w, "SW"),
        (use_quality, "")
    ];

    for (data_type, name) in all_data_types {
        if data_type != 0 && name != "" {
            radar.params.insert(
                name.into(),
                silv::ParamDescription {
                    description: String::new(),
                    units: String::new(),
                    meters_to_first_cell: 0.0,
                    meters_between_cells: gate_res as f32,
                },
            );
        }
    }

    let mut sweep = silv::Sweep {
        latitude: lat,
        longitude: lon,
        elevation: 0.0,
        nyquist_velocity: nyquist,
        ..Default::default()
    };

    while !data.is_empty() {
        let size = readle!(data, u16);
        if size != 6 {
            panic!("Angle information block size error, found {size}");
        }

        let _azimuth = readle!(data, u16) as f32 / 100.0;
        let elevation = readle!(data, u16) as f32 / 100.0;

        let mut ray = silv::Ray {
            azimuth: -elevation + 90.0,
            time: chrono::Utc.with_ymd_and_hms(
                start_time.0 as i32,
                start_time.1 as u32,
                start_time.2 as u32,
                start_time.3 as u32,
                start_time.4 as u32,
                start_time.5 as u32,
            ).latest().unwrap(),
            data: std::collections::HashMap::default(),
        };

        let observed_block_size = readle!(data, u16);

        assert!(
            (observed_block_size - 2)
                / (use_r + use_dbz + use_vel + use_zdr + use_kdp + use_phi + use_rho + use_w + use_quality)
                / 2
                == gates,
            "Observed block error",
        );

        for (data_type, name) in all_data_types {
            if data_type != 0 {
                let data = readle!(data, u16, gates as usize)
                    .into_iter()
                    .map(|v| read_data(v, name))
                    .collect();

                if name != "" {
                    ray.data.insert(name.into(), data);
                }
            }
        }

        sweep.rays.push(ray);
    }

    radar.sweeps.push(sweep);

    silv::write(radar, ".", &silv::RadyOptions::default());
}

#[derive(Parser)]
struct Args {
    /// Path(s) of file to convert. For a folder, use a * symbol at the end.
    #[clap(short, long, value_parser)]
    files: String,
}

fn main() {
    let args = Args::parse();

    for file in glob::glob(&args.files).unwrap() {
        read_file(file.unwrap())
    }
}
